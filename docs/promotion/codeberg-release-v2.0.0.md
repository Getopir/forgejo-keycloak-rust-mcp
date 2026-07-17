# Codeberg Release Text - v2.0.0

Title: `v2.0.0 - Forgejo 16 compatibility baseline`

`v2.0.0` starts the Forgejo 16-only release line. A configured gateway verifies
`/api/v1/version` before listening and requires the exact `16.0.0` semantic core
version while accepting Forgejo build metadata.

## Highlights

- Clear startup rejection for older, newer, prerelease, malformed, unreachable,
  or unsuccessful Forgejo version responses.
- Health readback reports both required and verified Forgejo versions.
- All 17 existing semantic operations remain covered by the pinned Forgejo 16
  contract.
- All 15 Forgejo 16 additions remain disabled pending individual reviews.
- No configuration or state migration from `1.3.1`.

Install from crates.io:

```sh
cargo install forgejo-keycloak-rust-mcp --version 2.0.0 --locked
```

See `docs/release-notes/2.0.0.md` for upgrade impact and
`docs/release-verification.md` for artifact verification.

No publicly known vulnerability with an assigned CVE was fixed in this
release.
