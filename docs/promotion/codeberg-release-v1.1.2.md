# Codeberg Release Text - v1.1.2

Title: `v1.1.2 - HTTPS Forgejo setup guard`

`v1.1.2` adds an explicit HTTPS setup guard for deployments where Forgejo or the
MCP public route is served over HTTPS.

## Highlights

- Adds `--tls` with `--ssl` as an alias on `forgejo-keycloak-rust-mcpd`.
- Fails fast when `--tls` is enabled and `--resource` or `--forgejo-url` uses
  `http://`.
- Documents the HTTPS Forgejo setup command in the repository docs, wiki
  fallback, and crates.io package README.

## Install

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.1.2 --locked
```

## HTTPS Forgejo Setup

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

## Safety

The flag validates public URL configuration. It does not make the daemon
terminate TLS directly; use a trusted reverse proxy or TLS layer for that.
