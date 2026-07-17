# Codeberg Release Text - v1.2.11

Title: `v1.2.11 - enforced domain-code coverage`

`v1.2.11` publishes and enforces majority domain-code coverage without changing
the gateway's executable Forgejo capability surface.

## Highlights

- Reproducible `cargo-llvm-cov` measurement in both CI environments.
- Required minimums: 55% lines, 50% functions, and 55% regions.
- Current baseline: 59.62% lines, 52.48% functions, and 58.50% regions.
- Public documentation defines the measurement scope and entrypoint exclusion.

No publicly known vulnerability was fixed in this release.

Install from crates.io:

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.2.11 --locked
```
