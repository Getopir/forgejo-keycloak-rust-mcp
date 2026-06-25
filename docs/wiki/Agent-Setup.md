# Agent Setup

Agents call `/mcp` with:

- `Authorization: Bearer <access-token>`
- JSON body with `operation` and optional `target`.

Use short-lived Keycloak access tokens. Keep tokens out of logs, prompts, wiki pages, and issue comments.
