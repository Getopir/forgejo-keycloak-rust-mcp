# Security Checks

Run these checks before a public release.

## Rust Checks

```sh
cargo fmt --check
cargo check --workspace
cargo test --workspace
```

## Dependency Audit

Install and run `cargo-audit`:

```sh
cargo install --locked cargo-audit
cargo audit
```

`cargo-audit` checks `Cargo.lock` against the RustSec advisory database.

## Dependency Policy

Install and run `cargo-deny`:

```sh
cargo install --locked cargo-deny
cargo deny check
```

The repository policy lives in `deny.toml` and covers advisories, licenses, duplicate dependency warnings, and registry source restrictions.

## SBOM

Install and run the CycloneDX Cargo plugin:

```sh
cargo install --locked cargo-cyclonedx
cargo cyclonedx --format json --output-file target/forgejo-keycloak-rust-mcp.cdx.json
```

Do not commit generated SBOM files unless the release process explicitly calls for storing that artifact in the repository. Prefer attaching SBOM files to hosted release artifacts.

## CI

`.forgejo/workflows/ci.yml` is provided for self-hosted Forgejo Actions runners. Codeberg documents hosted Actions as limited and points projects that need hosted CI toward Woodpecker CI. Treat the workflow as ready-to-use once a runner is attached to the repository or organization.
