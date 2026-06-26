# Install Guide

## Install From Crates.io

After the crates.io release is published, users can install both binaries with:

```sh
cargo install forgejo-keycloak-rust-mcp --locked
```

Installed binaries:

```text
forgejo-keycloak-rust-mcpd
forgejo-mcpctl
```

## Build From Source

```sh
git clone https://codeberg.org/rawholding/forgejo-keycloak-rust-mcp.git
cd forgejo-keycloak-rust-mcp
cargo test --workspace
cargo build --release -p forgejo-keycloak-rust-mcp
cargo build --release -p forgejo-keycloak-rust-mcp --bin forgejo-mcpctl
```

The release binary is:

```text
target/release/forgejo-keycloak-rust-mcpd
target/release/forgejo-mcpctl
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

For Phase 1 and Phase 2 Forgejo-backed tools, add:

```sh
FORGEJO_MCPD_FORGEJO_URL=https://forgejo.example.org
FORGEJO_MCPD_PRINCIPAL_MAP=/etc/forgejo-mcpd/principals.json
FORGEJO_MCPD_MAX_PAGE_LIMIT=50
FORGEJO_AGENT_READER_TOKEN=...
```

The principal map stores `api_token_env` names, not token values. Keep `/etc/forgejo-mcpd/forgejo-mcpd.env` and `/etc/forgejo-mcpd/principals.json` out of source control if they contain production identities or runtime token names. The `api_token_env` values must use ASCII letters, digits, and underscore only.

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
ExecStart=/usr/local/bin/forgejo-keycloak-rust-mcpd
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

## CLI Wrapper

`forgejo-mcpctl` is optional. It reads the gateway URL from `FORGEJO_MCPCTL_GATEWAY` and reads the bearer token from the environment variable named by `FORGEJO_MCPCTL_TOKEN_ENV`.

```sh
export FORGEJO_MCPCTL_GATEWAY=http://127.0.0.1:7080/mcp
export FORGEJO_MCPCTL_TOKEN_ENV=ACCESS_JWT
export ACCESS_JWT="$(get-agent-token)"

forgejo-mcpctl repository-metadata forgejo://repository/rawholding/forgejo-keycloak-rust-mcp
forgejo-mcpctl repository-issues rawholding/forgejo-keycloak-rust-mcp --state open --limit 25
```
