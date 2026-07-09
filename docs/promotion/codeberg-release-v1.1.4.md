# Codeberg Release Text - v1.1.4

Title: `v1.1.4 - Normalized PR creation readback`

`v1.1.4` fixes the Forgejo MCP `create_pull_request` response contract for governed agent workflows.

## Highlights

- Successful `create_pull_request` calls now return the normalized PR directly at `result.pull_request`.
- The returned PR includes `number`, `url` or `html_url`, `state`, `title`, and head/base metadata when Forgejo provides it.
- Sparse Forgejo create responses trigger an immediate open-PR readback by repo, head, base, and title.
- Ambiguous or missing readback results return clear errors with request context and candidate PR numbers, never secrets.

## Install

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.1.4 --locked
```
