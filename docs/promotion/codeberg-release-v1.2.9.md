# Codeberg Release Text - v1.2.9

Title: `v1.2.9 - mandatory strict Clippy analysis`

`v1.2.9` makes strict Rust source analysis mandatory without changing the
gateway's executable Forgejo capability surface.

## Highlights

- Internal Forgejo and public Codeberg CI run strict Clippy across the complete
  workspace and every target.
- Every Clippy warning fails push and pull-request validation.
- OpenSSF warning-strictness and continuous-static-analysis evidence is updated.

No publicly known vulnerability was fixed in this release.

Install from crates.io:

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.2.9 --locked
```
