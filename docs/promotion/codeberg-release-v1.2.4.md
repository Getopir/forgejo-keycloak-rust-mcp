# Codeberg Release Text - v1.2.4

Title: `v1.2.4 - automated dependency review and signed SBOM releases`

`v1.2.4` completes the remaining dependency-update, SBOM publication, and public branch-protection work.

## Highlights

- Weekly Renovate dependency pull requests with automerge disabled.
- Existing CI format, check, test, audit, deny, and SBOM gates apply to dependency updates.
- CycloneDX SBOMs are retained by CI and attached to hosted releases.
- Signed checksums cover source archives and all published SBOM documents.
- Public `main` branch protection requires review and successful checks.

Install from crates.io:

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.2.4 --locked
```

See `docs/release-verification.md` for checksum and SSH-signature verification.
