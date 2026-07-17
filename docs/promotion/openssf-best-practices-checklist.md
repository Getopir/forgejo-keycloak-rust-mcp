# OpenSSF Best Practices Badge Checklist

OpenSSF program page: `https://openssf.org/projects/best-practices-badge/`

OpenSSF Best Practices Badge service: `https://www.bestpractices.dev/en`

Project URL: `https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp`

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
| Automation proposal | Ready | Root `.bestpractices.json` contains evidence-backed proposed answers for maintainer review. |

## Gaps To Close

- Log in to `bestpractices.dev` as an authorized maintainer, create the entry for the Codeberg repository, choose the baseline series, review the automated `.bestpractices.json` proposals, and save only claims verified by the maintainer.
- Complete any criteria that depend on hosted settings not visible in the repository, including maintainer MFA and effective default-branch protection.
- Record the assigned numeric project ID here and add the service-generated badge only after the corresponding badge level is actually earned.

## Suggested Badge Claim

The public repository currently has no matching OpenSSF entry. Do not add a badge to `README.md` until `https://www.bestpractices.dev/projects.json?url=https%3A%2F%2Fcodeberg.org%2FGetOpir%2Fforgejo-keycloak-rust-mcp` returns the maintainer-created entry and that entry reports an earned badge level. An in-progress or fabricated badge would misrepresent the external self-certification state.
