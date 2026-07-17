# Install

Install from crates.io after publication:

```sh
cargo install forgejo-keycloak-rust-mcp --locked
```

This installs:

```text
forgejo-keycloak-rust-mcpd
forgejo-mcpctl
```

Build:

```sh
cargo test --workspace
cargo build --release -p forgejo-keycloak-rust-mcp
cargo build --release -p forgejo-keycloak-rust-mcp --bin forgejo-mcpctl
```

Run:

```sh
forgejo-keycloak-rust-mcpd \
  --issuer https://keycloak.example.org/realms/forgejo-agents \
  --discovery-url https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration \
  --audience forgejo-mcp \
  --resource https://mcp.example.org/mcp \
  --tls \
  --bind 127.0.0.1:7080 \
  --forgejo-url https://forgejo.example.org \
  --principal-map /etc/forgejo-mcpd/principals.json \
  --max-page-limit 50
```

Warning: for an HTTPS Forgejo deployment, set both the MCP public resource and
Forgejo base URL to `https://` and add `--tls` or `--ssl`. The flag makes setup
fail if `--resource` or `--forgejo-url` is accidentally left as `http://`.
Using a plain local bind such as `127.0.0.1:7080` is still valid behind a trusted
TLS reverse proxy.

HTTPS Forgejo setup command:

```sh
forgejo-keycloak-rust-mcpd \
  --issuer https://keycloak.example.org/realms/forgejo-agents \
  --discovery-url https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration \
  --audience forgejo-mcp \
  --resource https://forgejo.example.org/mcp \
  --ssl \
  --forgejo-url https://forgejo.example.org \
  --principal-map /etc/forgejo-mcpd/principals.json \
  --approval-store /var/lib/forgejo-mcpd/approvals.jsonl \
  --bind 127.0.0.1:7080
```

Keep Forgejo token values in runtime environment variables named by the principal map. Do not store token values in the map or in source control.

## Upgrade And Rollback

Before upgrading from `1.3.1`, upgrade Forgejo to `16.0.0` and verify its
`/api/v1/version` response. Back up the gateway binary, service configuration,
approval store, and audit log, install `2.1.0`, restart, and confirm `/health`
reports the required and verified Forgejo versions. Then verify capabilities,
an expected denial, and an authorized read.

There is no state or configuration migration. To roll back, restore the
`1.3.1` executable or run:

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.3.1 --locked --force
```

Restart and repeat the health, capability, denial, and authorized-read checks.
Rolling back the gateway does not roll back Forgejo.

CLI wrapper:

```sh
export FORGEJO_MCPCTL_GATEWAY=http://127.0.0.1:7080/mcp
export FORGEJO_MCPCTL_TOKEN_ENV=ACCESS_JWT
export ACCESS_JWT="<keycloak-access-token-from-your-token-broker>"
forgejo-mcpctl repository-metadata forgejo://repository/GetOpir/forgejo-keycloak-rust-mcp
```
