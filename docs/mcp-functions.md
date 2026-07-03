# MCP Functions

`1.1.3` exposes a hardened, curated MCP endpoint. It validates authentication, evaluates policy for registered operation names, maps Keycloak principals to Forgejo accounts when configured, executes bounded read operations, supports additive issue creation and issue or pull-request comments, creates pull requests after approval, publishes wiki pages after approval, returns stable resource URIs, validates persistent approval records for high-risk gates, supports approval-backed pull-request merge and release creation, exposes capability metadata, returns safe credential-reference status without secret values, returns bounded generated Forgejo API coverage metadata, and includes HTTPS setup guards for public Forgejo and MCP URLs.

## HTTP Surface

### `GET /health`

Returns service health:

```json
{"service":"forgejo-mcpd","status":"ok"}
```

### `GET /.well-known/oauth-protected-resource`

Returns OAuth protected-resource metadata for clients:

```json
{
  "resource": "https://mcp.example.org/mcp",
  "authorization_servers": ["https://keycloak.example.org/realms/forgejo-agents"],
  "bearer_methods_supported": ["header"],
  "scopes_supported": ["forgejo:repo:read"],
  "resource_signing_alg_values_supported": ["RS256"]
}
```

### `GET /capabilities`

Returns unauthenticated operation metadata for operators and agents:

```json
{
  "operations": [
    {
      "name": "create_pull_request",
      "scope": "forgejo:pr:write",
      "risk": "write_mutating",
      "approval_required": true,
      "description": "Create a pull request and optional review requests after exact-payload approval."
    }
  ],
  "disabled_but_planned": [
    {
      "name": "get_pr_checks",
      "scope": "forgejo:pr:read",
      "risk": "read_private",
      "approval_required": false,
      "reason": "planned read operation for PR readiness"
    }
  ]
}
```

This endpoint does not expose secrets and does not prove access to Forgejo data. It is for discovery of operation names, required scopes, risk classes, approval policy, and planned disabled operations.

### `POST /mcp`

Request:

```json
{
  "operation": "gateway_probe",
  "requested_operation": "merge_pull_request",
  "target": "owner/repository",
  "query": "repo",
  "limit": 25,
  "cursor": "2",
  "state": "open",
  "approval_id": "019f0c14-9f13-7e80-ae5f-5e3b82f5cc1a",
  "dry_run": true
}
```

Response fields:

- `request_id`: unique audit correlation ID.
- `subject`: Keycloak subject from the validated token.
- `oauth_client`: OAuth client ID when present.
- `preferred_username`: optional Keycloak username.
- `operation`: requested operation.
- `requested_operation`: operation to approve when `operation` is `create_approval`.
- `allowed`: policy decision.
- `reason`: allow or deny reason.
- `required_scope`: scope needed for the operation.
- `approval_required`: whether the operation is high-risk.
- `target`: caller-supplied target string for audit context.
- `query`: optional search query for metadata operations such as `forgejo_api_coverage`.
- `result`: operation-specific bounded output for Phase 2 tools.
- `limit`: effective server-capped page limit for list operations.
- `next_cursor`: page token for the next list call, when Forgejo returned a full page.

Resource summaries include `resource_uri` values. Current forms are:

- `forgejo://repository/{owner}/{repo}`
- `forgejo://issue/{owner}/{repo}/{number}`
- `forgejo://pull/{owner}/{repo}/{number}`
- `forgejo://pull-review/{owner}/{repo}/{pull_number}/{review_id}`
- `forgejo://release/{owner}/{repo}/{tag}`
- `forgejo://notification/{id}`
- `forgejo://issue-comment/{owner}/{repo}/{issue_number}/{comment_id}`
- `forgejo://wiki-page/{owner}/{repo}/{title}`

## Registered Operations

| Operation | Scope | Risk | Approval | Current behavior |
| --- | --- | --- | --- | --- |
| `gateway_probe` | `forgejo:repo:read` | Read private | No | Authenticates caller and returns policy decision metadata. |
| `list_repository_metadata` | `forgejo:repo:read` | Read private | No | Maps the Keycloak principal to a Forgejo account and fetches bounded repository metadata through Forgejo API when Phase 1 config is present. |
| `list_repository_issues` | `forgejo:issue:read` | Read private | No | Lists bounded issue summaries for `owner/repository`. |
| `create_issue` | `forgejo:issue:write` | Write additive | No | Creates a bounded Forgejo issue for `owner/repository`. |
| `create_issue_comment` | `forgejo:issue:write` | Write additive | No | Creates an issue or pull-request conversation comment for `owner/repository#number`. |
| `list_pull_requests` | `forgejo:pr:read` | Read private | No | Lists bounded pull-request summaries for `owner/repository`. |
| `create_pull_request` | `forgejo:pr:write` | Write mutating | Yes | Dry-run preview without approval, or approval-backed pull-request creation for `owner/repository`. Optional reviewer requests run after PR creation and are reported separately. |
| `list_pull_request_reviews` | `forgejo:pr:read` | Read private | No | Lists bounded review summaries for `owner/repository#number`. |
| `list_releases` | `forgejo:release:read` | Read private | No | Lists bounded release summaries for `owner/repository`. |
| `list_notifications` | `forgejo:notification:read` | Read private | No | Lists bounded notification summaries for the mapped Forgejo principal. |
| `list_wiki_pages` | `forgejo:wiki:read` | Read private | No | Lists bounded wiki page metadata for `owner/repository`. |
| `get_wiki_page` | `forgejo:wiki:read` | Read private | No | Reads bounded wiki page metadata for `owner/repository`; `query` is the page name. |
| `create_wiki_page` | `forgejo:wiki:write` | Write mutating | Yes | Dry-run preview without approval, or approval-backed wiki page creation. The body contains `title`, `content_base64`, and optional `message`. |
| `update_wiki_page` | `forgejo:wiki:write` | Write mutating | Yes | Dry-run preview without approval, or approval-backed wiki page update. The body contains `title`, `content_base64`, and optional `message`. |
| `credential_reference_status` | `forgejo:repo:read` | Read private | No | Reports mapped principal and downstream token environment-variable presence without returning token or secret values. |
| `forgejo_api_coverage` | `forgejo:repo:read` | Read private | No | Returns bounded generated Forgejo API endpoint classification metadata from the pinned Swagger document. |
| `create_approval` | `forgejo:approval:grant` | Write mutating | No | Creates a short-lived approval record for one exact approval-gated operation payload. |
| `create_release` | `forgejo:release:write` | Write mutating | Yes | Dry-run preview without approval, or approval-backed release creation with single-use approval consumption. |
| `merge_pull_request` | `forgejo:pr:merge` | Write mutating | Yes | Dry-run preview without approval, or approval-backed merge execution with single-use approval consumption. |
| `delete_repository` | `forgejo:org:admin` | Destructive | Yes | Approval-gated and not implemented as an executable tool. |

Unknown operations return `400`. Missing or invalid tokens return `401`. Missing required scope returns `403`.

## Phase 1 Repository Metadata

`list_repository_metadata` requires:

- `FORGEJO_MCPD_PRINCIPAL_MAP`
- `FORGEJO_MCPD_FORGEJO_URL`
- a mapping entry whose `api_token_env` names an environment variable containing that mapped Forgejo principal's API token

Example request:

```json
{
  "operation": "list_repository_metadata",
  "target": "forgejo://repository/GetOpir/forgejo-keycloak-rust-mcp"
}
```

The response includes the mapped Forgejo login, optional Forgejo user ID, trusted delegation headers derived from the mapping, and bounded repository metadata. It never returns the Forgejo API token.

## Phase 2 Curated Tools

All Phase 2 tools require:

- `FORGEJO_MCPD_PRINCIPAL_MAP`
- `FORGEJO_MCPD_FORGEJO_URL`
- a mapped principal whose `api_token_env` names an environment variable containing that principal's Forgejo API token

List tools accept:

- `limit`: optional item limit, capped by `FORGEJO_MCPD_MAX_PAGE_LIMIT`
- `cursor`: optional page token returned as `next_cursor`
- `state`: optional Forgejo state or notification status filter where the endpoint supports it

Examples:

```json
{"operation":"list_repository_issues","target":"GetOpir/forgejo-keycloak-rust-mcp","state":"open","limit":25}
```

```json
{"operation":"list_pull_requests","target":"GetOpir/forgejo-keycloak-rust-mcp","state":"open","limit":25}
```

```json
{"operation":"list_pull_request_reviews","target":"forgejo://pull/GetOpir/forgejo-keycloak-rust-mcp/1","limit":25}
```

```json
{"operation":"list_releases","target":"GetOpir/forgejo-keycloak-rust-mcp","limit":25}
```

```json
{"operation":"list_notifications","state":"unread","limit":25}
```

```json
{
  "operation": "create_issue",
  "target": "GetOpir/forgejo-keycloak-rust-mcp",
  "body": "{\"title\":\"Repair MCP adapter coverage\",\"body\":\"Observed through OPIR-P PRJ-SM-018.\"}"
}
```

```json
{
  "operation": "create_issue_comment",
  "target": "GetOpir/forgejo-keycloak-rust-mcp#1",
  "body": "Thanks, I verified this with the mapped agent."
}
```

```json
{"operation":"list_wiki_pages","target":"GetOpir/forgejo-keycloak-rust-mcp","limit":25}
```

```json
{"operation":"get_wiki_page","target":"GetOpir/forgejo-keycloak-rust-mcp","query":"Home"}
```

```json
{
  "operation": "create_wiki_page",
  "target": "GetOpir/forgejo-keycloak-rust-mcp",
  "body": "{\"title\":\"Agent-Runbook\",\"content_base64\":\"IyBBZ2VudCBSdW5ib29rCg==\",\"message\":\"Publish agent runbook\"}",
  "dry_run": true
}
```

```json
{"operation":"credential_reference_status"}
```

`create_issue` and `create_issue_comment` are additive and still rely on Forgejo ACLs for the mapped user. `create_pull_request`, `merge_pull_request`, `create_release`, `create_wiki_page`, and `update_wiki_page` are executable writes and require valid approval records created by different mapped principals.

Create pull-request dry-run preview:

```json
{
  "operation": "create_pull_request",
  "target": "GetOpir/forgejo-keycloak-rust-mcp",
  "body": "{\"head\":\"feature-branch\",\"base\":\"main\",\"title\":\"Add PR bootstrap\",\"body\":\"Details for reviewers\",\"assignees\":[\"alice\"],\"reviewers\":[\"bob\"]}",
  "dry_run": true
}
```

Create an approval record:

```json
{
  "operation": "create_approval",
  "requested_operation": "create_pull_request",
  "target": "GetOpir/forgejo-keycloak-rust-mcp",
  "body": "{\"head\":\"feature-branch\",\"base\":\"main\",\"title\":\"Add PR bootstrap\",\"body\":\"Details for reviewers\",\"assignees\":[\"alice\"],\"reviewers\":[\"bob\"]}"
}
```

Execute with the returned `approval_id` and the exact same payload:

```json
{
  "operation": "create_pull_request",
  "target": "GetOpir/forgejo-keycloak-rust-mcp",
  "body": "{\"head\":\"feature-branch\",\"base\":\"main\",\"title\":\"Add PR bootstrap\",\"body\":\"Details for reviewers\",\"assignees\":[\"alice\"],\"reviewers\":[\"bob\"]}",
  "approval_id": "019f0c14-9f13-7e80-ae5f-5e3b82f5cc1a"
}
```

Pull-request body fields:

- Required: `head`, `base`, `title`.
- Optional: `body`, `draft`, `assignee`, `assignees`, `reviewers`.

Reviewer requests are a second Forgejo call after PR creation. If reviewer assignment fails, the response still returns the created pull request with `reviewer_request_status` and `reviewer_request_error`.

## Phase 3 Generated API Coverage

`forgejo_api_coverage` returns metadata from the pinned Forgejo `15.0.3+gitea-1.22.0` Swagger document in `vendor/forgejo-api`.

The response includes:

- `summary`: source version, source SHA-256, total operation count, risk counts, target counts, disabled count, semantic-overlay count, approval-required count, destructive count, and admin count.
- `endpoints`: a bounded page of classified endpoint metadata.
- `limit`: effective server-capped page limit.
- `next_cursor`: next page token when more endpoints match.

Filters:

- `state`: optional filter. Supported values include `semantic_overlay`, `disabled`, `approval_required`, `destructive`, `admin`, `read_private`, `write_additive`, `write_mutating`, `secret`, `site_admin`, and `network_egress`.
- `query`: optional search query matched against method, path, Forgejo `operationId`, or semantic MCP operation name.

Examples:

```json
{"operation":"forgejo_api_coverage","state":"semantic_overlay","limit":25}
```

```json
{"operation":"forgejo_api_coverage","state":"destructive","query":"repo","limit":25}
```

Generated coverage is metadata-only by default. Only endpoints with `semantic_overlay` exposure are reachable through named MCP tools. Disabled endpoints are not generic API forwarding targets.

## Approval Store

`0.10.0` and later use persistent single-use approval validation for high-risk gates. Configure it with:

- `FORGEJO_MCPD_APPROVAL_STORE`: path to an append-only JSONL approval file.
- `FORGEJO_MCPD_APPROVAL_TTL_SECONDS`: approval lifetime in seconds. Defaults to `900`.

Create an approval record:

```json
{
  "operation": "create_approval",
  "requested_operation": "merge_pull_request",
  "target": "GetOpir/forgejo-keycloak-rust-mcp#12",
  "body": "{\"method\":\"squash\"}"
}
```

Use the returned `approval_id` with the exact same operation payload:

```json
{
  "operation": "merge_pull_request",
  "target": "GetOpir/forgejo-keycloak-rust-mcp#12",
  "body": "{\"method\":\"squash\"}",
  "approval_id": "019f0c14-9f13-7e80-ae5f-5e3b82f5cc1a"
}
```

The gateway rejects approval IDs when the record is missing, expired, revoked, already consumed, created by the same mapped principal that tries to execute it, or bound to a different operation, target, state, or body hash.

Preview a merge without mutating Forgejo:

```json
{
  "operation": "merge_pull_request",
  "target": "GetOpir/forgejo-keycloak-rust-mcp#12",
  "body": "{\"method\":\"squash\"}",
  "dry_run": true
}
```

Merge options are JSON in the `body` field. Supported `method` values are `merge`, `squash`, `rebase`, and `rebase-merge`. Optional fields are `title`, `message`, `delete_branch_after_merge`, `force_merge`, and `head_commit_id`.

Create a pull-request approval record:

```json
{
  "operation": "create_approval",
  "requested_operation": "create_pull_request",
  "target": "GetOpir/forgejo-keycloak-rust-mcp",
  "body": "{\"head\":\"feature-branch\",\"base\":\"main\",\"title\":\"Add feature\"}"
}
```

Use the returned `approval_id` with the exact same PR payload:

```json
{
  "operation": "create_pull_request",
  "target": "GetOpir/forgejo-keycloak-rust-mcp",
  "body": "{\"head\":\"feature-branch\",\"base\":\"main\",\"title\":\"Add feature\"}",
  "approval_id": "019f0c14-9f13-7e80-ae5f-5e3b82f5cc1a"
}
```

Create a release approval record:

```json
{
  "operation": "create_approval",
  "requested_operation": "create_release",
  "target": "GetOpir/forgejo-keycloak-rust-mcp",
  "body": "{\"tag_name\":\"v0.10.0\",\"name\":\"v0.10.0\",\"body\":\"Release notes\"}"
}
```

Use the returned `approval_id` with the exact same release payload:

```json
{
  "operation": "create_release",
  "target": "GetOpir/forgejo-keycloak-rust-mcp",
  "body": "{\"tag_name\":\"v0.10.0\",\"name\":\"v0.10.0\",\"body\":\"Release notes\"}",
  "approval_id": "019f0c14-9f13-7e80-ae5f-5e3b82f5cc1a"
}
```

Preview release creation without mutating Forgejo:

```json
{
  "operation": "create_release",
  "target": "GetOpir/forgejo-keycloak-rust-mcp",
  "body": "{\"tag_name\":\"v0.10.0\",\"name\":\"v0.10.0\",\"draft\":true}",
  "dry_run": true
}
```

Release options are JSON in the `body` field. Required field is `tag_name`. Optional fields are `target_commitish`, `name`, `body`, `draft`, `prerelease`, and `hide_archive_links`.

## CLI Wrapper

`forgejo-mcpctl` wraps the curated MCP calls for agents and operators. It reads a bearer token from an environment variable and posts to `/mcp`.

Build:

```sh
cargo build --release -p forgejo-keycloak-rust-mcp --bin forgejo-mcpctl
```

Example:

```sh
export FORGEJO_MCPCTL_GATEWAY=http://127.0.0.1:7080/mcp
export FORGEJO_MCPCTL_TOKEN_ENV=ACCESS_JWT
export ACCESS_JWT="<keycloak-access-token-from-your-token-broker>"

forgejo-mcpctl repository-issues forgejo://repository/GetOpir/forgejo-keycloak-rust-mcp --state open --limit 25
forgejo-mcpctl create-issue GetOpir/forgejo-keycloak-rust-mcp --title "Repair MCP adapter coverage" --body "Observed through OPIR-P PRJ-SM-018."
forgejo-mcpctl issue-comment forgejo://issue/GetOpir/forgejo-keycloak-rust-mcp/1 --body "Verified by mapped agent."
forgejo-mcpctl pull-requests GetOpir/forgejo-keycloak-rust-mcp --state open
forgejo-mcpctl create-pull-request GetOpir/forgejo-keycloak-rust-mcp --head feature-branch --base main --title "Add feature" --dry-run
forgejo-mcpctl wiki-pages GetOpir/forgejo-keycloak-rust-mcp --limit 25
forgejo-mcpctl wiki-page GetOpir/forgejo-keycloak-rust-mcp --page Home
forgejo-mcpctl create-wiki-page GetOpir/forgejo-keycloak-rust-mcp --title Agent-Runbook --content-base64 IyBBZ2VudCBSdW5ib29rCg== --dry-run
forgejo-mcpctl credential-reference-status
forgejo-mcpctl notifications --state unread --limit 25
forgejo-mcpctl api-coverage --filter semantic_overlay --limit 25
forgejo-mcpctl api-coverage --filter destructive --query repo --limit 25
forgejo-mcpctl create-release GetOpir/forgejo-keycloak-rust-mcp --tag-name v0.10.0 --name v0.10.0 --dry-run
forgejo-mcpctl merge-pull-request GetOpir/forgejo-keycloak-rust-mcp#12 --method squash --dry-run
forgejo-mcpctl create-approval merge_pull_request GetOpir/forgejo-keycloak-rust-mcp#12 --body '{"method":"squash"}'
```
