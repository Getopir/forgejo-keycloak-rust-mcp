# MCP Functions

The current release exposes:

- `GET /health`
- `GET /.well-known/oauth-protected-resource`
- `GET /.well-known/oauth-protected-resource/mcp`
- `GET /capabilities`
- `POST /mcp`

`POST /mcp` validates the token and evaluates policy for these registered operations:

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
- `delete_repository`

`gateway_probe` returns identity and policy metadata. `list_repository_metadata` executes a read-only Forgejo API lookup when principal mapping and Forgejo URL settings are configured.

Phase 2 baseline tools:

- `list_repository_issues`: bounded issue summaries for `owner/repository`.
- `create_issue_comment`: additive issue or pull-request comment for `owner/repository#number`.
- `list_pull_requests`: bounded pull-request summaries.
- `create_pull_request`: approval-backed PR creation from a pushed branch, with optional assignee and reviewer request inputs.
- `list_pull_request_reviews`: bounded review summaries for `owner/repository#number`.
- `list_releases`: bounded release summaries.
- `list_notifications`: bounded notification summaries for the mapped Forgejo principal.

Phase 3 generated coverage tool:

- `forgejo_api_coverage`: bounded metadata from the pinned Forgejo `15.0.3+gitea-1.22.0` Swagger document. It classifies all 491 operations by target type, risk, approval requirement, and exposure. It does not execute arbitrary Forgejo endpoints.

Capability discovery:

- `GET /capabilities` lists registered operation names, required scopes, risk classes, approval requirements, and planned disabled operations such as standalone PR update and check-readback tools.

List operations accept `limit` and `cursor`. The server caps `limit` with `FORGEJO_MCPD_MAX_PAGE_LIMIT` and returns `next_cursor` when another page may exist.

Resource summaries include stable `forgejo://...` resource URIs. Examples:

- `forgejo://repository/GetOpir/forgejo-keycloak-rust-mcp`
- `forgejo://issue/GetOpir/forgejo-keycloak-rust-mcp/1`
- `forgejo://pull/GetOpir/forgejo-keycloak-rust-mcp/1`
- `forgejo://release/GetOpir/forgejo-keycloak-rust-mcp/v0.10.0`
- `forgejo://notification/123`

High-risk mutations such as repository deletion and admin actions require approval and remain disabled. The stable `1.1.0` release supports approval-backed pull-request creation, pull-request merge, release creation, generated API classification coverage, and capability discovery while keeping non-reviewed generated endpoints disabled.

`create_approval` creates a short-lived record for one exact approval-gated operation payload. The gateway binds that record to the requested operation, target, state, SHA-256 body hash, and approving principal. Execution requires a different mapped principal, consumes the approval before the Forgejo call, and denies replay. `create_pull_request`, `merge_pull_request`, and `create_release` also support dry-run preview with no Forgejo mutation.

PR creation body fields:

- Required: `head`, `base`, `title`.
- Optional: `body`, `draft`, `assignee`, `assignees`, `reviewers`.

Reviewer requests are attempted after PR creation and reported separately as `reviewer_request_status` and `reviewer_request_error`.

The optional `forgejo-mcpctl` binary wraps these operations from a shell while reading the bearer token from an environment variable rather than a command-line argument.

Coverage examples:

```sh
forgejo-mcpctl api-coverage --filter semantic_overlay --limit 25
forgejo-mcpctl api-coverage --filter destructive --query repo --limit 25
forgejo-mcpctl create-pull-request GetOpir/forgejo-keycloak-rust-mcp --head feature-branch --base main --title "Add feature" --dry-run
```
