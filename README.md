# forgejo-keycloak-rust-mcp

[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/13642/badge)](https://www.bestpractices.dev/projects/13642)


Clean-room Rust MCP gateway for Forgejo with Keycloak identity and Forgejo ACL enforcement.

> **Canonical repository and contribution notice**
>
> This project is maintained on
> [Codeberg](https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp). The
> [GitHub repository](https://github.com/Getopir/forgejo-keycloak-rust-mcp) is
> a read-only mirror and may temporarily lag behind Codeberg. GitHub issues,
> pull requests, reviews, discussions, and other contributions are not
> monitored or accepted. Submit all issues and contributions on Codeberg.

## Version Compatibility

| Release line | Supported Forgejo contract |
| --- | --- |
| `1.x` through `1.3.1` | Forgejo versions before `16.0.0` |
| `2.x` starting with `2.0.0` | Forgejo `16.0.0` only |

Version `2.1.0` verifies the configured Forgejo server through
`/api/v1/version` before listening. It requires an exact `16.0.0` semantic core
version and accepts Forgejo build metadata such as
`16.0.0+gitea-1.22.0`.

Each new semantic operation receives one `2.x.0` minor release. Compatible
repairs increment the patch component, such as `2.2.1`. See the
[Forgejo 16 Release Plan](docs/forgejo-16-release-plan.md).

The governing rule is:

> Keycloak authenticates. The Rust gateway authorizes the operation class. Forgejo authorizes access to the actual repository or organization.

This project does not copy or translate GPL implementation code from other Forgejo MCP projects. Existing tools may be used as behavior checklists only.

## Project Status

- Shipped: identity and policy probe endpoints for validating agent bearer-token context.
- Shipped: principal mapping with duplicate-map validation, token-env validation, and trusted-header spoof rejection.
- Shipped: curated bounded Forgejo tools, typed resource URIs, and CLI wrappers for common agent/operator calls.
- Shipped: file-backed approval gates with exact payload and principal binding.
- Shipped: single-use approval-backed pull-request creation, pull-request merges, and release creation with dry-run preview.
- Shipped: bounded per-agent token-bucket rate limiting with `429` retry guidance and denied audit records.
- Shipped: unauthenticated capability discovery for agents and operators.
- Shipped: generated Forgejo API classification coverage pinned to the Forgejo `16.0.0` Swagger document.
- Shipped: bounded `get_branch_status` readback for one typed branch target and at most 50 commit-status summaries.
- Remaining: standalone PR update, standalone reviewer-request, required-check, PR-check, generic generated endpoint execution, admin execution, destructive execution, release deletion, release replacement, and release asset upload remain intentionally disabled.

## Current Scope

`2.1.0` adds the first post-baseline Forgejo 16 semantic operation. The stable
release line includes:

- Validates Keycloak-issued bearer tokens with issuer, audience, expiry, and JWKS checks.
- Serves OAuth protected-resource metadata for MCP clients.
- Adds `--tls` and `--ssl` HTTPS setup guards for deployments where Forgejo or the MCP public route is served over HTTPS.
- Provides a deterministic operation policy registry.
- Emits structured audit records without tokens or secret values.
- Exposes an authenticated `/mcp` policy probe for agents.
- Maps Keycloak `(issuer, subject)` principals to Forgejo accounts from an explicit local mapping file.
- Executes read-only repository metadata lookup through Forgejo API using the mapped principal's configured token environment variable.
- Reads a typed `owner/repository@branch` target with bounded branch, commit, protection, required-context, and combined-status summaries through the mapped principal.
- Builds trusted reverse-proxy identity headers from the mapped principal for deployments that use Forgejo reverse-proxy authentication.
- Rejects duplicate or malformed principal-map entries and caller-supplied trusted identity headers.
- Lists bounded issue, pull-request, pull-request review, release, and notification summaries, and submits evidence-backed PR reviews as the mapped reviewer identity.
- Reads bounded pull-request metadata, changed-file summaries, and diff text for independent review without exposing a Forgejo token to the caller.
- Creates additive issue or pull-request comments through the mapped Forgejo principal.
- Creates approval-backed pull requests and returns a normalized PR directly at `result.pull_request`.
- Persists authoritative PR readback at `result.readback`, including PR number, head SHA, state, merged state, merge commit SHA, branch-ref existence, combined check state, and stale classification.
- Falls back to authoritative base/head or open-PR readback when Forgejo returns a sparse PR creation response.
- Closes open no-diff stale PRs with a comment instead of reporting them as unfinished work.
- Reports exact failing check contexts and URLs before merge when required status checks are not green.
- Creates pull requests through the mapped Forgejo principal after exact-payload approval, with optional assignee and reviewer request inputs.
- Creates bounded Forgejo issues through the mapped principal with `forgejo:issue:write`.
- Lists and reads bounded wiki page metadata, and creates or updates wiki pages after exact-payload approval.
- Reports mapped credential-reference presence without returning downstream token or secret values.
- Enforces server-capped `limit` and page-token `cursor` handling for list operations.
- Returns stable `forgejo://...` resource URIs in repository, issue, pull-request, review, release, notification, comment, and wiki summaries.
- Adds `forgejo-mcpctl` as a token-env based CLI wrapper for curated MCP calls.
- Adds short-lived, file-backed approval records for high-risk operation gates.
- Rejects forged, expired, mismatched, or wrong-principal approval IDs.
- Consumes approval records before execution so they cannot be replayed.
- Applies bounded in-memory token buckets to enabled mapped agents keyed by immutable Keycloak issuer and subject.
- Requires approver and executor to be different mapped principals.
- Executes `merge_pull_request` only after a valid approval and Forgejo ACL check.
- Executes `create_release` only after a valid approval and Forgejo ACL check.
- Executes `create_pull_request` only after a valid approval and Forgejo ACL check.
- Exposes `GET /capabilities` for operation names, scopes, risk classes, approval requirements, and planned-but-disabled PR workflow operations.
- Provides dry-run merge previews that do not mutate Forgejo.
- Provides dry-run release previews that do not mutate Forgejo.
- Pins the Forgejo `16.0.0` Swagger document under `vendor/forgejo-api` and records the reviewed endpoint delta.
- Classifies all 506 pinned Forgejo API operations by target type, risk, approval requirement, and exposure.
- Exposes `forgejo_api_coverage` as a bounded metadata-only MCP operation.
- Adds `forgejo-mcpctl api-coverage` for operator and agent readback.
- Keeps every non-reviewed generated endpoint disabled until a semantic overlay is reviewed.
- Fails startup before binding when a configured Forgejo server is unavailable
  or does not report the required `16.0.0` core version.
- Reports the required and verified Forgejo versions from `GET /health`.

High-risk Forgejo mutations such as deletion and admin actions remain disabled. Pull-request creation, pull-request merge, release creation, and wiki publication are the reviewed approval-backed high-risk execution paths.

## Install

Prerequisites:

- Rust `1.95` or newer.
- A reachable Keycloak realm with OIDC discovery enabled.
- A Keycloak client or audience value for this MCP resource.

Build and test:

```sh
cargo test --workspace
cargo build --release -p forgejo-keycloak-rust-mcp
cargo build --release -p forgejo-keycloak-rust-mcp --bin forgejo-mcpctl
```

Install from crates.io after publication:

```sh
cargo install forgejo-keycloak-rust-mcp --locked
```

Run:

```sh
forgejo-keycloak-rust-mcpd \
  --issuer https://keycloak.example.org/realms/forgejo-agents \
  --discovery-url https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration \
  --audience forgejo-mcp \
  --resource https://mcp.example.org/mcp \
  --tls \
  --forgejo-url https://forgejo.example.org \
  --approval-store /var/lib/forgejo-mcpd/approvals.jsonl \
  --bind 127.0.0.1:7080
```

Warning: if your public Forgejo or MCP route is HTTPS, configure the public URLs
with `https://` and add `--tls` or `--ssl`. The flag makes the daemon fail fast
if `--resource` or `--forgejo-url` is accidentally left as `http://`. The local
bind address can still be plain HTTP when a trusted reverse proxy terminates TLS.

The same settings can be supplied through environment variables:

```sh
export FORGEJO_MCPD_ISSUER=https://keycloak.example.org/realms/forgejo-agents
export FORGEJO_MCPD_DISCOVERY_URL=https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration
export FORGEJO_MCPD_AUDIENCE=forgejo-mcp
export FORGEJO_MCPD_RESOURCE=https://mcp.example.org/mcp
export FORGEJO_MCPD_TLS=true
export FORGEJO_MCPD_FORGEJO_URL=https://forgejo.example.org
export FORGEJO_MCPD_FORGEJO_CONNECT_TIMEOUT_SECONDS=5
export FORGEJO_MCPD_FORGEJO_REQUEST_TIMEOUT_SECONDS=30
export FORGEJO_MCPD_BIND=127.0.0.1:7080
export FORGEJO_MCPD_APPROVAL_STORE=/var/lib/forgejo-mcpd/approvals.jsonl
forgejo-keycloak-rust-mcpd
```

## Endpoints

- `GET /health`
- `GET /.well-known/oauth-protected-resource`
- `GET /.well-known/oauth-protected-resource/mcp`
- `GET /capabilities`
- `POST /mcp`

`POST /mcp` requires `Authorization: Bearer <keycloak access token>`.

Example:

```sh
curl -sS http://127.0.0.1:7080/health

# Obtain this from your deployment's short-lived Keycloak token broker.
ACCESS_JWT="<keycloak-access-token-for-the-mcp-audience>"

curl -sS \
  -H "Authorization: Bearer ${ACCESS_JWT}" \
  -H "Content-Type: application/json" \
  -d '{"operation":"gateway_probe","target":"owner/repository"}' \
  http://127.0.0.1:7080/mcp
```

## Documentation

- [Contributing](CONTRIBUTING.md)
- [Coding Agent Guide](AGENTS.md)
- [Install Guide](docs/install.md)
- [Configuration](docs/configuration.md)
- [Features](docs/features.md)
- [MCP Functions](docs/mcp-functions.md)
- [Generated Forgejo API Coverage](docs/generated/forgejo-api-coverage.md)
- [Forgejo 16 Release Plan](docs/forgejo-16-release-plan.md)
- [Agent Setup](docs/agent-setup.md)
- [Testing](docs/testing.md)
- [Security Checks](docs/security-checks.md)
- [Threat Model](docs/threat-model.md)
- [JWKS Cache Limits And Key Rotation](docs/jwks-cache-and-key-rotation.md)
- [Release Artifact Verification](docs/release-verification.md)
- [Credential Rotation and Incident Response](docs/credential-rotation-and-incident-response.md)
- [Codeberg Publishing](docs/codeberg-publishing.md)
- [Crates.io Publishing](docs/crates-io-publishing.md)
- [Promotion Checklist](docs/promotion/README.md)
- [Release Notes 2.1.0](docs/release-notes/2.1.0.md)
- [Release Notes 1.2.11](docs/release-notes/1.2.11.md)
- [Release Notes 1.2.4](docs/release-notes/1.2.4.md)
- [Release Notes 1.2.3](docs/release-notes/1.2.3.md)
- [Release Notes 1.2.2](docs/release-notes/1.2.2.md)
- [Release Notes 1.2.1](docs/release-notes/1.2.1.md)
- [Release Notes 1.1.4](docs/release-notes/1.1.4.md)
- [Release Notes 1.1.3](docs/release-notes/1.1.3.md)
- [Release Notes 1.1.2](docs/release-notes/1.1.2.md)
- [Release Notes 1.1.0](docs/release-notes/1.1.0.md)
- [Release Notes 1.0.2](docs/release-notes/1.0.2.md)
- [Release Notes 1.0.1](docs/release-notes/1.0.1.md)
- [Release Notes 1.0.0](docs/release-notes/1.0.0.md)
- [Release Notes 0.10.0](docs/release-notes/0.10.0.md)
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
- `crates/forgejo-mcpd`: publishable `forgejo-keycloak-rust-mcp` binary package, HTTP daemon, principal mapping, Forgejo client, curated MCP tools, and MCP probe.
- `vendor/forgejo-api`: pinned Forgejo Swagger document and provenance.
- `tools/generate_forgejo_api_catalog.py`: reproducible generated API coverage report.
- `openspec/changes/forgejo-keycloak-rust-mcp`: intended behavior and acceptance criteria.
- `docs/wiki`: Markdown wiki fallback for public Forgejo/Codeberg hosting.

## Security

Do not put Keycloak client secrets, Forgejo tokens, private keys, or bearer tokens in configuration files committed to this repository. Use runtime environment variables, secret managers, or deployment-specific files outside source control.

See [SECURITY.md](SECURITY.md) before reporting vulnerabilities or sharing logs.

## License

This project is licensed under `AGPL-3.0-or-later`. See [LICENSE](LICENSE) and the full text in [LICENSES/AGPL-3.0-or-later.txt](LICENSES/AGPL-3.0-or-later.txt).

[![Get it on Codeberg](https://get-it-on.codeberg.org/get-it-on-blue-on-white.svg)](https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp)
