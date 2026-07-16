# Remaining Maintainer Backlog

This is the short, current backlog after `1.2.2`, not a restatement of the
completed phase roadmap. Entries below are deliberate follow-up work; there is
no commitment to enable every Forgejo API endpoint.

## Release And Supply Chain

| Improvement | Status |
| --- | --- |
| Complete REUSE copyright metadata for maintained source files. | Complete in `1.2.2` via `REUSE.toml` and `LICENSES/` |
| Produce signed release artifacts and document verification. | Complete in `1.2.3`; see [Release Artifact Verification](Release-Artifact-Verification.md) |
| Attach the CI-generated CycloneDX SBOM to hosted releases. | Remaining |
| Finish the OpenSSF project entry/badge after public-hosting prerequisites are met. | Remaining |
| Add automated dependency-update review with tests. | Remaining |
| Protect public default branches with required review and checks. | Hosted-repository setting |

## Credential And Operational Hardening

| Improvement | Status |
| --- | --- |
| Document credential-rotation and incident-response procedures. | Complete in `1.2.2`; see [Credential Rotation And Incident Response](Credential-Rotation-And-Incident-Response.md) |
| Add pull-request secret scanning in CI. | Remaining |
| Document JWKS cache limits and key-rotation behaviour. | Remaining |
| Export structured audit records to a durable sink. | Remaining |
| Publish a full threat model linked from the security documentation. | Remaining |
| Add per-agent rate limiting. Approval replay prevention is already shipped. | Remaining |

## Carefully Scoped Capability Work

| Improvement | Status |
| --- | --- |
| PR workflow readbacks: update PR, standalone reviewer request, branch status, required checks, and PR checks. | Deliberately disabled pending review |
| Role/group mapping to operation classes. | Remaining |
| Any further generated Forgejo endpoint. | Requires a named semantic operation, schema, scope, bounded output, audit, and approval policy where applicable |
| Destructive or instance-admin execution. | Intentionally disabled |

Completed work and the original phase plan are documented in the
[Documentation Archive](Documentation-Archive.md) and release notes.
