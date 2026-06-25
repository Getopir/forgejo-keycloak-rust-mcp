# MCP Functions

The current release exposes:

- `GET /health`
- `GET /.well-known/oauth-protected-resource`
- `GET /.well-known/oauth-protected-resource/mcp`
- `POST /mcp`

`POST /mcp` validates the token and evaluates policy for these registered operations:

- `gateway_probe`
- `list_repository_metadata`
- `create_issue_comment`
- `merge_pull_request`
- `delete_repository`

Only `gateway_probe` returns a concrete Phase 0 probe response today. The other operations are policy-registered for risk and scope validation before Forgejo execution is implemented.
