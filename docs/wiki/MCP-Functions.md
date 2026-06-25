# MCP Functions

The current release exposes:

- `GET /health`
- `GET /.well-known/oauth-protected-resource`
- `GET /.well-known/oauth-protected-resource/mcp`
- `POST /mcp`

`POST /mcp` validates the token and evaluates policy for these registered operations:

- `gateway_probe`
- `list_repository_metadata`
- `list_repository_issues`
- `create_issue_comment`
- `list_pull_requests`
- `list_pull_request_reviews`
- `list_releases`
- `list_notifications`
- `create_release`
- `merge_pull_request`
- `delete_repository`

`gateway_probe` returns identity and policy metadata. `list_repository_metadata` executes a read-only Forgejo API lookup when principal mapping and Forgejo URL settings are configured.

Phase 2 baseline tools:

- `list_repository_issues`: bounded issue summaries for `owner/repository`.
- `create_issue_comment`: additive issue or pull-request comment for `owner/repository#number`.
- `list_pull_requests`: bounded pull-request summaries.
- `list_pull_request_reviews`: bounded review summaries for `owner/repository#number`.
- `list_releases`: bounded release summaries.
- `list_notifications`: bounded notification summaries for the mapped Forgejo principal.

List operations accept `limit` and `cursor`. The server caps `limit` with `FORGEJO_MCPD_MAX_PAGE_LIMIT` and returns `next_cursor` when another page may exist.

Resource summaries include stable `forgejo://...` resource URIs. Examples:

- `forgejo://repository/rawholding/forgejo-keycloak-rust-mcp`
- `forgejo://issue/rawholding/forgejo-keycloak-rust-mcp/1`
- `forgejo://pull/rawholding/forgejo-keycloak-rust-mcp/1`
- `forgejo://release/rawholding/forgejo-keycloak-rust-mcp/v0.7.0`
- `forgejo://notification/123`

High-risk mutations such as release creation, pull-request merge, repository deletion, and admin actions require approval and are not executed by the baseline approval gate. Caller-supplied approval IDs are not trusted until a persistent approval store and validator are implemented.

The optional `forgejo-mcpctl` binary wraps these operations from a shell while reading the bearer token from an environment variable rather than a command-line argument.
