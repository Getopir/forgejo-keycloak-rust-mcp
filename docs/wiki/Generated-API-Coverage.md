# Generated API Coverage

Version `1.2.9` pins the Forgejo API metadata and classifies it before
any generated endpoint can become an MCP tool.

## Pinned Source

- Forgejo version: `16.0.0`
- Pinned file: `vendor/forgejo-api/forgejo-16.0.0-swagger.v1.json`
- SHA-256: `a41f976f1d616e273c0a1855a625928e59e758f324f0b02fc247a25a5469be84`
- Coverage report: `docs/generated/forgejo-api-coverage.md`
- Refresh review: `docs/generated/forgejo-api-coverage-review-16.0.0.md`

## Current Coverage

- Total operations: 506
- Semantic-overlay operations: 18
- Disabled metadata-only operations: 488

The semantic overlay connects reviewed Forgejo endpoints to named MCP
operations such as `list_repository_metadata`, `list_repository_issues`,
`create_issue`, `list_pull_requests`, `create_pull_request`, `list_releases`,
`create_release`, `list_wiki_pages`, `get_wiki_page`, `create_wiki_page`,
`update_wiki_page`, and `merge_pull_request`.

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
