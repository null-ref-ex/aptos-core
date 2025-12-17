// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! Configuration for the remote signer service.
//!
//! This module provides configuration structures for running the remote signer server.
//!
//! # Example Configuration (YAML)
//!
//! ```yaml
//! listen_address: "0.0.0.0:8443"
//!
//! # TLS configuration for secure connections
//! tls:
//!   cert_path: "/etc/signer/certs/server.pem"
//!   key_path: "/etc/signer/certs/server-key.pem"
//!   client_ca_cert_path: "/etc/signer/certs/client-ca.pem"  # For mTLS
//!
//! # Secure storage backend (on_disk or vault)
//! backend:
//!   type: on_disk
//!   path: "/etc/signer/secure_storage.json"
//!
//! # Validator identity initialization
//! initial_safety_rules_config:
//!   from_file:
//!     identity_blob_path: "/etc/signer/identity.yaml"
//!     waypoint:
//!       from_config: "0:abc123..."
//! ```
//!
//! # Testing Configuration (Insecure)
//!
//! For local testing, TLS can be omitted:
//!
//! ```yaml
//! listen_address: "0.0.0.0:8080"
//! backend:
//!   type: on_disk
//!   path: "/tmp/safety_storage.json"
//! initial_safety_rules_config:
//!   from_file:
//!     identity_blob_path: "/path/to/identity.yaml"
//!     waypoint:
//!       from_config: "0:..."
//! ```

use aptos_config::config::{SecureBackend, WaypointConfig};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// TLS configuration for the server.
///
/// When `client_ca_cert_path` is provided, the server will require and verify
/// client certificates (mutual TLS). This is strongly recommended for production.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServerTlsConfig {
    /// Path to server certificate (PEM format)
    pub cert_path: PathBuf,
    /// Path to server private key (PEM format)
    pub key_path: PathBuf,
    /// Path to CA certificate for client verification (enables mTLS).
    /// When set, clients must present a valid certificate signed by this CA.
    pub client_ca_cert_path: Option<PathBuf>,
}

/// Initial configuration for loading validator identity.
///
/// The remote signer needs to be initialized with the validator's consensus key
/// and waypoint before it can process signing requests.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InitialConfig {
    /// Load identity from file(s) at startup.
    ///
    /// This is the typical configuration for validators where the identity
    /// blob is pre-generated and stored securely.
    FromFile {
        /// Path to the primary identity blob file containing the consensus key
        identity_blob_path: PathBuf,
        /// Additional identity files for key rotation.
        /// These keys will be added to storage alongside the primary key,
        /// allowing the signer to use either key during rotation periods.
        #[serde(default)]
        overriding_identity_paths: Vec<PathBuf>,
        /// Waypoint configuration for chain verification
        waypoint: WaypointConfig,
    },
    /// No initial configuration.
    ///
    /// The safety rules must be initialized via the `initialize` RPC call
    /// before any signing operations can be performed.
    None,
}

/// Configuration for the remote signer server.
///
/// This struct is typically loaded from a YAML configuration file.
///
/// # Security Considerations
///
/// - Always use TLS in production environments
/// - Enable mTLS by providing `client_ca_cert_path` in the TLS config
/// - Use a secure storage backend (Vault recommended for production)
/// - Restrict network access to the listen address
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteSignerServerConfig {
    /// Address to listen on (e.g., "0.0.0.0:8443" or "127.0.0.1:8443")
    pub listen_address: String,
    /// TLS configuration. Omit for insecure connections (testing only).
    ///
    /// **WARNING**: Running without TLS exposes signing operations to network attacks.
    /// Only omit TLS for local development and testing.
    #[serde(default)]
    pub tls: Option<ServerTlsConfig>,
    /// Backend for secure storage of safety data and keys.
    ///
    /// Supported backends:
    /// - `on_disk`: Local file storage (suitable for single-node deployments)
    /// - `vault`: HashiCorp Vault (recommended for production)
    pub backend: SecureBackend,
    /// Initial safety rules configuration.
    ///
    /// Determines how the validator identity is loaded at startup.
    pub initial_safety_rules_config: InitialConfig,
    /// Whether to cache safety data in memory for performance.
    ///
    /// When enabled, safety data is kept in memory and periodically persisted.
    /// This improves signing latency but may result in data loss on crash.
    /// Default: true
    #[serde(default = "default_enable_cached_safety_data")]
    pub enable_cached_safety_data: bool,
}

fn default_enable_cached_safety_data() -> bool {
    true
}

impl RemoteSignerServerConfig {
    /// Load configuration from a YAML file
    pub fn load_from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
