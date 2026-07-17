# Features

The completed `1.x` line is the compatibility line for Forgejo versions before
`16.0.0`. The current `2.x` line targets Forgejo `16.0.0` only and enables one
reviewed semantic operation per later minor release. See the
[Forgejo 16 Release Plan](forgejo-16-release-plan.md).

When a Forgejo URL is configured, `2.1.0` verifies `/api/v1/version` before
listening and exposes the required and verified versions in `/health`.

## Identity And Policy

- Validates Keycloak bearer tokens with issuer, audience, expiry, and JWKS checks.
- Publishes OAuth protected-resource metadata for MCP clients.
- Provides `--tls` and `--ssl` setup guards for HTTPS public Forgejo and MCP URLs.
- Uses an explicit policy registry for every exposed MCP operation.
- Exposes unauthenticated `/capabilities` metadata for operation discovery.
- Records audit events without bearer tokens or downstream service credentials.
- Rejects caller-supplied trusted identity headers.

## Forgejo Principal Mapping

- Maps immutable Keycloak `(issuer, subject)` identity to a Forgejo principal.
- Rejects unknown, disabled, duplicate, or malformed mappings.
- Reads mapped Forgejo API tokens from runtime environment variables only.
- Builds trusted reverse-proxy headers from the server-side mapping when deployments use Forgejo trusted-header auth.

## Per-Agent Admission Control

Enabled mapped agents use bounded in-memory token buckets keyed by normalized Keycloak `(issuer, subject)`. Capacity, refill window, and tracked-agent bounds are configurable; exhausted buckets return HTTP `429` with `Retry-After` and produce a denied audit record. Proxy-level limits remain required for aggregate and non-agent traffic.

## Curated MCP Tools

- `gateway_probe`
- `list_repository_metadata`
- `get_branch_status`
- `list_repository_issues`
- `create_issue`
- `create_issue_comment`
- `list_pull_requests`
- `create_pull_request`
- `list_pull_request_reviews`
- `submit_pull_request_review`
- `get_pull_request_diff`
- `list_releases`
- `list_notifications`
- `list_wiki_pages`
- `get_wiki_page`
- `create_wiki_page`
- `update_wiki_page`
- `credential_reference_status`
- `forgejo_api_coverage`
- `create_approval`
- `create_release`
- `merge_pull_request`

List tools use server-capped `limit` and page-token `cursor` values. Summaries return stable `forgejo://...` resource URIs.

## Approval Gates

High-risk operations use file-backed, single-use approval records:

- approval IDs are UUIDs;
- approvals expire;
- approvals bind operation, target, state, body hash, Keycloak identity, OAuth client, and mapped Forgejo principal;
- executor and approver must be different mapped principals;
- approvals are consumed before Forgejo mutation calls.

`create_pull_request`, `merge_pull_request`, `create_release`, `create_wiki_page`, and `update_wiki_page` are executable after approval and Forgejo ACL checks. Successful `create_pull_request` responses return a normalized PR directly at `result.pull_request` and a richer `result.readback`; sparse Forgejo create responses trigger authoritative base/head or open-PR readback. Merge reads back the PR by number, reports exact failing check contexts, and closes open no-diff PRs as stale with a comment. Destructive and admin execution remains disabled.

## Generated API Coverage

The gateway pins the Forgejo `16.0.0` Swagger document and classifies all 506 operations by target type, risk, approval requirement, and exposure. The 15 operations added since the `15.0.3` pin remain disabled metadata until separately reviewed for MCP exposure.

`get_branch_status` accepts `owner/repository@branch` or a stable
`forgejo://branch/owner/repository/branch` URI. It returns one branch, at most
50 required contexts, and at most 50 commit statuses. Downstream branch and
status documents are capped at 64 KiB and 256 KiB respectively and use the
configured Forgejo request timeout.

Generated coverage is metadata-only unless an endpoint has a reviewed semantic MCP operation. The current semantic overlay covers 20 reviewed endpoints and leaves 486 metadata-only endpoints disabled.

## CLI

`forgejo-mcpctl` wraps the MCP endpoint for shell-based operators and agents while reading the bearer token from an environment variable.

Examples:

```sh
forgejo-mcpctl repository-metadata GetOpir/forgejo-keycloak-rust-mcp
forgejo-mcpctl branch-status GetOpir/forgejo-keycloak-rust-mcp@main
forgejo-mcpctl repository-issues GetOpir/forgejo-keycloak-rust-mcp --state open --limit 25
forgejo-mcpctl create-issue GetOpir/forgejo-keycloak-rust-mcp --title "Repair MCP adapter coverage"
forgejo-mcpctl api-coverage --filter semantic_overlay --limit 25
forgejo-mcpctl wiki-pages GetOpir/forgejo-keycloak-rust-mcp --limit 25
forgejo-mcpctl create-pull-request GetOpir/forgejo-keycloak-rust-mcp --head feature-branch --base main --title "Add feature" --dry-run
forgejo-mcpctl merge-pull-request GetOpir/forgejo-keycloak-rust-mcp#12 --method squash --dry-run
```
