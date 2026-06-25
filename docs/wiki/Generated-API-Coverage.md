# Generated API Coverage

Version `1.0.0` pins the live Forgejo API metadata and classifies it before
any generated endpoint can become an MCP tool.

## Pinned Source

- Forgejo version: `15.0.3+gitea-1.22.0`
- Pinned file: `vendor/forgejo-api/forgejo-15.0.3-gitea-1.22.0-swagger.v1.json`
- SHA-256: `a90f2fe1266a7a08dfcf682cd28db96c364e18a7de2a4e559a26afe3485bb26f`
- Coverage report: `docs/generated/forgejo-api-coverage.md`

## Current Coverage

- Total operations: 491
- Semantic-overlay operations: 9
- Disabled metadata-only operations: 482

The semantic overlay connects reviewed Forgejo endpoints to named MCP
operations such as `list_repository_metadata`, `list_repository_issues`,
`list_pull_requests`, `list_releases`, `create_release`, and
`merge_pull_request`.

## Safety Rule

Generated coverage is not generic endpoint forwarding. Endpoints are classified
for review and reporting, but they are disabled unless a reviewed semantic
operation exists. Unknown, admin, destructive, secret-bearing, and
network-egress endpoints stay unavailable to normal agents.

## Agent Readback

Agents can inspect the generated catalog with:

```sh
forgejo-mcpctl api-coverage --filter semantic_overlay --limit 25
forgejo-mcpctl api-coverage --filter destructive --query repo --limit 25
forgejo-mcpctl api-coverage --filter admin --limit 25
```

The MCP operation is `forgejo_api_coverage`. It requires
`forgejo:repo:read`, returns bounded pages, and supports `cursor` and `limit`.
