# Codeberg Release Text - v1.3.1

Title: `v1.3.1 - atomic approvals and bounded Forgejo requests`

`v1.3.1` hardens approval-backed mutations against concurrent replay and
bounds outbound Forgejo HTTP requests without expanding the executable
capability surface.

## Highlights

- Atomic approval validation and consumption across gateway processes.
- A simultaneous-consumer regression test that permits exactly one execution.
- Direct regression coverage for JWT header/JWK algorithm mismatches.
- Configurable 5-second connect and 30-second total Forgejo request timeouts.
- Updated maintained source and wiki documentation.

No publicly known vulnerability with an assigned CVE was fixed in this release.

Install from crates.io:

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.3.1 --locked
```
