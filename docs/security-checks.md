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
cargo cyclonedx --format json
```

This produces one `*.cdx.json` document beside each workspace crate manifest. CI validates SBOM generation on every push and pull request. Publishing a Forgejo release triggers `.forgejo/workflows/release-sbom.yml`, which regenerates and attaches the SBOM set to that hosted release. The signed local release builder includes those documents in `SHA256SUMS` for publication to Codeberg and other release hosts.

Do not commit generated SBOM files. Attach them to hosted release artifacts.

## CI

`.forgejo/workflows/ci.yml` runs on the attached self-hosted Forgejo Actions runner. Every push and pull request is scanned with the pinned Gitleaks release before the Rust checks run. CI verifies the downloaded scanner archive against its published SHA-256 digest and scans the complete checked-out Git history with redacted findings.

Run the same scan locally with Gitleaks `8.30.1`:

```sh
gitleaks git --redact --no-banner .
```

`.forgejo/workflows/dependency-updates.yml` runs Renovate weekly and opens dependency pull requests; those pull requests must pass the same secret scan, format, check, test, audit, dependency-policy, and SBOM jobs before review.
