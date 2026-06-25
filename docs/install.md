# Install Guide

## Build From Source

```sh
git clone https://codeberg.org/rawholding/forgejo-keycloak-rust-mcp.git
cd forgejo-keycloak-rust-mcp
cargo test --workspace
cargo build --release -p forgejo-mcpd
```

The release binary is:

```text
target/release/forgejo-mcpd
```

## Runtime Service

Create an unprivileged service user and run the daemon behind TLS or a trusted reverse proxy:

```sh
FORGEJO_MCPD_ISSUER=https://keycloak.example.org/realms/forgejo-agents
FORGEJO_MCPD_DISCOVERY_URL=https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration
FORGEJO_MCPD_AUDIENCE=forgejo-mcp
FORGEJO_MCPD_RESOURCE=https://mcp.example.org/mcp
FORGEJO_MCPD_BIND=127.0.0.1:7080
```

The daemon does not need a Forgejo token for Phase 0 because it only validates identity and operation policy.

## Systemd Example

```ini
[Unit]
Description=Forgejo Keycloak Rust MCP gateway
After=network-online.target
Wants=network-online.target

[Service]
User=forgejo-mcp
Group=forgejo-mcp
EnvironmentFile=/etc/forgejo-mcpd/forgejo-mcpd.env
ExecStart=/usr/local/bin/forgejo-mcpd
Restart=on-failure
RestartSec=5
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true

[Install]
WantedBy=multi-user.target
```

Keep `/etc/forgejo-mcpd/forgejo-mcpd.env` out of source control.
