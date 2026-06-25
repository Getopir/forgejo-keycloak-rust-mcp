# forgejo-keycloak-rust-mcp



Clean-room Rust MCP gateway for Forgejo with Keycloak identity and Forgejo ACL enforcement.

Version `0.9.0` means:

- `0`: pre-1.0 official release line.
- `9`: beta series 9.
- `0`: baseline release for beta series 9.

The governing rule is:

> Keycloak authenticates. The Rust gateway authorizes the operation class. Forgejo authorizes access to the actual repository or organization.

This project does not copy or translate GPL implementation code from other Forgejo MCP projects. Existing tools may be used as behavior checklists only.

## Project Status

- Current: Phase 0 identity and policy probe is complete.
- Current: Phase 1 principal mapping is hardened with duplicate-map validation, token-env validation, and trusted-header spoof rejection.
- Current: Phase 2 adds a curated bounded tool surface, typed resource URIs, and CLI wrappers for common agent/operator calls.
- Current: Phase 2 approval gates use a file-backed approval store with exact payload and principal binding.
- Current: Phase 2 supports single-use approval-backed pull-request merges with dry-run preview.
- Not yet: full issue, pull request, release, notification, admin, destructive, or generated Forgejo API coverage.

## Current Scope

`0.9.0` is a Phase 2 approval-backed merge release:

- Validates Keycloak-issued bearer tokens with issuer, audience, expiry, and JWKS checks.
- Serves OAuth protected-resource metadata for MCP clients.
- Provides a deterministic operation policy registry.
- Emits structured audit records without tokens or secret values.
- Exposes an authenticated `/mcp` policy probe for agents.
- Maps Keycloak `(issuer, subject)` principals to Forgejo accounts from an explicit local mapping file.
- Executes read-only repository metadata lookup through Forgejo API using the mapped principal's configured token environment variable.
- Builds trusted reverse-proxy identity headers from the mapped principal for deployments that use Forgejo reverse-proxy authentication.
- Rejects duplicate or malformed principal-map entries and caller-supplied trusted identity headers.
- Lists bounded issue, pull-request, pull-request review, release, and notification summaries.
- Creates additive issue or pull-request comments through the mapped Forgejo principal.
- Enforces server-capped `limit` and page-token `cursor` handling for list operations.
- Returns stable `forgejo://...` resource URIs in repository, issue, pull-request, review, release, notification, and comment summaries.
- Adds `forgejo-mcpctl` as a token-env based CLI wrapper for curated MCP calls.
- Adds short-lived, file-backed approval records for high-risk operation gates.
- Rejects forged, expired, mismatched, or wrong-principal approval IDs.
- Consumes approval records before execution so they cannot be replayed.
- Requires approver and executor to be different mapped principals.
- Executes `merge_pull_request` only after a valid approval and Forgejo ACL check.
- Provides dry-run merge previews that do not mutate Forgejo.

High-risk Forgejo mutations such as release publication, deletion, and admin actions remain disabled. Pull-request merge is the first approval-backed high-risk execution path.

## Install

Prerequisites:

- Rust `1.95` or newer.
- A reachable Keycloak realm with OIDC discovery enabled.
- A Keycloak client or audience value for this MCP resource.

Build and test:

```sh
cargo test --workspace
cargo build --release -p forgejo-mcpd
cargo build --release -p forgejo-mcpd --bin forgejo-mcpctl
```

Run:

```sh
forgejo-mcpd \
  --issuer https://keycloak.example.org/realms/forgejo-agents \
  --discovery-url https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration \
  --audience forgejo-mcp \
  --resource https://mcp.example.org/mcp \
  --approval-store /var/lib/forgejo-mcpd/approvals.jsonl \
  --bind 127.0.0.1:7080
```

The same settings can be supplied through environment variables:

```sh
export FORGEJO_MCPD_ISSUER=https://keycloak.example.org/realms/forgejo-agents
export FORGEJO_MCPD_DISCOVERY_URL=https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration
export FORGEJO_MCPD_AUDIENCE=forgejo-mcp
export FORGEJO_MCPD_RESOURCE=https://mcp.example.org/mcp
export FORGEJO_MCPD_BIND=127.0.0.1:7080
export FORGEJO_MCPD_APPROVAL_STORE=/var/lib/forgejo-mcpd/approvals.jsonl
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
- [Security Checks](docs/security-checks.md)
- [Codeberg Publishing](docs/codeberg-publishing.md)
- [Promotion Checklist](docs/promotion/README.md)
- [Release Notes 0.9.0](docs/release-notes/0.9.0.md)
- [Release Notes 0.8.0](docs/release-notes/0.8.0.md)
- [Release Notes 0.7.0](docs/release-notes/0.7.0.md)
- [Release Notes 0.6.0](docs/release-notes/0.6.0.md)
- [Release Notes 0.5.0](docs/release-notes/0.5.0.md)
- [Release Notes 0.4.2](docs/release-notes/0.4.2.md)
- [Release Notes 0.4.1](docs/release-notes/0.4.1.md)
- [Release Notes 0.4.0](docs/release-notes/0.4.0.md)
- [Lab Deployment](deploy/lab/README.md)
- [Wiki Fallback](docs/wiki/Home.md)

## Repository Layout

- `crates/identity`: Keycloak OIDC discovery, JWKS fetch, JWT claim and audience validation.
- `crates/policy`: operation registry, risk classes, scope checks, approval requirements.
- `crates/audit`: structured audit event schema.
- `crates/forgejo-mcpd`: HTTP daemon, principal mapping, Forgejo client, curated MCP tools, and MCP probe.
- `openspec/changes/forgejo-keycloak-rust-mcp`: intended behavior and acceptance criteria.
- `docs/wiki`: Markdown wiki fallback for public Forgejo/Codeberg hosting.

## Security

Do not put Keycloak client secrets, Forgejo tokens, private keys, or bearer tokens in configuration files committed to this repository. Use runtime environment variables, secret managers, or deployment-specific files outside source control.

See [SECURITY.md](SECURITY.md) before reporting vulnerabilities or sharing logs.

## License

This project is licensed under `AGPL-3.0-or-later`. See [LICENSE](LICENSE) and the full text in [LICENSES/AGPL-3.0-or-later.txt](LICENSES/AGPL-3.0-or-later.txt).

[![Get it on Codeberg](https://get-it-on.codeberg.org/get-it-on-blue-on-white.svg)](https://codeberg.org/rawholding/forgejo-keycloak-rust-mcp)
