# Codeberg Release Text - v1.2.1

Title: `v1.2.1 - bounded PR diff and review tools`

`v1.2.1` lets governed agents inspect a bounded pull-request diff and submit an evidence-backed Forgejo review through the mapped reviewer identity.

## Highlights

- `get_pull_request_diff` returns bounded PR metadata, changed-file summaries, and diff text without exposing Forgejo credentials.
- Server-side byte, file, line, and pagination caps keep review context predictable.
- `submit_pull_request_review` supports approve and request-changes reviews through the mapped Forgejo principal.
- Capability discovery, policy classification, CLI commands, tests, and documentation cover both tools.

## Install

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.2.1 --locked
```
