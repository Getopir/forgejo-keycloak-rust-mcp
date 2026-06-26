# Features

`1.1.0` is the PR workflow and discovery release of the Forgejo Keycloak Rust MCP gateway.

## Identity And Policy

- Validates Keycloak bearer tokens with issuer, audience, expiry, and JWKS checks.
- Publishes OAuth protected-resource metadata for MCP clients.
- Uses an explicit policy registry for every exposed MCP operation.
- Exposes unauthenticated `/capabilities` metadata for operation discovery.
- Records audit events without bearer tokens or downstream service credentials.
- Rejects caller-supplied trusted identity headers.

## Forgejo Principal Mapping

- Maps immutable Keycloak `(issuer, subject)` identity to a Forgejo principal.
- Rejects unknown, disabled, duplicate, or malformed mappings.
- Reads mapped Forgejo API tokens from runtime environment variables only.
- Builds trusted reverse-proxy headers from the server-side mapping when deployments use Forgejo trusted-header auth.

## Curated MCP Tools

- `gateway_probe`
- `list_repository_metadata`
- `list_repository_issues`
- `create_issue_comment`
- `list_pull_requests`
- `create_pull_request`
- `list_pull_request_reviews`
- `list_releases`
- `list_notifications`
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

`create_pull_request`, `merge_pull_request`, and `create_release` are executable after approval and Forgejo ACL checks. Destructive and admin execution remains disabled.

## Generated API Coverage

The gateway pins the Forgejo `15.0.3+gitea-1.22.0` Swagger document and classifies all 491 operations by target type, risk, approval requirement, and exposure.

Generated coverage is metadata-only unless an endpoint has a reviewed semantic MCP operation. In `1.1.0`, 10 operations are exposed through the semantic overlay and 481 remain disabled.

## CLI

`forgejo-mcpctl` wraps the MCP endpoint for shell-based operators and agents while reading the bearer token from an environment variable.

Examples:

```sh
forgejo-mcpctl repository-metadata GetOpir/forgejo-keycloak-rust-mcp
forgejo-mcpctl repository-issues GetOpir/forgejo-keycloak-rust-mcp --state open --limit 25
forgejo-mcpctl api-coverage --filter semantic_overlay --limit 25
forgejo-mcpctl create-pull-request GetOpir/forgejo-keycloak-rust-mcp --head feature-branch --base main --title "Add feature" --dry-run
forgejo-mcpctl merge-pull-request GetOpir/forgejo-keycloak-rust-mcp#12 --method squash --dry-run
```
