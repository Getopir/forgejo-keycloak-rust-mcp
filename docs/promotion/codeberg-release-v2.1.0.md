# Codeberg Release Text - v2.1.0

Title: `v2.1.0 - bounded branch status readback`

`v2.1.0` adds `get_branch_status`, the first individually reviewed semantic
operation after the Forgejo 16 compatibility baseline.

## Highlights

- Typed `owner/repository@branch` and stable `forgejo://branch/...` targets.
- Mapped-principal Forgejo ACL enforcement with `forgejo:repo:read` and no
  approval requirement.
- At most 50 required contexts and 50 commit statuses with explicit truncation
  metadata and bounded string fields.
- Fixed 64 KiB branch, 256 KiB combined-status, and 320 KiB operation-result
  limits plus existing configurable Forgejo request timeouts.
- Structured denial, downstream-failure, and success audit behavior.
- CLI support through `forgejo-mcpctl branch-status`.
- Reviewed catalog coverage for `repoGetBranch` and
  `repoGetCombinedStatusByRef`; all other newly reviewed endpoints remain
  disabled.

Install from crates.io:

```sh
cargo install forgejo-keycloak-rust-mcp --version 2.1.0 --locked
```

See `docs/release-notes/2.1.0.md` for upgrade impact and
`docs/release-verification.md` for artifact verification.

No publicly known vulnerability with an assigned CVE was fixed in this
release.
