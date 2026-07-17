# OpenSSF Best Practices Badge Checklist

OpenSSF program page: `https://openssf.org/projects/best-practices-badge/`

OpenSSF Best Practices Badge service: `https://www.bestpractices.dev/en`

Project URL: `https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp`

OpenSSF project ID: `13642`

Earned badge: [Passing](https://www.bestpractices.dev/projects/13642), first achieved 2026-07-17

## Current Evidence

| Area | Current Status | Evidence |
| --- | --- | --- |
| Public source repository | Ready | Codeberg repository is public. |
| License | Ready | `LICENSE` and `LICENSES/AGPL-3.0-or-later.txt`. |
| Basic build instructions | Ready | `README.md`, `docs/install.md`. |
| Test instructions | Ready | `docs/testing.md`. |
| Security checks | Ready | `docs/security-checks.md`, `deny.toml`, `.forgejo/workflows/ci.yml`, Gitleaks, RustSec, dependency policy, and SBOM checks. |
| Security reporting process | Ready | `SECURITY.md`, incident-response runbook, and maintained threat model. |
| Contribution process | Ready | `CONTRIBUTING.md`, including test, license, security, and AI-assisted contribution rules. |
| Roadmap | Ready | `docs/wiki/Roadmap.md`. |
| Release notes | Ready | Hosted releases and `docs/release-notes/`. |
| OpenSSF Passing badge | Earned | Project `13642` reports `badge_level=passing` and 100% passing-series completion. |
| Automation proposal | Complete | Root `.bestpractices.json` records the assigned project and evidence-backed answers. |

## Follow-Up Work

- Complete any criteria that depend on hosted settings not visible in the repository, including maintainer MFA and effective default-branch protection.
- Review Silver and OpenSSF Baseline criteria separately; the `1.3.0` release claims only the earned Passing badge.

## Verified Badge Claim

The public OpenSSF API returns project `13642` for the canonical Codeberg URL and reports `badge_level=passing`, `passing_saved=true`, and an achieved timestamp. The README uses the service-generated badge URL and links directly to that entry.
