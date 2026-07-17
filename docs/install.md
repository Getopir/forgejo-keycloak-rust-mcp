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
git clone https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp.git
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
FORGEJO_MCPD_TLS=true
FORGEJO_MCPD_BIND=127.0.0.1:7080
```

Warning: when Forgejo or the MCP public route is HTTPS, the public URLs must use
`https://`. Add `--tls` or `--ssl` to the daemon command, or set
`FORGEJO_MCPD_TLS=true`, so setup fails instead of advertising or calling an
accidental `http://` URL. The daemon may still bind to `127.0.0.1:7080` over
plain HTTP when TLS is terminated by a local reverse proxy.

For Phase 1 and Phase 2 Forgejo-backed tools, add:

```sh
FORGEJO_MCPD_FORGEJO_URL=https://forgejo.example.org
FORGEJO_MCPD_PRINCIPAL_MAP=/etc/forgejo-mcpd/principals.json
FORGEJO_MCPD_MAX_PAGE_LIMIT=50
FORGEJO_AGENT_READER_TOKEN=...
```

Equivalent HTTPS Forgejo daemon command:

```sh
forgejo-keycloak-rust-mcpd \
  --issuer https://keycloak.example.org/realms/forgejo-agents \
  --discovery-url https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration \
  --audience forgejo-mcp \
  --resource https://forgejo.example.org/mcp \
  --tls \
  --forgejo-url https://forgejo.example.org \
  --principal-map /etc/forgejo-mcpd/principals.json \
  --approval-store /var/lib/forgejo-mcpd/approvals.jsonl \
  --bind 127.0.0.1:7080
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

## Upgrade From 1.3.1

1. Upgrade the Forgejo server to `16.0.0` and confirm
   `GET /api/v1/version` reports `16.0.0` with optional build metadata.
2. Back up the gateway executable, service unit, non-secret configuration,
   approval store, and audit log according to their retention policies.
3. Install `forgejo-keycloak-rust-mcp` `2.0.0` and restart the service.
4. Confirm `/health` reports `required_forgejo_version` as `16.0.0` and a
   non-null matching `verified_forgejo_version`.
5. Verify capability metadata, one expected denial, and one authorized
   read-only Forgejo operation before enabling normal traffic.

There is no configuration or state-file migration. The breaking change is the
Forgejo server compatibility boundary.

## Roll Back To 1.3.1

Stop the service, restore the backed-up `1.3.1` executable, and restart it with
the unchanged configuration and state files. For a crates.io installation:

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.3.1 --locked --force
```

Confirm health, capability metadata, an expected denial, and an authorized
read before restoring normal traffic. Rolling back the gateway does not roll
back Forgejo and does not require approval-store or audit-log conversion.

## CLI Wrapper

`forgejo-mcpctl` is optional. It reads the gateway URL from `FORGEJO_MCPCTL_GATEWAY` and reads the bearer token from the environment variable named by `FORGEJO_MCPCTL_TOKEN_ENV`.

```sh
export FORGEJO_MCPCTL_GATEWAY=http://127.0.0.1:7080/mcp
export FORGEJO_MCPCTL_TOKEN_ENV=ACCESS_JWT
export ACCESS_JWT="<keycloak-access-token-from-your-token-broker>"

forgejo-mcpctl repository-metadata forgejo://repository/GetOpir/forgejo-keycloak-rust-mcp
forgejo-mcpctl repository-issues GetOpir/forgejo-keycloak-rust-mcp --state open --limit 25
```
