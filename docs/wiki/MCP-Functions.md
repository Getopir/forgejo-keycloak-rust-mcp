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
- `delete_repository`

`gateway_probe` returns identity and policy metadata. `list_repository_metadata` executes a read-only Forgejo API lookup when principal mapping and Forgejo URL settings are configured.

Phase 2 baseline tools:

- `list_repository_issues`: bounded issue summaries for `owner/repository`.
- `create_issue`: additive issue creation for `owner/repository`.
- `create_issue_comment`: additive issue or pull-request comment for `owner/repository#number`.
- `list_pull_requests`: bounded pull-request summaries.
- `create_pull_request`: approval-backed PR creation from a pushed branch, with optional assignee and reviewer request inputs. Successful responses return the normalized PR at `result.pull_request`.
- `list_pull_request_reviews`: bounded review summaries for `owner/repository#number`.
- `submit_pull_request_review`: submit an evidence-backed `APPROVED` or `REQUEST_CHANGES` review as the mapped reviewer identity; pass the review text in `body` and the verdict in `state`.
- `get_pull_request_diff`: bounded pull-request metadata, changed-file summaries, and diff text for `owner/repository#number`.
- `list_releases`: bounded release summaries.
- `list_notifications`: bounded notification summaries for the mapped Forgejo principal.
- `list_wiki_pages`: bounded wiki page metadata for `owner/repository`.
- `get_wiki_page`: bounded wiki page metadata for `owner/repository`; pass the page name in `query`.
- `create_wiki_page`: approval-backed wiki page creation with `title`, `content_base64`, and optional `message`.
- `update_wiki_page`: approval-backed wiki page update with `title`, `content_base64`, and optional `message`.
- `credential_reference_status`: mapped identity and downstream token environment-variable presence without secret values.

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
- `forgejo://wiki-page/GetOpir/forgejo-keycloak-rust-mcp/Home`

High-risk mutations such as repository deletion and admin actions require approval and remain disabled. The stable `1.2.7` release supports bounded per-agent admission control, pull-request diff inspection, evidence-backed review submission, additive issue creation, approval-backed pull-request creation with normalized readback, pull-request merge with status-context reporting, stale no-diff PR closure, release creation, approval-backed wiki publication, safe credential-reference status, generated API classification coverage, capability discovery, and HTTPS setup guards while keeping non-reviewed generated endpoints disabled.

`create_approval` creates a short-lived record for one exact approval-gated operation payload. The gateway binds that record to the requested operation, target, state, SHA-256 body hash, and approving principal. Execution requires a different mapped principal, consumes the approval before the Forgejo call, and denies replay. `create_pull_request`, `merge_pull_request`, `create_release`, `create_wiki_page`, and `update_wiki_page` also support dry-run preview with no Forgejo mutation.

PR creation body fields:

- Required: `head`, `base`, `title`.
- Optional: `body`, `draft`, `assignee`, `assignees`, `reviewers`.

Reviewer requests are attempted after PR creation and reported separately as `reviewer_request_status` and `reviewer_request_error`.

After execution, `result.pull_request` always contains a normalized PR object with `number`, `state`, `title`, and `url` or `html_url`. Head/base `ref`, `sha`, `label`, `mergeable`, `merged`, and `merge_commit_sha` are included when Forgejo returns them. `result.readback` persists the PR number, head SHA, state, merged state, merge commit SHA, combined check state, branch-ref existence, and stale classification. Sparse create responses trigger base/head readback before list-based matching. No successful response is returned without a PR number.

Before merge, the gateway reads the PR back by number and checks the head SHA status. Non-green check failures include exact context, status, and URLs. Open PRs with no commits and no changed files ahead of base are commented and closed as stale instead of reported as unfinished work.

The optional `forgejo-mcpctl` binary wraps these operations from a shell while reading the bearer token from an environment variable rather than a command-line argument.

Coverage examples:

```sh
forgejo-mcpctl api-coverage --filter semantic_overlay --limit 25
forgejo-mcpctl api-coverage --filter destructive --query repo --limit 25
forgejo-mcpctl create-pull-request GetOpir/forgejo-keycloak-rust-mcp --head feature-branch --base main --title "Add feature" --dry-run
forgejo-mcpctl create-issue GetOpir/forgejo-keycloak-rust-mcp --title "Repair MCP adapter coverage"
forgejo-mcpctl wiki-pages GetOpir/forgejo-keycloak-rust-mcp --limit 25
forgejo-mcpctl create-wiki-page GetOpir/forgejo-keycloak-rust-mcp --title Agent-Runbook --content-base64 IyBBZ2VudCBSdW5ib29rCg== --dry-run
forgejo-mcpctl credential-reference-status
```
