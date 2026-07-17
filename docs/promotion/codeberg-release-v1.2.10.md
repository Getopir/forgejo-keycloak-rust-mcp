# Codeberg Release Text - v1.2.10

Title: `v1.2.10 - enforced JWT signing-key strength`

`v1.2.10` validates Keycloak signing keys before startup without expanding the
gateway's executable Forgejo capability surface.

## Highlights

- Explicit asymmetric JWT algorithm allowlist and algorithm/key-type matching.
- RSA signing keys must be at least 2048 bits.
- Supported EC and EdDSA algorithms require approved matching curves.
- Weak, ambiguous, duplicate, symmetric, or unsupported signing keys fail
  startup and cannot validate access tokens.

No publicly known vulnerability was fixed in this release.

Install from crates.io:

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.2.10 --locked
```
