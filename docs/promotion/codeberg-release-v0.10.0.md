# Codeberg Release: v0.10.0

Tag: `v0.10.0`

Title: `v0.10.0 - Approval-backed release creation`

## Summary

`v0.10.0` extends the Phase 2 approval model to Forgejo release publication. Agents can preview release creation without mutating Forgejo, request an approval for the exact release payload, and execute the release only with a valid single-use approval from a different mapped principal.

## Added

- Executable `create_release` after exact-payload approval validation.
- Dry-run release preview with no Forgejo mutation.
- Release payload parsing for `tag_name`, `target_commitish`, `name`, `body`, `draft`, `prerelease`, and `hide_archive_links`.
- Single-use approval consumption before the Forgejo release API call.
- `forgejo-mcpctl create-release` CLI support.

## Security Notes

- Release creation remains high-risk and requires `forgejo:release:write`.
- The approval record binds operation, target, state, payload hash, approver identity, and mapped Forgejo account.
- The approver and executor must be different mapped principals.
- Admin, destructive, generated API, release deletion, release replacement, and release asset upload remain disabled.

## Verification

- `cargo fmt --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo audit`

The hosted Codeberg release object must be created separately from the pushed git tag.

