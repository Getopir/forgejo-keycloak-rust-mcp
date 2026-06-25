# Live Readback - 2026-06-25

## Codeberg Release

Hosted Codeberg release object is present for `v0.10.0`.

- API readback: `https://codeberg.org/api/v1/repos/rawholding/forgejo-keycloak-rust-mcp/releases/tags/v0.10.0`
- HTML URL: `https://codeberg.org/rawholding/forgejo-keycloak-rust-mcp/releases/tag/v0.10.0`
- Tag: `v0.10.0`
- Title: `v0.10.0 - Approval-backed release creation`
- Draft: `false`
- Prerelease: `true`

The temporary Codeberg API token used for release creation was stored in OpenBao at:

```text
kv/data/prod/codeberg/rawholding/forgejo-keycloak-rust-mcp/release-token
```

Do not copy the token value into repository files or operational notes.

## Live Forgejo MCP Gateway

Target host:

- VMID: `118`
- Hostname: `svc-forgejo`
- IP: `192.168.87.91`
- Forgejo UI: `http://192.168.87.91:3000`
- Rust MCP gateway: `http://192.168.87.91:7080/mcp`

Installed binaries:

```text
/usr/local/bin/forgejo-keycloak-rust-mcpd
/usr/local/bin/forgejo-keycloak-rust-mcpctl
```

Systemd unit:

```text
forgejo-keycloak-rust-mcpd.service
```

Runtime configuration:

```text
/etc/forgejo-keycloak-rust-mcpd/env
/var/lib/forgejo-keycloak-rust-mcpd/approvals.jsonl
```

Readbacks:

- `GET http://192.168.87.91:7080/health` returned `{"service":"forgejo-mcpd","status":"ok"}`.
- `GET http://192.168.87.91:7080/.well-known/oauth-protected-resource` returned the MCP resource metadata.
- `POST http://192.168.87.91:7080/mcp` with valid JSON and no bearer token returned `401`, as expected.
- `systemctl is-active forgejo-keycloak-rust-mcpd.service` returned `active`.
- Listener readback showed `0.0.0.0:7080`.

Notes:

- The older Go `forgejo-mcp.service` remains active on port `8090`; it was not modified.
- The Rust gateway uses internal Keycloak HTTP discovery at `http://192.168.87.63:8080/realms/master/.well-known/openid-configuration` while preserving issuer `https://keycloak:8443/realms/master`.
- Principal mapping and mapped Forgejo principal tokens are still required before full authenticated Forgejo-backed MCP tool tests can be run.

