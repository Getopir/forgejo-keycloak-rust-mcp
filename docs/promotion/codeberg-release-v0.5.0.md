# Codeberg Release: v0.5.0

Tag: `v0.5.0`

Title: `v0.5.0 - Phase 1 principal mapping and repository metadata`

## Release Notes

`v0.5.0` is the first Phase 1 beta for `forgejo-keycloak-rust-mcp`.

This release adds the first Forgejo-backed read-only capability while keeping mutating Forgejo operations disabled.

## Current Capabilities

- Validates Keycloak-issued bearer tokens with issuer, audience, expiry, and JWKS checks.
- Evaluates operation policy before any Forgejo call.
- Maps Keycloak `(issuer, subject)` to a Forgejo account through an explicit local mapping file.
- Rejects unknown or disabled mappings before Forgejo-backed calls.
- Fetches bounded repository metadata through Forgejo API for `list_repository_metadata`.
- Looks up Forgejo API tokens through per-principal environment variable names.
- Derives trusted reverse-proxy identity headers from mapped Forgejo principal data.
- Emits audit records with Keycloak and Forgejo principal metadata and no token values.

## Not Yet Included

- Issue, pull request, review, release, or notification execution.
- Approval gates for mutating actions.
- Generated Forgejo API coverage.
- Admin or destructive operations.

## License

`AGPL-3.0-or-later`

## Verification

- `cargo check --workspace`
- `cargo fmt --check`
- `cargo test --workspace`
- `git diff --check`
- secret/token scan
- internal-host scan outside preserved original inputs

## Links

- Source: `https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp`
- Wiki: `https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/wiki`
- Roadmap: `https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/wiki/Roadmap`
