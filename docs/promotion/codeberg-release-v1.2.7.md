# Codeberg Release Text - v1.2.7

Title: `v1.2.7 - bounded per-agent admission control`

`v1.2.7` adds bounded per-agent request admission and publishes the current OpenSSF evidence preparation without expanding executable Forgejo capabilities.

## Highlights

- Per-agent token buckets keyed by validated immutable Keycloak identity.
- Configurable capacity, refill window, and bounded tracked-agent state.
- HTTP `429` responses with `Retry-After` guidance and denied audit records.
- Documented process-local and multi-instance limitations with proxy controls retained.
- Public `.bestpractices.json` evidence proposals for later maintainer-reviewed OpenSSF self-certification.
- Maintainer backlog entry for a safe Forgejo-linked MCP settings surface.

Install from crates.io:

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.2.7 --locked
```

See `docs/release-verification.md` for checksum and SSH-signature verification.
