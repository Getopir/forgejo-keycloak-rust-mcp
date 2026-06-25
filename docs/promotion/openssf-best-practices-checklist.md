# OpenSSF Best Practices Badge Checklist

OpenSSF Best Practices Badge site: `https://www.bestpractices.dev/en`

Project URL: `https://codeberg.org/rawholding/forgejo-keycloak-rust-mcp`

## Current Evidence

| Area | Current Status | Evidence |
| --- | --- | --- |
| Public source repository | Ready | Codeberg repository is public. |
| License | Ready | `LICENSE` and `LICENSES/AGPL-3.0-or-later.txt`. |
| Basic build instructions | Ready | `README.md`, `docs/install.md`. |
| Test instructions | Ready | `docs/testing.md`. |
| Security reporting process | Started | `SECURITY.md`. |
| Contribution process | Started | `CONTRIBUTING.md`. |
| Roadmap | Ready | `docs/wiki/Roadmap.md`. |
| Release notes | Ready | `docs/release-notes/0.4.2.md`. |

## Gaps To Close

- Create the OpenSSF project entry and complete the web checklist.
- Add CI for `cargo fmt`, `cargo test`, dependency audit, dependency policy, and SBOM generation.
- Add a public threat model.
- Decide whether to add a dedicated private security contact.
- Add branch protection and required checks once CI exists.
- Add release artifact signing or documented source-tag verification.

## Suggested Badge Claim

Do not add an OpenSSF badge to `README.md` until the checklist is created and the project has at least a passing or baseline badge status.
