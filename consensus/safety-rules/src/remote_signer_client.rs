// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! gRPC client for remote safety rules/signing service.
//!
//! This module implements the [`TSafetyRules`] trait over a gRPC connection to a remote
//! signing service. The remote service holds the validator's private keys and performs
//! all safety rule validation and signing operations.
//!
//! # Architecture
//!
//! The `RemoteSignerClient` acts as a proxy that forwards all safety rules operations
//! to a remote `aptos-remote-signer` service:
//!
//! ```text
//! +-------------------+       gRPC/mTLS        +----------------------+
//! |  Validator Node   |  ------------------>   |  aptos-remote-signer |
//! |                   |  <------------------   |                      |
//! |  RemoteSignerClient                        |  SafetyRulesServiceImpl
//! |  (TSafetyRules)   |                        |  (SafetyRules)       |
//! +-------------------+                        +----------------------+
//! ```
//!
//! # Security
//!
//! - **Mutual TLS**: By default, requires mTLS for secure communication
//! - **Server-side validation**: All safety rule checks happen on the remote signer
//! - **Key isolation**: Private keys never leave the remote signer process
//!
//! # Usage
//!
//! The client is typically created via [`SafetyRulesManager`](crate::SafetyRulesManager):
//!
//! ```ignore
//! let config = RemoteSignerConfig {
//!     server_address: "https://signer.internal:8443".to_string(),
//!     tls_config: Some(RemoteSignerTlsConfig {
//!         ca_cert_path: "/etc/certs/ca.pem".into(),
//!         client_cert_path: "/etc/certs/client.pem".into(),
//!         client_key_path: "/etc/certs/client-key.pem".into(),
//!     }),
//!     allow_insecure: false,
//!     ..Default::default()
//! };
//! let client = RemoteSignerClient::new(config)?;
//! ```
//!
//! # Retry Logic
//!
//! The client implements automatic retry with exponential backoff for transient failures.
//! Configure via `max_retries` and `initial_backoff_ms` in [`RemoteSignerConfig`].

use crate::{ConsensusState, Error, TSafetyRules};
use aptos_config::config::RemoteSignerConfig;
use aptos_consensus_types::{
    block_data::BlockData,
    order_vote::OrderVote,
    order_vote_proposal::OrderVoteProposal,
    safety_data::SafetyData,
    timeout_2chain::{TwoChainTimeout, TwoChainTimeoutCertificate},
    vote::Vote,
    vote_proposal::VoteProposal,
};
use aptos_crypto::bls12381;
use aptos_logger::{info, warn};
use aptos_protos::safety_rules::v1::{
    self as proto, safety_rules_service_client::SafetyRulesServiceClient,
};
use aptos_types::{
    epoch_change::EpochChangeProof,
    ledger_info::{LedgerInfo, LedgerInfoWithSignatures},
};
use std::{sync::Arc, time::Duration};
use tokio::{runtime::Runtime, sync::Mutex};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint, Identity};

/// A client that implements [`TSafetyRules`] by making gRPC calls to a remote signing service.
///
/// This client delegates all consensus signing operations to a remote `aptos-remote-signer`
/// service. The remote service holds the validator's private keys and enforces all safety
/// rules, preventing signing of conflicting blocks even if the validator node is compromised.
///
/// # Thread Safety
///
/// `RemoteSignerClient` is thread-safe and can be cloned. All clones share the same
/// underlying gRPC connection and tokio runtime.
///
/// # Error Handling
///
/// Operations return [`Error`] on failure. The client automatically retries transient
/// failures (network errors, timeouts) with exponential backoff.
pub struct RemoteSignerClient {
    client: Arc<Mutex<SafetyRulesServiceClient<Channel>>>,
    runtime: Arc<Runtime>,
    config: RemoteSignerConfig,
}

impl RemoteSignerClient {
    /// Creates a new RemoteSignerClient that connects to the specified remote signer service.
    pub fn new(config: RemoteSignerConfig) -> Result<Self, Error> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .thread_name("remote-signer-client")
            .build()
            .map_err(|e| Error::InternalError(format!("Failed to create tokio runtime: {}", e)))?;

        let client = runtime.block_on(Self::create_client(&config))?;

        info!(
            "Created RemoteSignerClient connecting to {}",
            config.server_address
        );

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
            runtime: Arc::new(runtime),
            config,
        })
    }

    async fn create_client(
        config: &RemoteSignerConfig,
    ) -> Result<SafetyRulesServiceClient<Channel>, Error> {
        let mut endpoint = Endpoint::from_shared(config.server_address.clone())
            .map_err(|e| Error::InternalError(format!("Invalid server address: {}", e)))?
            .connect_timeout(config.connect_timeout())
            .timeout(config.request_timeout());

        // Configure TLS if provided
        if let Some(tls) = &config.tls_config {
            let ca_cert = tokio::fs::read(&tls.ca_cert_path).await.map_err(|e| {
                Error::InternalError(format!(
                    "Failed to read CA cert from {:?}: {}",
                    tls.ca_cert_path, e
                ))
            })?;
            let client_cert = tokio::fs::read(&tls.client_cert_path).await.map_err(|e| {
                Error::InternalError(format!(
                    "Failed to read client cert from {:?}: {}",
                    tls.client_cert_path, e
                ))
            })?;
            let client_key = tokio::fs::read(&tls.client_key_path).await.map_err(|e| {
                Error::InternalError(format!(
                    "Failed to read client key from {:?}: {}",
                    tls.client_key_path, e
                ))
            })?;

            let tls_config = ClientTlsConfig::new()
                .ca_certificate(Certificate::from_pem(ca_cert))
                .identity(Identity::from_pem(client_cert, client_key));

            endpoint = endpoint
                .tls_config(tls_config)
                .map_err(|e| Error::InternalError(format!("TLS config error: {}", e)))?;
        } else if !config.allow_insecure {
            // TLS is not configured and insecure mode is not allowed
            return Err(Error::InternalError(
                "TLS configuration is required. Set allow_insecure=true for testing without TLS."
                    .to_string(),
            ));
        } else {
            // Insecure mode - log a warning
            warn!(
                "Connecting to remote signer WITHOUT TLS - this is insecure and should only be used for testing!"
            );
        }

        let channel = endpoint.connect().await.map_err(|e| {
            Error::InternalError(format!("Failed to connect to remote signer: {}", e))
        })?;

        Ok(SafetyRulesServiceClient::new(channel))
    }

    /// Executes an async operation with retry logic.
    fn with_retry<F, T, Fut>(&self, operation: &str, f: F) -> Result<T, Error>
    where
        F: Fn(SafetyRulesServiceClient<Channel>) -> Fut,
        Fut: std::future::Future<Output = Result<tonic::Response<T>, tonic::Status>>,
    {
        let mut backoff_ms = self.config.initial_backoff_ms;
        let mut attempts = 0;

        loop {
            let client = self
                .runtime
                .block_on(async { self.client.lock().await.clone() });

            match self.runtime.block_on(f(client)) {
                Ok(response) => return Ok(response.into_inner()),
                Err(status) => {
                    attempts += 1;
                    if attempts >= self.config.max_retries {
                        return Err(Error::InternalError(format!(
                            "Remote signer {} failed after {} attempts: {}",
                            operation, attempts, status
                        )));
                    }
                    warn!(
                        "Remote signer {} attempt {} failed: {}, retrying in {}ms",
                        operation, attempts, status, backoff_ms
                    );
                    std::thread::sleep(Duration::from_millis(backoff_ms));
                    backoff_ms = (backoff_ms * 2).min(30000); // Cap at 30 seconds
                }
            }
        }
    }

    /// Converts a proto ErrorResponse to our Error type.
    fn proto_error_to_error(err: &proto::ErrorResponse) -> Error {
        // Map error types back to our Error enum based on prefix matching
        let error_type = err.error_type.as_str();
        let message = err.message.clone();

        if error_type.starts_with("NotInitialized") {
            Error::NotInitialized(message)
        } else if error_type.starts_with("SerializationError") {
            Error::SerializationError(message)
        } else if error_type.starts_with("IncorrectLastVotedRound") {
            Error::IncorrectLastVotedRound(0, 0)
        } else if error_type.starts_with("IncorrectPreferredRound") {
            Error::IncorrectPreferredRound(0, 0)
        } else if error_type.starts_with("InvalidQuorumCertificate") {
            Error::InvalidQuorumCertificate(message)
        } else if error_type.starts_with("InvalidProposal") {
            Error::InvalidProposal(message)
        } else if error_type.starts_with("VoteProposalSignatureNotFound") {
            Error::VoteProposalSignatureNotFound
        } else {
            Error::InternalError(format!("{}: {}", error_type, message))
        }
    }
}

impl TSafetyRules for RemoteSignerClient {
    fn consensus_state(&mut self) -> Result<ConsensusState, Error> {
        let response: proto::ConsensusStateResponse =
            self.with_retry("consensus_state", |mut client| async move {
                client
                    .consensus_state(proto::ConsensusStateRequest {})
                    .await
            })?;

        match response.result {
            Some(proto::consensus_state_response::Result::State(state)) => {
                let waypoint = bcs::from_bytes(&state.waypoint).map_err(|e| {
                    Error::SerializationError(format!("Failed to deserialize waypoint: {}", e))
                })?;

                // Construct SafetyData from the proto fields
                let safety_data = SafetyData::new(
                    state.epoch,
                    state.last_voted_round,
                    state.preferred_round,
                    state.preferred_round, // one_chain_round (use preferred_round as approximation)
                    None,
                    0, // highest_timeout_round (not tracked in proto)
                );

                Ok(ConsensusState::new(
                    safety_data,
                    waypoint,
                    state.in_validator_set,
                ))
            }
            Some(proto::consensus_state_response::Result::Error(err)) => {
                Err(Self::proto_error_to_error(&err))
            }
            None => Err(Error::InternalError(
                "Empty response from consensus_state".to_string(),
            )),
        }
    }

    fn initialize(&mut self, proof: &EpochChangeProof) -> Result<(), Error> {
        let epoch_change_proof = bcs::to_bytes(proof).map_err(|e| {
            Error::SerializationError(format!("Failed to serialize EpochChangeProof: {}", e))
        })?;

        let response: proto::InitializeResponse =
            self.with_retry("initialize", |mut client| {
                let epoch_change_proof = epoch_change_proof.clone();
                async move {
                    client
                        .initialize(proto::InitializeRequest { epoch_change_proof })
                        .await
                }
            })?;

        match response.result {
            Some(proto::initialize_response::Result::Success(_)) => Ok(()),
            Some(proto::initialize_response::Result::Error(err)) => {
                Err(Self::proto_error_to_error(&err))
            }
            None => Err(Error::InternalError(
                "Empty response from initialize".to_string(),
            )),
        }
    }

    fn sign_proposal(&mut self, block_data: &BlockData) -> Result<bls12381::Signature, Error> {
        let block_data_bytes = bcs::to_bytes(block_data).map_err(|e| {
            Error::SerializationError(format!("Failed to serialize BlockData: {}", e))
        })?;

        let response: proto::SignProposalResponse =
            self.with_retry("sign_proposal", |mut client| {
                let block_data = block_data_bytes.clone();
                async move {
                    client
                        .sign_proposal(proto::SignProposalRequest { block_data })
                        .await
                }
            })?;

        match response.result {
            Some(proto::sign_proposal_response::Result::Signature(sig_bytes)) => {
                bcs::from_bytes(&sig_bytes).map_err(|e| {
                    Error::SerializationError(format!("Failed to deserialize signature: {}", e))
                })
            }
            Some(proto::sign_proposal_response::Result::Error(err)) => {
                Err(Self::proto_error_to_error(&err))
            }
            None => Err(Error::InternalError(
                "Empty response from sign_proposal".to_string(),
            )),
        }
    }

    fn sign_timeout_with_qc(
        &mut self,
        timeout: &TwoChainTimeout,
        timeout_cert: Option<&TwoChainTimeoutCertificate>,
    ) -> Result<bls12381::Signature, Error> {
        let timeout_bytes = bcs::to_bytes(timeout).map_err(|e| {
            Error::SerializationError(format!("Failed to serialize TwoChainTimeout: {}", e))
        })?;

        let timeout_cert_bytes = timeout_cert
            .map(|tc| bcs::to_bytes(tc))
            .transpose()
            .map_err(|e| {
                Error::SerializationError(format!(
                    "Failed to serialize TwoChainTimeoutCertificate: {}",
                    e
                ))
            })?;

        let response: proto::SignTimeoutWithQcResponse =
            self.with_retry("sign_timeout_with_qc", |mut client| {
                let timeout = timeout_bytes.clone();
                let timeout_cert = timeout_cert_bytes.clone();
                async move {
                    client
                        .sign_timeout_with_qc(proto::SignTimeoutWithQcRequest {
                            timeout,
                            timeout_cert,
                        })
                        .await
                }
            })?;

        match response.result {
            Some(proto::sign_timeout_with_qc_response::Result::Signature(sig_bytes)) => {
                bcs::from_bytes(&sig_bytes).map_err(|e| {
                    Error::SerializationError(format!("Failed to deserialize signature: {}", e))
                })
            }
            Some(proto::sign_timeout_with_qc_response::Result::Error(err)) => {
                Err(Self::proto_error_to_error(&err))
            }
            None => Err(Error::InternalError(
                "Empty response from sign_timeout_with_qc".to_string(),
            )),
        }
    }

    fn construct_and_sign_vote_two_chain(
        &mut self,
        vote_proposal: &VoteProposal,
        timeout_cert: Option<&TwoChainTimeoutCertificate>,
    ) -> Result<Vote, Error> {
        let vote_proposal_bytes = bcs::to_bytes(vote_proposal).map_err(|e| {
            Error::SerializationError(format!("Failed to serialize VoteProposal: {}", e))
        })?;

        let timeout_cert_bytes = timeout_cert
            .map(|tc| bcs::to_bytes(tc))
            .transpose()
            .map_err(|e| {
                Error::SerializationError(format!(
                    "Failed to serialize TwoChainTimeoutCertificate: {}",
                    e
                ))
            })?;

        let response: proto::ConstructAndSignVoteTwoChainResponse =
            self.with_retry("construct_and_sign_vote_two_chain", |mut client| {
                let vote_proposal = vote_proposal_bytes.clone();
                let timeout_cert = timeout_cert_bytes.clone();
                async move {
                    client
                        .construct_and_sign_vote_two_chain(
                            proto::ConstructAndSignVoteTwoChainRequest {
                                vote_proposal,
                                timeout_cert,
                            },
                        )
                        .await
                }
            })?;

        match response.result {
            Some(proto::construct_and_sign_vote_two_chain_response::Result::Vote(vote_bytes)) => {
                bcs::from_bytes(&vote_bytes).map_err(|e| {
                    Error::SerializationError(format!("Failed to deserialize Vote: {}", e))
                })
            }
            Some(proto::construct_and_sign_vote_two_chain_response::Result::Error(err)) => {
                Err(Self::proto_error_to_error(&err))
            }
            None => Err(Error::InternalError(
                "Empty response from construct_and_sign_vote_two_chain".to_string(),
            )),
        }
    }

    fn construct_and_sign_order_vote(
        &mut self,
        order_vote_proposal: &OrderVoteProposal,
    ) -> Result<OrderVote, Error> {
        let order_vote_proposal_bytes = bcs::to_bytes(order_vote_proposal).map_err(|e| {
            Error::SerializationError(format!("Failed to serialize OrderVoteProposal: {}", e))
        })?;

        let response: proto::ConstructAndSignOrderVoteResponse =
            self.with_retry("construct_and_sign_order_vote", |mut client| {
                let order_vote_proposal = order_vote_proposal_bytes.clone();
                async move {
                    client
                        .construct_and_sign_order_vote(proto::ConstructAndSignOrderVoteRequest {
                            order_vote_proposal,
                        })
                        .await
                }
            })?;

        match response.result {
            Some(proto::construct_and_sign_order_vote_response::Result::OrderVote(
                order_vote_bytes,
            )) => bcs::from_bytes(&order_vote_bytes).map_err(|e| {
                Error::SerializationError(format!("Failed to deserialize OrderVote: {}", e))
            }),
            Some(proto::construct_and_sign_order_vote_response::Result::Error(err)) => {
                Err(Self::proto_error_to_error(&err))
            }
            None => Err(Error::InternalError(
                "Empty response from construct_and_sign_order_vote".to_string(),
            )),
        }
    }

    fn sign_commit_vote(
        &mut self,
        ledger_info: LedgerInfoWithSignatures,
        new_ledger_info: LedgerInfo,
    ) -> Result<bls12381::Signature, Error> {
        let ledger_info_bytes = bcs::to_bytes(&ledger_info).map_err(|e| {
            Error::SerializationError(format!(
                "Failed to serialize LedgerInfoWithSignatures: {}",
                e
            ))
        })?;

        let new_ledger_info_bytes = bcs::to_bytes(&new_ledger_info).map_err(|e| {
            Error::SerializationError(format!("Failed to serialize LedgerInfo: {}", e))
        })?;

        let response: proto::SignCommitVoteResponse =
            self.with_retry("sign_commit_vote", |mut client| {
                let ledger_info = ledger_info_bytes.clone();
                let new_ledger_info = new_ledger_info_bytes.clone();
                async move {
                    client
                        .sign_commit_vote(proto::SignCommitVoteRequest {
                            ledger_info,
                            new_ledger_info,
                        })
                        .await
                }
            })?;

        match response.result {
            Some(proto::sign_commit_vote_response::Result::Signature(sig_bytes)) => {
                bcs::from_bytes(&sig_bytes).map_err(|e| {
                    Error::SerializationError(format!("Failed to deserialize signature: {}", e))
                })
            }
            Some(proto::sign_commit_vote_response::Result::Error(err)) => {
                Err(Self::proto_error_to_error(&err))
            }
            None => Err(Error::InternalError(
                "Empty response from sign_commit_vote".to_string(),
            )),
        }
    }
}

impl Clone for RemoteSignerClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            runtime: self.runtime.clone(),
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proto_error_to_error_not_initialized() {
        let proto_err = proto::ErrorResponse {
            error_type: "NotInitialized".to_string(),
            message: "not initialized yet".to_string(),
        };
        match RemoteSignerClient::proto_error_to_error(&proto_err) {
            Error::NotInitialized(msg) => assert_eq!(msg, "not initialized yet"),
            other => panic!("Expected NotInitialized, got {:?}", other),
        }
    }

    #[test]
    fn test_proto_error_to_error_serialization() {
        let proto_err = proto::ErrorResponse {
            error_type: "SerializationError".to_string(),
            message: "failed to serialize".to_string(),
        };
        match RemoteSignerClient::proto_error_to_error(&proto_err) {
            Error::SerializationError(msg) => assert_eq!(msg, "failed to serialize"),
            other => panic!("Expected SerializationError, got {:?}", other),
        }
    }

    #[test]
    fn test_proto_error_to_error_invalid_qc() {
        let proto_err = proto::ErrorResponse {
            error_type: "InvalidQuorumCertificate".to_string(),
            message: "bad qc".to_string(),
        };
        match RemoteSignerClient::proto_error_to_error(&proto_err) {
            Error::InvalidQuorumCertificate(msg) => assert_eq!(msg, "bad qc"),
            other => panic!("Expected InvalidQuorumCertificate, got {:?}", other),
        }
    }

    #[test]
    fn test_proto_error_to_error_invalid_proposal() {
        let proto_err = proto::ErrorResponse {
            error_type: "InvalidProposal".to_string(),
            message: "bad proposal".to_string(),
        };
        match RemoteSignerClient::proto_error_to_error(&proto_err) {
            Error::InvalidProposal(msg) => assert_eq!(msg, "bad proposal"),
            other => panic!("Expected InvalidProposal, got {:?}", other),
        }
    }

    #[test]
    fn test_proto_error_to_error_unknown() {
        let proto_err = proto::ErrorResponse {
            error_type: "SomeUnknownError".to_string(),
            message: "unknown error message".to_string(),
        };
        match RemoteSignerClient::proto_error_to_error(&proto_err) {
            Error::InternalError(msg) => {
                assert!(msg.contains("SomeUnknownError"));
                assert!(msg.contains("unknown error message"));
            }
            other => panic!("Expected InternalError, got {:?}", other),
        }
    }

    #[test]
    fn test_remote_signer_config_timeout_conversion() {
        // Test that timeout conversion methods work correctly
        let config = RemoteSignerConfig {
            server_address: "https://localhost:8443".to_string(),
            tls_config: None,
            allow_insecure: false,
            connect_timeout_ms: 5000,
            request_timeout_ms: 10000,
            max_retries: 3,
            initial_backoff_ms: 100,
        };
        assert_eq!(config.connect_timeout(), Duration::from_millis(5000));
        assert_eq!(config.request_timeout(), Duration::from_millis(10000));
    }

    #[test]
    fn test_remote_signer_config_custom_timeouts() {
        let config = RemoteSignerConfig {
            server_address: "https://localhost:8443".to_string(),
            tls_config: None,
            allow_insecure: true,
            connect_timeout_ms: 1000,
            request_timeout_ms: 2000,
            max_retries: 5,
            initial_backoff_ms: 200,
        };
        assert_eq!(config.connect_timeout(), Duration::from_millis(1000));
        assert_eq!(config.request_timeout(), Duration::from_millis(2000));
    }

    #[test]
    fn test_remote_signer_config_insecure_mode() {
        // Test insecure mode configuration
        let config = RemoteSignerConfig {
            server_address: "http://localhost:8080".to_string(),
            tls_config: None,
            allow_insecure: true,
            connect_timeout_ms: 5000,
            request_timeout_ms: 10000,
            max_retries: 3,
            initial_backoff_ms: 100,
        };
        assert!(config.allow_insecure);
        assert!(config.tls_config.is_none());
    }
}
