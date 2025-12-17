// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! Aptos Remote Signer Service
//!
//! A standalone gRPC service that holds validator private keys and performs
//! consensus signing operations. This allows the validator node to be decoupled
//! from the signing functionality, improving security by isolating key material.
//!
//! # Usage
//!
//! ```bash
//! aptos-remote-signer --config /path/to/config.yaml
//! ```

mod config;
mod service;

use aptos_config::config::IdentityBlob;
use aptos_logger::{info, Level, Logger};
use aptos_protos::safety_rules::v1::safety_rules_service_server::SafetyRulesServiceServer;
use aptos_safety_rules::{PersistentSafetyStorage, SafetyRules};
use aptos_secure_storage::{KVStorage, Storage};
use clap::Parser;
use config::{InitialConfig, RemoteSignerServerConfig};
use service::SafetyRulesServiceImpl;
use std::path::PathBuf;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};

#[derive(Parser)]
#[clap(
    name = "aptos-remote-signer",
    about = "Aptos Remote Signer Service for validator consensus operations"
)]
struct Args {
    /// Path to the configuration file
    #[clap(short, long)]
    config: PathBuf,

    /// Log level (error, warn, info, debug, trace)
    #[clap(long, default_value = "info")]
    log_level: Level,
}

/// Creates the PersistentSafetyStorage from configuration
fn create_storage(config: &RemoteSignerServerConfig) -> anyhow::Result<PersistentSafetyStorage> {
    let backend = &config.backend;
    let internal_storage: Storage = backend.into();

    internal_storage
        .available()
        .map_err(|e| anyhow::anyhow!("Storage is not available: {:?}", e))?;

    match &config.initial_safety_rules_config {
        InitialConfig::FromFile {
            identity_blob_path,
            overriding_identity_paths,
            waypoint,
        } => {
            let identity_blob = IdentityBlob::from_file(identity_blob_path)?;
            let waypoint = waypoint.waypoint();

            let storage = PersistentSafetyStorage::initialize(
                internal_storage,
                identity_blob
                    .account_address
                    .ok_or_else(|| anyhow::anyhow!("AccountAddress needed for safety rules"))?,
                identity_blob
                    .consensus_private_key
                    .ok_or_else(|| anyhow::anyhow!("Consensus key needed for safety rules"))?,
                waypoint,
                config.enable_cached_safety_data,
            );

            // Handle overriding identity blobs for key rotation
            let mut storage = storage;
            for path in overriding_identity_paths {
                let blob = IdentityBlob::from_file(path)?;
                if let Some(sk) = blob.consensus_private_key {
                    use aptos_crypto::bls12381::PublicKey;
                    use aptos_global_constants::CONSENSUS_KEY;
                    use aptos_secure_storage::KVStorage;

                    let pk_hex = hex::encode(PublicKey::from(&sk).to_bytes());
                    let storage_key = format!("{}_{}", CONSENSUS_KEY, pk_hex);
                    if let Err(e) = storage.internal_store().set(storage_key.as_str(), sk) {
                        info!("Setting {} failed: {}", storage_key, e);
                    } else {
                        info!("Setting {} succeeded", storage_key);
                    }
                }
            }

            Ok(storage)
        }
        InitialConfig::None => {
            // Check if already initialized
            let storage =
                PersistentSafetyStorage::new(internal_storage, config.enable_cached_safety_data);
            if storage.author().is_err() {
                anyhow::bail!(
                    "Safety rules storage is not initialized, provide an initial config"
                );
            }
            Ok(storage)
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize logging
    Logger::builder().level(args.log_level).build();

    info!("Loading configuration from {:?}", args.config);
    let config = RemoteSignerServerConfig::load_from_file(&args.config)?;

    info!("Initializing storage");
    let storage = create_storage(&config)?;

    info!("Creating SafetyRules instance");
    let safety_rules = SafetyRules::new(storage, false);
    let service = SafetyRulesServiceImpl::new(safety_rules);

    let listen_addr = config
        .listen_address
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid listen address: {}", e))?;

    let mut server_builder = Server::builder();

    // Configure TLS if provided
    if let Some(tls_config) = &config.tls {
        info!("Configuring TLS");
        let cert = tokio::fs::read(&tls_config.cert_path).await?;
        let key = tokio::fs::read(&tls_config.key_path).await?;
        let identity = Identity::from_pem(cert, key);

        let mut tls = ServerTlsConfig::new().identity(identity);

        // Configure client certificate verification for mTLS
        if let Some(ca_cert_path) = &tls_config.client_ca_cert_path {
            info!("Configuring mTLS with client certificate verification");
            let ca_cert = tokio::fs::read(ca_cert_path).await?;
            tls = tls.client_ca_root(Certificate::from_pem(ca_cert));
        }

        server_builder = server_builder.tls_config(tls)?;
    } else {
        info!("WARNING: Running without TLS - not recommended for production");
    }

    info!("Starting remote signer service on {}", config.listen_address);

    // Handle shutdown signals
    let shutdown = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
        info!("Received shutdown signal");
    };

    server_builder
        .add_service(SafetyRulesServiceServer::new(service))
        .serve_with_shutdown(listen_addr, shutdown)
        .await?;

    info!("Remote signer service stopped");
    Ok(())
}
