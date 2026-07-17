# Threat Model

This page summarizes the maintained threat model for release `1.2.6`. The complete source-controlled model is in `docs/threat-model.md`.

## Objective And Boundaries

The gateway must prevent an unauthenticated, incorrectly identified, insufficiently scoped, or unapproved caller from causing a Forgejo action or learning repository data. Keycloak authenticates, the gateway authorizes named operation classes and maps the principal, and Forgejo remains the final repository and organization authorization authority.

Trust boundaries are the caller-facing HTTP endpoint, Keycloak OIDC discovery and JWKS, the local principal map and secret environment, the approval store, the Forgejo API, and tracing or durable audit output. TLS termination, host isolation, secret injection, backups, and centralized audit retention are operator-owned boundaries.

## Protected Assets And Attackers

Protected assets include Keycloak and Forgejo credentials, private repository data, repository integrity, mappings and approvals, audit evidence, service availability, and signed release artifacts. Threat actors include unauthenticated callers, compromised valid callers, malicious repository collaborators, network attackers, supply-chain attackers, compromised hosts, and operator configuration mistakes.

## Principal Threats And Controls

| Threat | Primary controls | Residual responsibility |
| --- | --- | --- |
| Identity spoofing or token misuse | JWT signature, issuer, audience, expiry, scope, and immutable subject mapping | Use short-lived tokens and rotate on compromise |
| Confused-deputy access | Per-principal Forgejo token plus Forgejo ACL enforcement | Review mapping and token ownership together |
| Approval forgery or replay | Exact-payload, expiring, different-principal, single-use approvals | Protect the local approval file and service account |
| Operation escalation | Closed operation registry; unreviewed generated endpoints disabled | Review every new semantic operation and risk class |
| Credential disclosure | Token references, token-free responses and audits, CI secret scanning | Harden the host and rotate exposed environment secrets |
| Unbounded reads or denial of service | Page and diff bounds, typed responses, and bounded per-agent token buckets | Apply proxy limits for unauthenticated, human, aggregate, body, concurrency, timeout, and multi-instance traffic; buckets reset on restart |
| Stale signing keys | Validated startup JWKS | Follow overlap-and-restart rotation; there is no TTL, automatic refresh, or JWKS size cap |
| Network interception | HTTPS configuration guard | Terminate authenticated TLS and keep the daemon bind private |
| Audit loss or tampering | Structured append-only synchronized JSONL export | Export off-host, alert on runtime write failures, rotate, and protect retention |
| Supply-chain compromise | Locked dependencies, RustSec, dependency policy, Gitleaks, SBOMs, and signed checksums | Protect CI, registry, release token, and signing-key trust |

## Assumptions And Exclusions

Keycloak signing and claims, Forgejo ACLs, the gateway host, service configuration, clocks, and secret injection must be trustworthy and maintained. A fully compromised gateway host, Forgejo or Keycloak administrator, operating system, reverse proxy, or generic generated Forgejo API execution is outside the application's direct control.

Review the model whenever an operation, trust boundary, credential flow, token or approval rule, audit path, or supported topology changes. See [Security Model](Security-Model.md), [JWKS Cache Limits And Key Rotation](JWKS-Cache-Limits-And-Key-Rotation.md), and [Credential Rotation And Incident Response](Credential-Rotation-And-Incident-Response.md).
