# Codeberg Release Text - v1.2.0

Title: `v1.2.0 - PR source-authority hygiene`

`v1.2.0` hardens Forgejo MCP pull-request readback, stale PR handling, and merge-check reporting for governed agent workflows.

## Highlights

- Successful PR create responses now include `result.readback` with PR number, head SHA, state, merged state, merge commit SHA, branch-ref existence, combined check state, and stale classification.
- Sparse Forgejo PR create responses are read back by base/head before falling back to bounded open-PR matching.
- Open PRs with no commits and no changed files ahead of base are commented and closed as stale instead of treated as unfinished work.
- Merge failures now report exact non-success check contexts, status values, and target/status URLs.
- `forgejo-mcpctl merge-pull-request` supports `--status-check-wait-seconds` and `--status-check-poll-seconds`.
- MCP gateway logs include request/readback shape fields needed to debug sparse payloads without logging secrets.

## Install

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.2.0 --locked
```
