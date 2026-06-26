# Codeberg Release Text - v1.1.0

Title: `v1.1.0 - Pull-request creation and capability discovery`

`v1.1.0` adds the missing branch-to-pull-request bootstrap operation and a discovery endpoint so agents can inspect supported MCP operations without reading source.

## Highlights

- Adds approval-gated `create_pull_request`.
- Adds `forgejo-mcpctl create-pull-request`.
- Adds unauthenticated `GET /capabilities` for operation names, scopes, risk classes, approval flags, and planned disabled operations.
- Documents short-lived Keycloak token broker guidance for agents.
- Documents canonical issuer/resource URL guidance for operators.
- Regenerates Forgejo API coverage with 10 reviewed semantic overlays and 481 disabled endpoints.

## Install

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.1.0 --locked
```

## Safety

The release does not add generic Forgejo API forwarding. Standalone PR update, standalone reviewer request, branch status, required-check, PR-check, admin, destructive, and generated endpoint execution remain disabled until separately reviewed.

