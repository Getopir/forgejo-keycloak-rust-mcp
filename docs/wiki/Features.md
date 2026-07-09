# Features

`1.2.0` is the PR/source-authority hygiene release of the Forgejo Keycloak Rust MCP gateway.

## Identity And Policy

- Keycloak bearer-token validation.
- OAuth protected-resource metadata.
- `--tls` and `--ssl` setup guards for HTTPS public Forgejo and MCP URLs.
- Unauthenticated `/capabilities` operation discovery.
- Explicit operation registry with required scope, risk class, and approval policy.
- Token-free audit events.
- Trusted-header spoof rejection.

## Forgejo Principal Mapping

- Explicit Keycloak `(issuer, subject)` to Forgejo account mapping.
- Disabled, unknown, duplicate, and malformed mapping rejection.
- Runtime-only Forgejo token environment variables.
- Optional trusted-header delegation derived from the server-side mapping.

## MCP Tools

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

## Approval Gates

Approval records are file-backed, short-lived, exact-payload-bound, single-use,
and require different mapped principals for approval and execution.

Executable high-risk tools in `1.2.0`:

- `create_pull_request`
- `merge_pull_request`
- `create_release`
- `create_wiki_page`
- `update_wiki_page`

`create_pull_request` returns a normalized PR directly at `result.pull_request`
and a richer `result.readback`. If Forgejo returns a sparse create response,
the gateway reads by base/head and then by open PR list before returning
success. Merge reads back the PR by number, reports exact failing check
contexts, and closes open no-diff PRs as stale with a comment.

Admin and destructive execution remains disabled.

## Generated API Coverage

The gateway pins the Forgejo `15.0.3+gitea-1.22.0` Swagger document and
classifies all 491 operations. Only 15 reviewed semantic-overlay operations are
exposed. The other 476 are metadata-only and disabled.

Agents can inspect this safely:

```sh
forgejo-mcpctl api-coverage --filter semantic_overlay --limit 25
forgejo-mcpctl api-coverage --filter destructive --query repo --limit 25
forgejo-mcpctl create-pull-request GetOpir/forgejo-keycloak-rust-mcp --head feature-branch --base main --title "Add feature" --dry-run
```
