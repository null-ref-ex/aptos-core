// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

#[cfg(test)]
use crate::config::persistable_config::PersistableConfig;
use crate::{
    config::{
        config_sanitizer::ConfigSanitizer, node_config_loader::NodeType, Error, IdentityBlob,
        LoggerConfig, NodeConfig, SecureBackend, WaypointConfig,
    },
    keys::ConfigKey,
};
use anyhow::bail;
use aptos_crypto::{bls12381, Uniform};
use aptos_types::{chain_id::ChainId, network_address::NetworkAddress, waypoint::Waypoint, PeerId};
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};
use std::{
    net::{SocketAddr, ToSocketAddrs},
    path::PathBuf,
    time::Duration,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct SafetyRulesConfig {
    pub backend: SecureBackend,
    pub logger: LoggerConfig,
    pub service: SafetyRulesService,
    pub test: Option<SafetyRulesTestConfig>,
    // Read/Write/Connect networking operation timeout in milliseconds.
    pub network_timeout_ms: u64,
    pub enable_cached_safety_data: bool,
    pub initial_safety_rules_config: InitialSafetyRulesConfig,
}

impl Default for SafetyRulesConfig {
    fn default() -> Self {
        Self {
            backend: SecureBackend::InMemoryStorage,
            logger: LoggerConfig::default(),
            service: SafetyRulesService::Local,
            test: None,
            // Default value of 30 seconds for a timeout
            network_timeout_ms: 30_000,
            enable_cached_safety_data: true,
            initial_safety_rules_config: InitialSafetyRulesConfig::None,
        }
    }
}

impl SafetyRulesConfig {
    pub fn set_data_dir(&mut self, data_dir: PathBuf) {
        if let SecureBackend::OnDiskStorage(backend) = &mut self.backend {
            backend.set_data_dir(data_dir);
        }
    }

    #[cfg(test)]
    /// Returns the default safety rules config for a validator (only used by tests)
    pub fn get_default_config() -> Self {
        let contents = include_str!("test_data/safety_rules.yaml");
        SafetyRulesConfig::parse_serialized_config(contents).unwrap_or_else(|error| {
            panic!(
                "Failed to parse default safety rules config! Error: {}",
                error
            )
        })
    }
}

impl ConfigSanitizer for SafetyRulesConfig {
    fn sanitize(
        node_config: &NodeConfig,
        node_type: NodeType,
        chain_id: Option<ChainId>,
    ) -> Result<(), Error> {
        let sanitizer_name = Self::get_sanitizer_name();
        let safety_rules_config = &node_config.consensus.safety_rules;

        // If the node is not a validator, there's nothing to be done
        if !node_type.is_validator() {
            return Ok(());
        }

        if let Some(chain_id) = chain_id {
            // Verify that the secure backend is appropriate for mainnet validators
            if chain_id.is_mainnet()
                && node_type.is_validator()
                && safety_rules_config.backend.is_in_memory()
            {
                return Err(Error::ConfigSanitizerFailed(
                    sanitizer_name,
                    "The secure backend should not be set to in memory storage in mainnet!"
                        .to_string(),
                ));
            }

            // Verify that the safety rules service is set to local for optimal performance
            if chain_id.is_mainnet() && !safety_rules_config.service.is_local() {
                return Err(Error::ConfigSanitizerFailed(
                    sanitizer_name,
                    format!("The safety rules service should be set to local in mainnet for optimal performance! Given config: {:?}", &safety_rules_config.service)
                ));
            }

            // Verify that the safety rules test config is not enabled in mainnet
            if chain_id.is_mainnet() && safety_rules_config.test.is_some() {
                return Err(Error::ConfigSanitizerFailed(
                    sanitizer_name,
                    "The safety rules test config should not be used in mainnet!".to_string(),
                ));
            }
        }

        Ok(())
    }
}

// TODO: Find a cleaner way so WaypointConfig isn't duplicated
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InitialSafetyRulesConfig {
    FromFile {
        identity_blob_path: PathBuf,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        overriding_identity_paths: Vec<PathBuf>,
        waypoint: WaypointConfig,
    },
    None,
}

impl InitialSafetyRulesConfig {
    pub fn from_file(
        identity_blob_path: PathBuf,
        overriding_identity_paths: Vec<PathBuf>,
        waypoint: WaypointConfig,
    ) -> Self {
        Self::FromFile {
            identity_blob_path,
            overriding_identity_paths,
            waypoint,
        }
    }

    pub fn waypoint(&self) -> Waypoint {
        match self {
            InitialSafetyRulesConfig::FromFile { waypoint, .. } => waypoint.waypoint(),
            InitialSafetyRulesConfig::None => panic!("Must have a waypoint"),
        }
    }

    pub fn has_identity_blob(&self) -> bool {
        match self {
            InitialSafetyRulesConfig::FromFile { .. } => true,
            InitialSafetyRulesConfig::None => false,
        }
    }

    pub fn identity_blob(&self) -> anyhow::Result<IdentityBlob> {
        match self {
            InitialSafetyRulesConfig::FromFile {
                identity_blob_path, ..
            } => IdentityBlob::from_file(identity_blob_path),
            InitialSafetyRulesConfig::None => {
                bail!("loading identity blob failed with missing initial safety rules config")
            },
        }
    }

    pub fn overriding_identity_blobs(&self) -> anyhow::Result<Vec<IdentityBlob>> {
        match self {
            InitialSafetyRulesConfig::FromFile {
                overriding_identity_paths,
                ..
            } => {
                let mut blobs = vec![];
                for path in overriding_identity_paths {
                    let blob = IdentityBlob::from_file(path)?;
                    blobs.push(blob);
                }
                Ok(blobs)
            },
            InitialSafetyRulesConfig::None => {
                bail!("loading overriding identity blobs failed with missing initial safety rules config")
            },
        }
    }

    #[cfg(feature = "smoke-test")]
    pub fn overriding_identity_blob_paths_mut(&mut self) -> &mut Vec<PathBuf> {
        match self {
            InitialSafetyRulesConfig::FromFile {
                overriding_identity_paths,
                ..
            } => overriding_identity_paths,
            InitialSafetyRulesConfig::None => {
                unreachable!()
            },
        }
    }
}

/// TLS configuration for the remote signer client.
///
/// All three certificate paths are required for mutual TLS (mTLS) authentication.
/// The client will present its certificate to the server, and verify the server's
/// certificate against the provided CA.
///
/// # Example
///
/// ```yaml
/// tls_config:
///   ca_cert_path: "/etc/aptos/certs/ca.pem"
///   client_cert_path: "/etc/aptos/certs/validator.pem"
///   client_key_path: "/etc/aptos/certs/validator-key.pem"
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteSignerTlsConfig {
    /// Path to CA certificate for verifying the server (PEM format)
    pub ca_cert_path: PathBuf,
    /// Path to client certificate for mutual TLS authentication (PEM format)
    pub client_cert_path: PathBuf,
    /// Path to client private key for mutual TLS authentication (PEM format)
    pub client_key_path: PathBuf,
}

/// Configuration for connecting to a remote signer service.
///
/// The remote signer client delegates all consensus signing operations to
/// a separate service via gRPC. This improves security by isolating private
/// keys from the validator node.
///
/// # Production Example (with mTLS)
///
/// ```yaml
/// safety_rules:
///   service:
///     type: remote_signer
///     server_address: "https://signer.internal:8443"
///     tls_config:
///       ca_cert_path: "/etc/aptos/certs/ca.pem"
///       client_cert_path: "/etc/aptos/certs/validator.pem"
///       client_key_path: "/etc/aptos/certs/validator-key.pem"
///     connect_timeout_ms: 5000
///     request_timeout_ms: 10000
///     max_retries: 3
/// ```
///
/// # Testing Example (insecure)
///
/// ```yaml
/// safety_rules:
///   service:
///     type: remote_signer
///     server_address: "http://localhost:8080"
///     allow_insecure: true
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteSignerConfig {
    /// Server address including scheme and port.
    ///
    /// Use `https://` for TLS connections (required for production).
    /// Use `http://` only with `allow_insecure: true` for testing.
    ///
    /// Examples: `"https://signer.internal:8443"`, `"http://localhost:8080"`
    pub server_address: String,
    /// TLS configuration for secure connections.
    ///
    /// Required unless `allow_insecure` is set to `true`.
    /// Provides mutual TLS authentication where both the client and server
    /// verify each other's certificates.
    #[serde(default)]
    pub tls_config: Option<RemoteSignerTlsConfig>,
    /// Allow connections without TLS encryption.
    ///
    /// **WARNING**: Only enable this for local development and testing.
    /// Insecure connections expose signing operations to network attacks.
    ///
    /// When `true` and `tls_config` is `None`, connects without TLS.
    /// When `false` (default), `tls_config` must be provided.
    #[serde(default)]
    pub allow_insecure: bool,
    /// Connection timeout in milliseconds.
    ///
    /// How long to wait when establishing a connection to the remote signer.
    /// Default: 5000ms (5 seconds)
    #[serde(default = "RemoteSignerConfig::default_connect_timeout_ms")]
    pub connect_timeout_ms: u64,
    /// Request timeout in milliseconds.
    ///
    /// How long to wait for each signing request to complete.
    /// Default: 10000ms (10 seconds)
    #[serde(default = "RemoteSignerConfig::default_request_timeout_ms")]
    pub request_timeout_ms: u64,
    /// Maximum number of retry attempts for failed requests.
    ///
    /// Uses exponential backoff between retries starting from `initial_backoff_ms`.
    /// Default: 3
    #[serde(default = "RemoteSignerConfig::default_max_retries")]
    pub max_retries: u32,
    /// Initial backoff delay in milliseconds for retry attempts.
    ///
    /// The backoff doubles with each retry, up to a maximum of 30 seconds.
    /// Default: 100ms
    #[serde(default = "RemoteSignerConfig::default_initial_backoff_ms")]
    pub initial_backoff_ms: u64,
}

impl RemoteSignerConfig {
    fn default_connect_timeout_ms() -> u64 {
        5000
    }

    fn default_request_timeout_ms() -> u64 {
        10000
    }

    fn default_max_retries() -> u32 {
        3
    }

    fn default_initial_backoff_ms() -> u64 {
        100
    }

    /// Returns the connect timeout as a Duration
    pub fn connect_timeout(&self) -> Duration {
        Duration::from_millis(self.connect_timeout_ms)
    }

    /// Returns the request timeout as a Duration
    pub fn request_timeout(&self) -> Duration {
        Duration::from_millis(self.request_timeout_ms)
    }
}

/// Defines how safety rules should be executed
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum SafetyRulesService {
    /// This runs safety rules in the same thread as event processor
    Local,
    /// This is the production, separate service approach
    Process(RemoteService),
    /// This runs safety rules in the same thread as event processor but data is passed through the
    /// light weight RPC (serializer)
    Serializer,
    /// This creates a separate thread to run safety rules, it is similar to a fork / exec style
    Thread,
    /// Remote signer service via gRPC with mTLS authentication
    RemoteSigner(RemoteSignerConfig),
}

impl SafetyRulesService {
    /// Returns true iff the service is local
    fn is_local(&self) -> bool {
        matches!(self, SafetyRulesService::Local)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteService {
    pub server_address: NetworkAddress,
}

impl RemoteService {
    pub fn server_address(&self) -> SocketAddr {
        self.server_address
            .to_socket_addrs()
            .expect("server_address invalid")
            .next()
            .expect("server_address invalid")
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SafetyRulesTestConfig {
    pub author: PeerId,
    pub consensus_key: Option<ConfigKey<bls12381::PrivateKey>>,
    pub waypoint: Option<Waypoint>,
}

impl SafetyRulesTestConfig {
    pub fn new(author: PeerId) -> Self {
        Self {
            author,
            consensus_key: None,
            waypoint: None,
        }
    }

    pub fn consensus_key(&mut self, key: bls12381::PrivateKey) {
        self.consensus_key = Some(ConfigKey::new(key));
    }

    pub fn random_consensus_key(&mut self, rng: &mut StdRng) {
        let privkey = bls12381::PrivateKey::generate(rng);
        self.consensus_key = Some(ConfigKey::<bls12381::PrivateKey>::new(privkey));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConsensusConfig;

    #[test]
    fn test_sanitize_invalid_backend_for_mainnet() {
        // Create a node config with an invalid backend for mainnet
        let node_config = NodeConfig {
            consensus: ConsensusConfig {
                safety_rules: SafetyRulesConfig {
                    backend: SecureBackend::InMemoryStorage,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        // Verify that the config sanitizer fails
        let error = SafetyRulesConfig::sanitize(
            &node_config,
            NodeType::Validator,
            Some(ChainId::mainnet()),
        )
        .unwrap_err();
        assert!(matches!(error, Error::ConfigSanitizerFailed(_, _)));
    }

    #[test]
    fn test_sanitize_backend_for_mainnet_fullnodes() {
        // Create a node config with an invalid backend for mainnet validators
        let node_config = NodeConfig {
            consensus: ConsensusConfig {
                safety_rules: SafetyRulesConfig {
                    backend: SecureBackend::InMemoryStorage,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        // Verify that the config sanitizer passes because the node is a fullnode
        SafetyRulesConfig::sanitize(
            &node_config,
            NodeType::PublicFullnode,
            Some(ChainId::mainnet()),
        )
        .unwrap();
    }

    #[test]
    fn test_sanitize_invalid_service_for_mainnet() {
        // Create a node config with a non-local service
        let node_config = NodeConfig {
            consensus: ConsensusConfig {
                safety_rules: SafetyRulesConfig {
                    service: SafetyRulesService::Serializer,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        // Verify that the config sanitizer fails
        let error = SafetyRulesConfig::sanitize(
            &node_config,
            NodeType::Validator,
            Some(ChainId::mainnet()),
        )
        .unwrap_err();
        assert!(matches!(error, Error::ConfigSanitizerFailed(_, _)));
    }

    #[test]
    fn test_sanitize_test_config_on_mainnet() {
        // Create a node config with a test config
        let node_config = NodeConfig {
            consensus: ConsensusConfig {
                safety_rules: SafetyRulesConfig {
                    test: Some(SafetyRulesTestConfig::new(PeerId::random())),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        // Verify that the config sanitizer fails
        let error = SafetyRulesConfig::sanitize(
            &node_config,
            NodeType::Validator,
            Some(ChainId::mainnet()),
        )
        .unwrap_err();
        assert!(matches!(error, Error::ConfigSanitizerFailed(_, _)));
    }
}
