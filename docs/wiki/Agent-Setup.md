# Agent Setup

Agents call `/mcp` with:

- `Authorization: Bearer <access-token>`
- JSON body with `operation` and optional `target`.

Use short-lived Keycloak access tokens. Keep tokens out of logs, prompts, wiki pages, and issue comments.

## Discovery

Call `GET /capabilities` before selecting a tool. It lists operation names, required scopes, risk classes, approval requirements, and planned disabled operations. It does not return repository data or secrets.

## Token Path

Preferred production flow:

1. The agent receives a scoped OPIR-P work order or equivalent control-plane authorization.
2. A trusted token broker returns a short-lived Keycloak access token for the MCP audience.
3. The agent stores the token in memory, commonly as `ACCESS_JWT`.
4. The agent calls `/mcp` with `Authorization: Bearer ${ACCESS_JWT}`.

Agents do not need Forgejo personal access tokens. The gateway maps the Keycloak subject to a Forgejo principal and reads the downstream Forgejo token from the gateway runtime environment.

## Pull-Request Bootstrap

Use `create_pull_request` after a branch has already been pushed:

```sh
forgejo-mcpctl create-pull-request GetOpir/forgejo-keycloak-rust-mcp \
  --head feature-branch \
  --base main \
  --title "Add feature" \
  --body "Implementation notes" \
  --reviewer maintainer \
  --dry-run
```

Execution requires an exact-payload approval record created by a different mapped principal. Current PR lifecycle support covers PR creation, PR listing, review listing, issue/PR comments, approval-gated merge, and release publication. Standalone PR update, standalone reviewer request, branch status, required checks, and PR check readback remain planned disabled capabilities.
