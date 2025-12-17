// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! # Aptos Remote Signer Service
//!
//! A standalone gRPC service that holds validator private keys and performs
//! consensus signing operations. This allows the validator node to be decoupled
//! from the signing functionality, improving security by isolating key material.
//!
//! ## Architecture
//!
//! The remote signer implements the `TSafetyRules` trait over gRPC, allowing
//! validators to delegate all signing operations to a separate process:
//!
//! ```text
//! +-------------------+       gRPC/mTLS        +----------------------+
//! |  Validator Node   |  ------------------>   |  aptos-remote-signer |
//! |                   |  <------------------   |                      |
//! |  RemoteSignerClient                        |  SafetyRules         |
//! |  (TSafetyRules)   |                        |  ValidatorSigner     |
//! +-------------------+                        |  PersistentStorage   |
//!                                              +----------------------+
//! ```
//!
//! ## Security Features
//!
//! - **Mutual TLS (mTLS)**: Both parties authenticate using certificates
//! - **Server-side Safety Rules**: All safety validation happens on the signer
//! - **Key Isolation**: Private keys never leave the remote signer process
//!
//! ## Modules
//!
//! - [`config`]: Server configuration structures for the remote signer
//! - [`service`]: gRPC service implementation wrapping SafetyRules
//!
//! ## Usage
//!
//! The remote signer is typically run as a standalone binary:
//!
//! ```bash
//! aptos-remote-signer --config /path/to/config.yaml
//! ```
//!
//! See the crate README for detailed configuration examples.

pub mod config;
pub mod service;

pub use config::RemoteSignerServerConfig;
pub use service::SafetyRulesServiceImpl;
