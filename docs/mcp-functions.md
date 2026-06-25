# MCP Functions

`0.6.0` exposes a Phase 1 hardened and Phase 2 baseline MCP endpoint. It validates authentication, evaluates policy for registered operation names, maps Keycloak principals to Forgejo accounts when configured, executes bounded read operations, and supports additive issue or pull-request comments.

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

### `POST /mcp`

Request:

```json
{
  "operation": "gateway_probe",
  "target": "owner/repository",
  "limit": 25,
  "cursor": "2",
  "state": "open"
}
```

Response fields:

- `request_id`: unique audit correlation ID.
- `subject`: Keycloak subject from the validated token.
- `oauth_client`: OAuth client ID when present.
- `preferred_username`: optional Keycloak username.
- `operation`: requested operation.
- `allowed`: policy decision.
- `reason`: allow or deny reason.
- `required_scope`: scope needed for the operation.
- `approval_required`: whether the operation is high-risk.
- `target`: caller-supplied target string for audit context.
- `result`: operation-specific bounded output for Phase 2 tools.
- `limit`: effective server-capped page limit for list operations.
- `next_cursor`: page token for the next list call, when Forgejo returned a full page.

## Registered Operations

| Operation | Scope | Risk | Approval | Current behavior |
| --- | --- | --- | --- | --- |
| `gateway_probe` | `forgejo:repo:read` | Read private | No | Authenticates caller and returns policy decision metadata. |
| `list_repository_metadata` | `forgejo:repo:read` | Read private | No | Maps the Keycloak principal to a Forgejo account and fetches bounded repository metadata through Forgejo API when Phase 1 config is present. |
| `list_repository_issues` | `forgejo:issue:read` | Read private | No | Lists bounded issue summaries for `owner/repository`. |
| `create_issue_comment` | `forgejo:issue:write` | Write additive | No | Creates an issue or pull-request conversation comment for `owner/repository#number`. |
| `list_pull_requests` | `forgejo:pr:read` | Read private | No | Lists bounded pull-request summaries for `owner/repository`. |
| `list_pull_request_reviews` | `forgejo:pr:read` | Read private | No | Lists bounded review summaries for `owner/repository#number`. |
| `list_releases` | `forgejo:release:read` | Read private | No | Lists bounded release summaries for `owner/repository`. |
| `list_notifications` | `forgejo:notification:read` | Read private | No | Lists bounded notification summaries for the mapped Forgejo principal. |
| `create_release` | `forgejo:release:write` | Write mutating | Yes | Approval-gated; no release is created without an approval record. |
| `merge_pull_request` | `forgejo:pr:merge` | Write mutating | Yes | Approval-gated; no merge is executed without an approval record. |
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
  "target": "rawholding/forgejo-keycloak-rust-mcp"
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
{"operation":"list_repository_issues","target":"rawholding/forgejo-keycloak-rust-mcp","state":"open","limit":25}
```

```json
{"operation":"list_pull_requests","target":"rawholding/forgejo-keycloak-rust-mcp","state":"open","limit":25}
```

```json
{"operation":"list_pull_request_reviews","target":"rawholding/forgejo-keycloak-rust-mcp#1","limit":25}
```

```json
{"operation":"list_releases","target":"rawholding/forgejo-keycloak-rust-mcp","limit":25}
```

```json
{"operation":"list_notifications","state":"unread","limit":25}
```

```json
{
  "operation": "create_issue_comment",
  "target": "rawholding/forgejo-keycloak-rust-mcp#1",
  "body": "Thanks, I verified this with the mapped agent."
}
```

`create_issue_comment` is the only executable write in `0.6.0`. It is additive and still relies on Forgejo ACLs for the mapped user. High-risk writes return an approval-required response and do not execute. Caller-supplied `approval_id` values are not authority until a persistent approval store and validator are implemented.
