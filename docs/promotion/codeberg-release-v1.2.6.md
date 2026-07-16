# Codeberg Release Text - v1.2.6

Title: `v1.2.6 - published threat model and operational hardening`

`v1.2.6` publishes the full maintained threat model and releases the audit, JWKS, and secret-scanning hardening completed after `1.2.4`.

## Highlights

- Full threat model with explicit trust boundaries, attacker profiles, mitigations, residual risks, and deployment assumptions.
- Checksum-verified Gitleaks scanning on pushes and pull requests.
- Durable append-only JSONL audit export through `FORGEJO_MCPD_AUDIT_LOG`.
- Explicit JWKS cache limits and safe signing-key rotation procedure.
- No expansion of the executable Forgejo operation surface.

Install from crates.io:

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.2.6 --locked
```

See `docs/release-verification.md` for checksum and SSH-signature verification.
