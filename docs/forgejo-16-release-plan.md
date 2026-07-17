# Forgejo 16 Release Plan

## Compatibility Boundary

The `1.x` release line is the compatibility line for Forgejo versions before
`16.0.0`. Its final published release is `1.3.1`.

The `2.x` release line targets Forgejo `16.0.0` only. Version `2.0.0` establishes
that compatibility boundary and verifies the existing curated operations against
Forgejo 16 before any new semantic operation is enabled.

Users must not assume that a `2.x` binary supports an older Forgejo server. The
gateway must fail clearly when a required Forgejo 16 contract is unavailable.

## Versioning And Stop Rule

- `2.0.0` establishes the Forgejo 16-only baseline without adding a new
  executable operation.
- Each planned semantic operation receives exactly one minor release, starting
  with `2.1.0`.
- A compatible bug or security repair increments the patch component of the
  affected minor line, for example `2.2.1`.
- Unrelated operations must not be batched into one feature release.
- Work stops after every release until its source, tag, release notes, artifacts,
  wiki, registry package, deployment, and readback are complete.

## Semantic Operation Contract

Every new operation must satisfy all of these requirements before release:

1. A stable, named semantic operation rather than generic generated endpoint
   execution.
2. Typed request and response schemas with malformed and unknown input tests.
3. A least-privilege Keycloak scope and an explicit risk classification.
4. Server-enforced pagination, item, byte, and time bounds as applicable.
5. Structured audit records for allow, deny, downstream failure, and success
   without tokens, credentials, or unbounded response content.
6. An explicit approval decision. Read operations normally require no approval;
   mutations require exact-payload, different-principal, single-use approval
   unless a documented review proves a narrower policy is safe.
7. Forgejo ACL enforcement using the mapped principal.
8. Unit and integration coverage for success, denial, invalid input, bound
   enforcement, and downstream failure.
9. Updated capability metadata, CLI support where useful, public docs, wiki,
   generated coverage review, and human-readable release notes.
10. A release verification record including the Forgejo 16 version tested and
    the deployed readback result.

## Release Train

| Version | Issue | Deliverable | Approval policy |
| --- | --- | --- | --- |
| `2.0.0` | [#1](https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/issues/1) | Complete: established and verified the Forgejo 16-only compatibility baseline | No new operation |
| `2.1.0` | [#2](https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/issues/2) | Complete: added bounded `get_branch_status` with typed branch targets and combined status summaries | Read; no approval |
| `2.2.0` | [#3](https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/issues/3) | Add bounded `get_required_checks` | Read; no approval |
| `2.3.0` | [#4](https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/issues/4) | Add bounded `get_pr_checks` | Read; no approval |
| `2.4.0` | [#5](https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/issues/5) | Add `update_pull_request` | Exact-payload approval |
| `2.5.0` | [#6](https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/issues/6) | Add standalone `request_reviewers` | Exact-payload approval |
| `2.6.0` | [#7](https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/issues/7) | Add bounded `list_action_runs` | Read; no approval |
| `2.7.0` | Open before work starts | Add bounded `list_action_run_jobs` | Read; no approval |
| `2.8.0` | Open before work starts | Add bounded `get_action_job_logs` | Read; no approval; byte limit |
| `2.9.0` | Open before work starts | Add bounded `get_action_run_logs` | Read; no approval; byte limit |
| `2.10.0` | Open before work starts | Add bounded `list_action_artifacts` | Read; no approval |
| `2.11.0` | Open before work starts | Add bounded `download_action_artifact` | Read; no approval; size and content limits |
| `2.12.0` | Open before work starts | Add `cancel_action_run` | Exact-payload approval |
| `2.13.0` | Open before work starts | Add bounded `get_repository_branch` | Read; no approval |
| `2.14.0` | Open before work starts | Add bounded `list_repository_commits` | Read; no approval |
| `2.15.0` | Open before work starts | Add bounded `get_repository_file` | Read; no approval; byte limit |
| `2.16.0` | Open before work starts | Add `upload_release_asset` | Exact-payload approval; size and content-type limits |
| `2.17.0` | Open before work starts | Add `update_issue` | Exact-payload approval |
| `2.18.0` | Open before work starts | Add role/group mapping to operation classes | Policy change; deny by default |

The version targets are planning identifiers. If a repair release is required,
it is inserted without renumbering later minor releases.

Codeberg enforces anti-spam limits on bulk issue creation. Remaining rows must
receive their individual public issue before implementation begins; the release
must not proceed while its issue cell still says `Open before work starts`.

## Intentionally Disabled Forgejo 16 Endpoints

Forgejo 16 admin token management, artifact deletion, action-run deletion, and
ActivityPub follow operations remain disabled. They are not implicitly approved
by the `2.x` migration and require separate threat, scope, approval, and output
reviews before an issue may propose executable exposure.

## Per-Release Evidence

Before starting the next issue, the completed release must have:

- reviewed commits and passing format, test, strict Clippy, coverage, dependency,
  secret-scan, and SBOM checks;
- a signed `vX.Y.Z` tag pointing at the published source commit;
- human-readable release notes describing upgrade impact and CVE status;
- matching hosted releases and artifacts on local Forgejo and Codeberg;
- synchronized Codeberg and local Forgejo wikis;
- matching crates.io packages and a clean `cargo install --locked` check;
- a deployed service readback against Forgejo 16, including health, capability,
  expected denial, and one authorized read or approved mutation;
- an issue closeout comment linking the tag, releases, tests, and deployment
  evidence.

This evidence supports the OpenSSF requirements for public interim changes,
unique versions, tags, searchable issue discussion, and human-readable release
notes. The badge remains a self-certification and must be updated if project
practice stops matching its recorded answers.
