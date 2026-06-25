# forgejo-keycloak-rust-mcp

Clean-room Rust MCP gateway for Forgejo with Keycloak identity and Forgejo ACL enforcement.

Version `0.4.0` means:

- `0`: pre-1.0 official release line.
- `4`: beta series 4.
- `0`: baseline release for that beta series.

The governing rule is:

> Keycloak authenticates. The Rust gateway authorizes the operation class. Forgejo authorizes access to the actual repository or organization.

This project does not copy or translate GPL implementation code from other Forgejo MCP projects. Existing tools may be used as behavior checklists only.

## Current Scope

`0.4.0` is a Phase 0 gateway release:

- Validates Keycloak-issued bearer tokens with issuer, audience, expiry, and JWKS checks.
- Serves OAuth protected-resource metadata for MCP clients.
- Provides a deterministic operation policy registry.
- Emits structured audit records without tokens or secret values.
- Exposes an authenticated `/mcp` policy probe for agents.

Full Forgejo API execution is intentionally not enabled yet. The current `/mcp` handler proves identity and policy decisions before Forgejo delegation is added.

## Install

Prerequisites:

- Rust `1.95` or newer.
- A reachable Keycloak realm with OIDC discovery enabled.
- A Keycloak client or audience value for this MCP resource.

Build and test:

```sh
cargo test --workspace
cargo build --release -p forgejo-mcpd
```

Run:

```sh
forgejo-mcpd \
  --issuer https://keycloak.example.org/realms/forgejo-agents \
  --discovery-url https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration \
  --audience forgejo-mcp \
  --resource https://mcp.example.org/mcp \
  --bind 127.0.0.1:7080
```

The same settings can be supplied through environment variables:

```sh
export FORGEJO_MCPD_ISSUER=https://keycloak.example.org/realms/forgejo-agents
export FORGEJO_MCPD_DISCOVERY_URL=https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration
export FORGEJO_MCPD_AUDIENCE=forgejo-mcp
export FORGEJO_MCPD_RESOURCE=https://mcp.example.org/mcp
export FORGEJO_MCPD_BIND=127.0.0.1:7080
forgejo-mcpd
```

## Endpoints

- `GET /health`
- `GET /.well-known/oauth-protected-resource`
- `GET /.well-known/oauth-protected-resource/mcp`
- `POST /mcp`

`POST /mcp` requires `Authorization: Bearer <keycloak access token>`.

Example:

```sh
curl -sS http://127.0.0.1:7080/health

ACCESS_JWT="$(get-agent-token)"

curl -sS \
  -H "Authorization: Bearer ${ACCESS_JWT}" \
  -H "Content-Type: application/json" \
  -d '{"operation":"gateway_probe","target":"owner/repository"}' \
  http://127.0.0.1:7080/mcp
```

## Documentation

- [Install Guide](docs/install.md)
- [Configuration](docs/configuration.md)
- [MCP Functions](docs/mcp-functions.md)
- [Agent Setup](docs/agent-setup.md)
- [Testing](docs/testing.md)
- [Release Notes 0.4.0](docs/release-notes/0.4.0.md)
- [Lab Deployment](deploy/lab/README.md)
- [Wiki Fallback](docs/wiki/Home.md)

## Repository Layout

- `crates/identity`: Keycloak OIDC discovery, JWKS fetch, JWT claim and audience validation.
- `crates/policy`: operation registry, risk classes, scope checks, approval requirements.
- `crates/audit`: structured audit event schema.
- `crates/forgejo-mcpd`: HTTP daemon and Phase 0 MCP probe.
- `openspec/changes/forgejo-keycloak-rust-mcp`: intended behavior and acceptance criteria.
- `docs/wiki`: Markdown wiki fallback for public Forgejo/Codeberg hosting.

## Security

Do not put Keycloak client secrets, Forgejo tokens, private keys, or bearer tokens in configuration files committed to this repository. Use runtime environment variables, secret managers, or deployment-specific files outside source control.
