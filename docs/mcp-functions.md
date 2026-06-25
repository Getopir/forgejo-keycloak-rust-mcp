# MCP Functions

`0.5.0` exposes a Phase 1 MCP endpoint. It validates authentication, evaluates policy for registered operation names, maps Keycloak principals to Forgejo accounts when configured, and executes the read-only repository metadata tool. It does not execute Forgejo mutations yet.

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
  "target": "owner/repository"
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

## Registered Operations

| Operation | Scope | Risk | Approval | Current behavior |
| --- | --- | --- | --- | --- |
| `gateway_probe` | `forgejo:repo:read` | Read private | No | Authenticates caller and returns policy decision metadata. |
| `list_repository_metadata` | `forgejo:repo:read` | Read private | No | Maps the Keycloak principal to a Forgejo account and fetches bounded repository metadata through Forgejo API when Phase 1 config is present. |
| `create_issue_comment` | `forgejo:issue:write` | Write additive | No | Policy entry only in this release. |
| `merge_pull_request` | `forgejo:pr:merge` | Write mutating | Yes | Policy entry only in this release. |
| `delete_repository` | `forgejo:org:admin` | Destructive | Yes | Policy entry only in this release. |

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
