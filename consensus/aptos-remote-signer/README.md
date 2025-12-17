# Aptos Remote Signer

A standalone gRPC service that holds validator private keys and performs consensus signing operations for Aptos validators.

## Overview

The remote signer allows validators to decouple their signing functionality from the main validator node, improving security by isolating key material in a separate process that can run on dedicated hardware or in a hardened environment.

### Architecture

```
+-------------------+       gRPC/mTLS        +----------------------+
|  Validator Node   |  ------------------>   |  aptos-remote-signer |
|                   |  <------------------   |                      |
|  RemoteSignerClient                        |  SafetyRules         |
|  (TSafetyRules)   |                        |  ValidatorSigner     |
+-------------------+                        |  PersistentStorage   |
                                             +----------------------+
```

### Security Model

- **Mutual TLS (mTLS)**: Both the validator node and remote signer authenticate each other using TLS certificates
- **Server-side Safety Rules**: All consensus safety rule validation is performed on the remote signer, preventing signing of conflicting blocks even if the validator node is compromised
- **Key Isolation**: Private keys never leave the remote signer process

## Usage

### Starting the Remote Signer

```bash
aptos-remote-signer --config /path/to/config.yaml
```

### Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `-c, --config` | Path to configuration file | Required |
| `--log-level` | Log level (error, warn, info, debug, trace) | info |

## Configuration

### Server Configuration (config.yaml)

```yaml
# Address to listen on
listen_address: "0.0.0.0:8443"

# TLS configuration (required for production)
tls:
  cert_path: "/etc/signer/certs/server.pem"
  key_path: "/etc/signer/certs/server-key.pem"
  # Client CA for mTLS (optional but recommended)
  client_ca_cert_path: "/etc/signer/certs/client-ca.pem"

# Secure storage backend
backend:
  type: on_disk
  path: "/etc/signer/secure_storage.json"
  # Or use Vault:
  # type: vault
  # server: "https://vault.example.com:8200"
  # token: "s.xxxxx"

# Initial safety rules configuration
initial_safety_rules_config:
  from_file:
    identity_blob_path: "/etc/signer/identity.yaml"
    waypoint:
      from_config: "0:abc123..."
    # Optional: additional keys for rotation
    overriding_identity_paths: []

# Cache safety data for performance (default: true)
enable_cached_safety_data: true
```

### Validator Node Configuration

Configure the validator to use the remote signer in the node configuration:

```yaml
consensus:
  safety_rules:
    service:
      type: remote_signer
      server_address: "https://signer.internal:8443"
      tls_config:
        ca_cert_path: "/etc/aptos/certs/ca.pem"
        client_cert_path: "/etc/aptos/certs/validator.pem"
        client_key_path: "/etc/aptos/certs/validator-key.pem"
      connect_timeout_ms: 5000
      request_timeout_ms: 10000
      max_retries: 3
      initial_backoff_ms: 100
```

### Insecure Mode (Testing Only)

For local development and testing, you can disable TLS:

**Server config:**
```yaml
listen_address: "0.0.0.0:8080"
# Omit tls section entirely
backend:
  type: on_disk
  path: "/tmp/safety_storage.json"
initial_safety_rules_config:
  from_file:
    identity_blob_path: "/path/to/identity.yaml"
    waypoint:
      from_config: "0:..."
```

**Validator config:**
```yaml
consensus:
  safety_rules:
    service:
      type: remote_signer
      server_address: "http://localhost:8080"
      allow_insecure: true
```

**WARNING**: Never use insecure mode in production. It disables all transport security and exposes signing operations to network attacks.

## Signing Operations

The remote signer handles all consensus signing operations:

| Operation | Description |
|-----------|-------------|
| `consensus_state` | Query current consensus state (epoch, rounds, waypoint) |
| `initialize` | Initialize safety rules for a new epoch |
| `sign_proposal` | Sign a block proposal |
| `sign_timeout_with_qc` | Sign a timeout message |
| `construct_and_sign_vote_two_chain` | Construct and sign a vote |
| `construct_and_sign_order_vote` | Sign an order vote |
| `sign_commit_vote` | Sign a commit vote |

## Deployment Considerations

### Network Security

- Deploy the remote signer in a private network segment, not accessible from the public internet
- Use firewall rules to restrict access to only authorized validator nodes
- Consider using a service mesh for additional security controls

### High Availability

- The remote signer maintains state in the configured storage backend
- For HA deployments, use a shared storage backend (e.g., Vault) that can be accessed by multiple instances
- Only one instance should be active at a time to prevent conflicting signatures

### Key Management

- Generate the identity blob securely and transfer it to the remote signer through a secure channel
- Consider using a Hardware Security Module (HSM) through Vault for key storage
- Implement key rotation procedures using the `overriding_identity_paths` configuration

### Monitoring

- Monitor the remote signer logs for signing failures or connectivity issues
- Set up alerts for high error rates or latency
- Track metrics for signing operations latency and success rates

## Building

```bash
cargo build -p aptos-remote-signer --release
```

The binary will be available at `target/release/aptos-remote-signer`.

## Testing

```bash
# Run unit tests
cargo test -p aptos-remote-signer

# Run client tests
cargo test -p aptos-safety-rules remote_signer_client
```

## Troubleshooting

### Connection Refused

- Verify the remote signer is running and listening on the configured address
- Check firewall rules allow traffic on the configured port
- Ensure the server address in the validator config matches the remote signer's listen address

### TLS Handshake Failures

- Verify certificates are valid and not expired
- Ensure the CA certificate chain is complete
- Check that the server certificate's Subject Alternative Name (SAN) matches the hostname used to connect

### Signing Failures

- Check the remote signer logs for safety rule violations
- Verify the epoch and waypoint are correctly initialized
- Ensure the identity blob contains the correct consensus key

## License

Apache-2.0
