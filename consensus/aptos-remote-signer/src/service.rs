// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! gRPC service implementation for the remote signer.
//!
//! This service wraps SafetyRules and exposes it over gRPC. All safety rule
//! validation and signing operations are performed server-side.

use aptos_consensus_types::{
    block_data::BlockData,
    order_vote_proposal::OrderVoteProposal,
    timeout_2chain::{TwoChainTimeout, TwoChainTimeoutCertificate},
    vote_proposal::VoteProposal,
};
use aptos_logger::{debug, error, info};
use aptos_protos::safety_rules::v1::{
    self as proto,
    safety_rules_service_server::SafetyRulesService,
};
use aptos_safety_rules::{Error as SafetyRulesError, SafetyRules, TSafetyRules};
use aptos_types::{
    epoch_change::EpochChangeProof,
    ledger_info::{LedgerInfo, LedgerInfoWithSignatures},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

/// The gRPC service implementation for remote signing.
pub struct SafetyRulesServiceImpl {
    safety_rules: Arc<Mutex<SafetyRules>>,
}

impl SafetyRulesServiceImpl {
    /// Creates a new SafetyRulesServiceImpl wrapping the provided SafetyRules instance.
    pub fn new(safety_rules: SafetyRules) -> Self {
        info!("Created SafetyRulesServiceImpl");
        Self {
            safety_rules: Arc::new(Mutex::new(safety_rules)),
        }
    }

    /// Converts a SafetyRulesError to a proto ErrorResponse.
    fn error_to_proto(err: &SafetyRulesError) -> proto::ErrorResponse {
        proto::ErrorResponse {
            error_type: format!("{:?}", err).split('(').next().unwrap_or("Unknown").to_string(),
            message: err.to_string(),
        }
    }
}

#[tonic::async_trait]
impl SafetyRulesService for SafetyRulesServiceImpl {
    async fn consensus_state(
        &self,
        _request: Request<proto::ConsensusStateRequest>,
    ) -> Result<Response<proto::ConsensusStateResponse>, Status> {
        debug!("consensus_state request received");

        let mut safety_rules = self.safety_rules.lock().await;
        match safety_rules.consensus_state() {
            Ok(state) => {
                let waypoint_bytes = bcs::to_bytes(&state.waypoint()).map_err(|e| {
                    Status::internal(format!("Failed to serialize waypoint: {}", e))
                })?;

                Ok(Response::new(proto::ConsensusStateResponse {
                    result: Some(proto::consensus_state_response::Result::State(
                        proto::ConsensusState {
                            epoch: state.epoch(),
                            last_voted_round: state.last_voted_round(),
                            preferred_round: state.preferred_round(),
                            waypoint: waypoint_bytes,
                            in_validator_set: state.in_validator_set(),
                        },
                    )),
                }))
            }
            Err(e) => {
                error!("consensus_state failed: {}", e);
                Ok(Response::new(proto::ConsensusStateResponse {
                    result: Some(proto::consensus_state_response::Result::Error(
                        Self::error_to_proto(&e),
                    )),
                }))
            }
        }
    }

    async fn initialize(
        &self,
        request: Request<proto::InitializeRequest>,
    ) -> Result<Response<proto::InitializeResponse>, Status> {
        debug!("initialize request received");

        let req = request.into_inner();
        let proof: EpochChangeProof = bcs::from_bytes(&req.epoch_change_proof).map_err(|e| {
            Status::invalid_argument(format!("Failed to deserialize EpochChangeProof: {}", e))
        })?;

        let mut safety_rules = self.safety_rules.lock().await;
        match safety_rules.initialize(&proof) {
            Ok(()) => {
                info!("Safety rules initialized successfully");
                Ok(Response::new(proto::InitializeResponse {
                    result: Some(proto::initialize_response::Result::Success(proto::Empty {})),
                }))
            }
            Err(e) => {
                error!("initialize failed: {}", e);
                Ok(Response::new(proto::InitializeResponse {
                    result: Some(proto::initialize_response::Result::Error(
                        Self::error_to_proto(&e),
                    )),
                }))
            }
        }
    }

    async fn sign_proposal(
        &self,
        request: Request<proto::SignProposalRequest>,
    ) -> Result<Response<proto::SignProposalResponse>, Status> {
        debug!("sign_proposal request received");

        let req = request.into_inner();
        let block_data: BlockData = bcs::from_bytes(&req.block_data).map_err(|e| {
            Status::invalid_argument(format!("Failed to deserialize BlockData: {}", e))
        })?;

        let mut safety_rules = self.safety_rules.lock().await;
        match safety_rules.sign_proposal(&block_data) {
            Ok(signature) => {
                let sig_bytes = bcs::to_bytes(&signature).map_err(|e| {
                    Status::internal(format!("Failed to serialize signature: {}", e))
                })?;
                Ok(Response::new(proto::SignProposalResponse {
                    result: Some(proto::sign_proposal_response::Result::Signature(sig_bytes)),
                }))
            }
            Err(e) => {
                error!("sign_proposal failed: {}", e);
                Ok(Response::new(proto::SignProposalResponse {
                    result: Some(proto::sign_proposal_response::Result::Error(
                        Self::error_to_proto(&e),
                    )),
                }))
            }
        }
    }

    async fn sign_timeout_with_qc(
        &self,
        request: Request<proto::SignTimeoutWithQcRequest>,
    ) -> Result<Response<proto::SignTimeoutWithQcResponse>, Status> {
        debug!("sign_timeout_with_qc request received");

        let req = request.into_inner();
        let timeout: TwoChainTimeout = bcs::from_bytes(&req.timeout).map_err(|e| {
            Status::invalid_argument(format!("Failed to deserialize TwoChainTimeout: {}", e))
        })?;

        let timeout_cert: Option<TwoChainTimeoutCertificate> = req
            .timeout_cert
            .map(|bytes| bcs::from_bytes(&bytes))
            .transpose()
            .map_err(|e| {
                Status::invalid_argument(format!(
                    "Failed to deserialize TwoChainTimeoutCertificate: {}",
                    e
                ))
            })?;

        let mut safety_rules = self.safety_rules.lock().await;
        match safety_rules.sign_timeout_with_qc(&timeout, timeout_cert.as_ref()) {
            Ok(signature) => {
                let sig_bytes = bcs::to_bytes(&signature).map_err(|e| {
                    Status::internal(format!("Failed to serialize signature: {}", e))
                })?;
                Ok(Response::new(proto::SignTimeoutWithQcResponse {
                    result: Some(proto::sign_timeout_with_qc_response::Result::Signature(
                        sig_bytes,
                    )),
                }))
            }
            Err(e) => {
                error!("sign_timeout_with_qc failed: {}", e);
                Ok(Response::new(proto::SignTimeoutWithQcResponse {
                    result: Some(proto::sign_timeout_with_qc_response::Result::Error(
                        Self::error_to_proto(&e),
                    )),
                }))
            }
        }
    }

    async fn construct_and_sign_vote_two_chain(
        &self,
        request: Request<proto::ConstructAndSignVoteTwoChainRequest>,
    ) -> Result<Response<proto::ConstructAndSignVoteTwoChainResponse>, Status> {
        debug!("construct_and_sign_vote_two_chain request received");

        let req = request.into_inner();
        let vote_proposal: VoteProposal = bcs::from_bytes(&req.vote_proposal).map_err(|e| {
            Status::invalid_argument(format!("Failed to deserialize VoteProposal: {}", e))
        })?;

        let timeout_cert: Option<TwoChainTimeoutCertificate> = req
            .timeout_cert
            .map(|bytes| bcs::from_bytes(&bytes))
            .transpose()
            .map_err(|e| {
                Status::invalid_argument(format!(
                    "Failed to deserialize TwoChainTimeoutCertificate: {}",
                    e
                ))
            })?;

        let mut safety_rules = self.safety_rules.lock().await;
        match safety_rules.construct_and_sign_vote_two_chain(&vote_proposal, timeout_cert.as_ref())
        {
            Ok(vote) => {
                let vote_bytes = bcs::to_bytes(&vote).map_err(|e| {
                    Status::internal(format!("Failed to serialize Vote: {}", e))
                })?;
                Ok(Response::new(proto::ConstructAndSignVoteTwoChainResponse {
                    result: Some(
                        proto::construct_and_sign_vote_two_chain_response::Result::Vote(vote_bytes),
                    ),
                }))
            }
            Err(e) => {
                error!("construct_and_sign_vote_two_chain failed: {}", e);
                Ok(Response::new(proto::ConstructAndSignVoteTwoChainResponse {
                    result: Some(
                        proto::construct_and_sign_vote_two_chain_response::Result::Error(
                            Self::error_to_proto(&e),
                        ),
                    ),
                }))
            }
        }
    }

    async fn construct_and_sign_order_vote(
        &self,
        request: Request<proto::ConstructAndSignOrderVoteRequest>,
    ) -> Result<Response<proto::ConstructAndSignOrderVoteResponse>, Status> {
        debug!("construct_and_sign_order_vote request received");

        let req = request.into_inner();
        let order_vote_proposal: OrderVoteProposal =
            bcs::from_bytes(&req.order_vote_proposal).map_err(|e| {
                Status::invalid_argument(format!(
                    "Failed to deserialize OrderVoteProposal: {}",
                    e
                ))
            })?;

        let mut safety_rules = self.safety_rules.lock().await;
        match safety_rules.construct_and_sign_order_vote(&order_vote_proposal) {
            Ok(order_vote) => {
                let order_vote_bytes = bcs::to_bytes(&order_vote).map_err(|e| {
                    Status::internal(format!("Failed to serialize OrderVote: {}", e))
                })?;
                Ok(Response::new(proto::ConstructAndSignOrderVoteResponse {
                    result: Some(
                        proto::construct_and_sign_order_vote_response::Result::OrderVote(
                            order_vote_bytes,
                        ),
                    ),
                }))
            }
            Err(e) => {
                error!("construct_and_sign_order_vote failed: {}", e);
                Ok(Response::new(proto::ConstructAndSignOrderVoteResponse {
                    result: Some(
                        proto::construct_and_sign_order_vote_response::Result::Error(
                            Self::error_to_proto(&e),
                        ),
                    ),
                }))
            }
        }
    }

    async fn sign_commit_vote(
        &self,
        request: Request<proto::SignCommitVoteRequest>,
    ) -> Result<Response<proto::SignCommitVoteResponse>, Status> {
        debug!("sign_commit_vote request received");

        let req = request.into_inner();
        let ledger_info: LedgerInfoWithSignatures =
            bcs::from_bytes(&req.ledger_info).map_err(|e| {
                Status::invalid_argument(format!(
                    "Failed to deserialize LedgerInfoWithSignatures: {}",
                    e
                ))
            })?;

        let new_ledger_info: LedgerInfo =
            bcs::from_bytes(&req.new_ledger_info).map_err(|e| {
                Status::invalid_argument(format!("Failed to deserialize LedgerInfo: {}", e))
            })?;

        let mut safety_rules = self.safety_rules.lock().await;
        match safety_rules.sign_commit_vote(ledger_info, new_ledger_info) {
            Ok(signature) => {
                let sig_bytes = bcs::to_bytes(&signature).map_err(|e| {
                    Status::internal(format!("Failed to serialize signature: {}", e))
                })?;
                Ok(Response::new(proto::SignCommitVoteResponse {
                    result: Some(proto::sign_commit_vote_response::Result::Signature(
                        sig_bytes,
                    )),
                }))
            }
            Err(e) => {
                error!("sign_commit_vote failed: {}", e);
                Ok(Response::new(proto::SignCommitVoteResponse {
                    result: Some(proto::sign_commit_vote_response::Result::Error(
                        Self::error_to_proto(&e),
                    )),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_to_proto_not_initialized() {
        let err = SafetyRulesError::NotInitialized("test error".to_string());
        let proto_err = SafetyRulesServiceImpl::error_to_proto(&err);
        assert_eq!(proto_err.error_type, "NotInitialized");
        assert!(proto_err.message.contains("test error"));
    }

    #[test]
    fn test_error_to_proto_serialization_error() {
        let err = SafetyRulesError::SerializationError("bad data".to_string());
        let proto_err = SafetyRulesServiceImpl::error_to_proto(&err);
        assert_eq!(proto_err.error_type, "SerializationError");
        assert!(proto_err.message.contains("bad data"));
    }

    #[test]
    fn test_error_to_proto_internal_error() {
        let err = SafetyRulesError::InternalError("something went wrong".to_string());
        let proto_err = SafetyRulesServiceImpl::error_to_proto(&err);
        assert_eq!(proto_err.error_type, "InternalError");
        assert!(proto_err.message.contains("something went wrong"));
    }

    #[test]
    fn test_error_to_proto_invalid_qc() {
        let err = SafetyRulesError::InvalidQuorumCertificate("qc error".to_string());
        let proto_err = SafetyRulesServiceImpl::error_to_proto(&err);
        assert_eq!(proto_err.error_type, "InvalidQuorumCertificate");
        assert!(proto_err.message.contains("qc error"));
    }

    #[test]
    fn test_error_to_proto_invalid_proposal() {
        let err = SafetyRulesError::InvalidProposal("proposal error".to_string());
        let proto_err = SafetyRulesServiceImpl::error_to_proto(&err);
        assert_eq!(proto_err.error_type, "InvalidProposal");
        assert!(proto_err.message.contains("proposal error"));
    }
}
