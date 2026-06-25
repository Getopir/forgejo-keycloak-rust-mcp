# forgejo-keycloak-rust-mcp

Clean-room Rust MCP gateway for Forgejo with Keycloak identity and Forgejo ACL enforcement.

The governing rule for this repository is:

> Keycloak authenticates. The Rust gateway authorizes the operation class. Forgejo authorizes access to the actual repository or organization.

This repository intentionally starts with the identity chain before broad API coverage. Sqcows and goern are feature references only; GPL source is not copied or translated.

## Phase 0 scope

- Validate Keycloak-issued bearer tokens for the MCP resource audience.
- Serve OAuth protected-resource metadata for MCP clients.
- Keep a deterministic operation policy registry.
- Emit structured audit event records without token or secret values.
- Provide a small authenticated `/mcp` probe for the VM lab.

Broad Forgejo coverage, trusted-header delegation, approvals, generated endpoint manifests, and admin tools are tracked in the OpenSpec change and OPIR-O project.

## Run locally

```powershell
cargo test --workspace
cargo run -p forgejo-mcpd -- `
  --issuer http://keycloak:8080/realms/master `
  --discovery-url http://192.168.87.63:8080/realms/master/.well-known/openid-configuration `
  --audience http://192.168.87.190/mcp `
  --resource http://192.168.87.190/mcp `
  --bind 127.0.0.1:7080
```

The daemon exposes:

- `GET /health`
- `GET /.well-known/oauth-protected-resource`
- `GET /.well-known/oauth-protected-resource/mcp`
- `POST /mcp`

`POST /mcp` requires `Authorization: Bearer <keycloak access token>`.

## Repository layout

- `crates/identity`: Keycloak OIDC discovery, JWKS fetch, JWT claim and audience validation.
- `crates/policy`: operation registry, risk classes, scope checks, approval requirements.
- `crates/audit`: structured audit event schema.
- `crates/forgejo-mcpd`: HTTP daemon and Phase 0 MCP probe.
- `openspec/changes/forgejo-keycloak-rust-mcp`: intended behavior and acceptance criteria.
- `opir/projects`: OPIR-O project plan and repeatable project creation script.
- `deploy/lab`: VM lab deployment notes and scripts.
