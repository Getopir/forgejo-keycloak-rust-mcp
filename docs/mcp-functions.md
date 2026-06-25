# MCP Functions

`0.4.2` exposes a Phase 0 MCP probe endpoint. It validates authentication and evaluates policy for registered operation names. It does not execute Forgejo mutations yet.

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
| `list_repository_metadata` | `forgejo:repo:read` | Read private | No | Policy entry only in this release. |
| `create_issue_comment` | `forgejo:issue:write` | Write additive | No | Policy entry only in this release. |
| `merge_pull_request` | `forgejo:pr:merge` | Write mutating | Yes | Policy entry only in this release. |
| `delete_repository` | `forgejo:org:admin` | Destructive | Yes | Policy entry only in this release. |

Unknown operations return `400`. Missing or invalid tokens return `401`. Missing required scope returns `403`.
