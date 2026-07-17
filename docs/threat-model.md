# Threat Model

Status: maintained for release `1.2.8`
Last reviewed: 2026-07-17

## Scope And Security Objective

This model covers `forgejo-keycloak-rust-mcpd`, `forgejo-mcpctl`, the identity, policy, audit, approval, principal-mapping, and Forgejo client code in this repository, and their documented deployment interfaces. It covers the path from a caller presenting a Keycloak access token to the gateway making an authorized Forgejo API request.

The primary objective is to prevent an unauthenticated, incorrectly identified, insufficiently scoped, or unapproved caller from causing a Forgejo action or learning repository data through the gateway. Secondary objectives are to avoid credential disclosure, bound returned data, preserve useful audit evidence, and keep unreviewed Forgejo API operations unreachable.

Keycloak remains the authentication authority. The gateway authorizes named operation classes and binds identities to configured Forgejo principals. Forgejo remains the final repository and organization authorization authority.

## Protected Assets

- Keycloak access tokens, signing-key trust, subjects, audiences, and scopes.
- Forgejo personal or service-account tokens referenced by the principal map.
- Private repository content, pull-request diffs, issues, releases, wiki content, and notifications.
- Repository integrity, branches, reviews, releases, and approval-gated mutations.
- Principal-map, approval-store, and daemon configuration integrity.
- Structured audit records and their retention history.
- Release source, dependencies, SBOMs, checksums, and signing identity.
- Gateway and Forgejo availability.

## Actors And Attacker Profiles

- **Authorized human or agent:** owns a valid Keycloak token and is expected to stay within assigned scopes.
- **Malicious or compromised caller:** has no token, a stolen token, or a valid low-privilege token and attempts privilege escalation or data extraction.
- **Malicious repository collaborator:** has some Forgejo access and attempts to use gateway behavior, pull requests, or CI inputs to gain broader access.
- **Network attacker:** can observe, replay, redirect, or modify traffic where TLS and network boundaries are incorrectly deployed.
- **Compromised gateway host or service account:** can read runtime environment, principal maps, approval files, or audit files. This actor is largely outside the controls of the application process.
- **Supply-chain attacker:** attempts to introduce malicious source, dependencies, CI actions, build tools, or release artifacts.
- **Operator:** controls deployment URLs, Keycloak configuration, principal mappings, filesystem permissions, reverse proxy, and retention settings. Operator mistakes are considered a threat source.

## Trust Boundaries And Data Flow

1. A caller crosses the public HTTP boundary and sends a bearer token and a bounded MCP request to the gateway.
2. The gateway crosses the Keycloak/OIDC boundary at startup to load discovery metadata and JWKS. At request time it validates signature, issuer, audience, expiry, and scopes locally.
3. The validated `(issuer, subject)` crosses the local configuration boundary into the principal map, which names a Forgejo login and the environment-variable reference for that principal's Forgejo token.
4. High-risk reviewed operations cross the approval-store boundary. The approval must match the exact request, be unexpired, be issued by a different mapped principal, and be consumed once.
5. The gateway crosses the Forgejo API boundary using the mapped principal's token. Forgejo applies its own repository and organization ACLs.
6. Results cross back to the caller only through operation-specific bounded response types. Raw downstream tokens are never returned.
7. Audit metadata crosses the logging boundary to tracing and, when configured, an append-only synchronized JSONL file.

TLS termination, host isolation, Keycloak administration, Forgejo administration, secret injection, filesystem durability, backup, and centralized log ingestion are deployment boundaries rather than code-owned controls.

## Threats, Controls, And Residual Risk

| Threat | Existing controls | Residual risk and required operation |
| --- | --- | --- |
| Spoofed or malformed caller identity | Strict bearer parsing; JWT signature, issuer, audience, and expiry validation; unknown or disabled mappings denied | A stolen valid token works until expiry or revocation controls take effect. Keep access tokens short-lived and contain incidents through the rotation runbook. |
| Forged trusted reverse-proxy identity | Caller-supplied configured identity headers are rejected; headers are derived from the validated mapping | A proxy that fails to strip alternate identity headers or exposes a trusted backend path can bypass the intended boundary. Restrict and test proxy routes. |
| Confused deputy or cross-account access | Immutable `(issuer, subject)` mapping; per-principal Forgejo token; Forgejo ACL enforcement on every downstream call | A mapping or injected token that names the wrong Forgejo account grants that account's authority. Review mappings and token ownership together. |
| Scope or operation escalation | Closed operation registry; explicit scopes and risk classes; unknown generated endpoints remain disabled | Incorrect registry classification is security-sensitive. Review policy changes and generated API coverage before enabling operations. |
| Approval forgery or replay | Exact-payload binding, expiry, approver/executor separation, and consume-before-execute single use | A service account with write access to the approval file can tamper with local state. Restrict file permissions and isolate the service account. |
| Destructive or instance-admin action | Destructive, generic generated, and instance-admin execution paths are intentionally disabled | Future capability additions can expand impact. Each must have a named semantic overlay, bounded schema, scope, audit behavior, and approval policy. |
| Credential disclosure | Forgejo tokens are read from named environment variables; mapping and audit schemas exclude token values; responses omit downstream credentials; secret scanning runs in CI | A compromised process or host can read environment secrets. Use a dedicated service account, restrict diagnostics and core dumps, and rotate exposed credentials. |
| Sensitive-data over-read or resource exhaustion | Typed responses, page caps, bounded diff size, reviewed endpoint-specific parsing, and bounded token buckets for mapped agents | JWKS size has no application-level cap. Rate-limit state is process-local and excludes non-agent traffic. Apply reverse-proxy limits and monitor memory, request rate, and response volume. |
| Stale or malicious signing-key state | Discovery and JWKS are fetched over the configured transport; tokens must match a loaded key | JWKS is a startup-only snapshot with no TTL, refresh-on-`kid`, or size cap. Follow the documented overlap-and-restart rotation procedure and protect OIDC transport. |
| Network interception or endpoint redirection | Optional `--tls` guard rejects HTTP public resource and Forgejo URLs | The daemon does not terminate TLS. A misconfigured reverse proxy, CA store, DNS, or unprotected local network can expose tokens. Use authenticated TLS and restrict the local bind. |
| Request-target injection or SSRF | Forgejo base URL is operator configuration; repository and numbered targets are parsed into typed path components; callers cannot select arbitrary upstream hosts | Compromised configuration can redirect Forgejo credentials. Protect service configuration and validate the configured origin during deployment. |
| Audit suppression, tampering, or disk exhaustion | Token-safe structured schema; append-only JSONL mode; each configured write is flushed and synchronized; startup fails if the sink cannot open | Runtime sink failures are logged but do not fail requests. Local privileged users can alter files and unbounded retention can fill storage. Export off-host, alert on write errors, rotate, and protect retention. |
| Dependency, CI, or release compromise | Locked Rust dependencies, RustSec audit, `cargo-deny`, checksum-verified Gitleaks, CycloneDX SBOMs, signed release checksum manifests, and review-required dependency updates | CI actions, tool downloads, registries, and maintainer signing hosts remain trusted dependencies. Pin and review updates, protect release credentials, and verify published artifacts. |
| Denial of service | Bounded list and diff outputs; malformed requests fail before Forgejo calls; mapped agents receive per-identity token buckets with bounded tracking and `429` retry guidance | No global admission queue or distributed limiter exists, and buckets reset on restart. Enforce unauthenticated, body, concurrency, aggregate-rate, and timeout limits at the reverse proxy and monitor Keycloak and Forgejo dependencies. |

## Security Assumptions

- Keycloak securely controls realm signing keys, token claims, client configuration, and administrator access.
- Forgejo tokens have the least privileges needed and Forgejo ACLs are correctly maintained.
- The gateway host, service configuration, principal map, approval store, and secret injection path are accessible only to authorized operators and the service account.
- Public traffic uses authenticated TLS even when the daemon listens on loopback HTTP behind a reverse proxy.
- Clock synchronization is sufficient for token and approval expiry decisions.
- Durable audit storage, rotation, backup, alerting, and off-host retention are configured by the operator.
- Callers do not treat dry-run output as proof that a later mutation will still be authorized or succeed.

## Out Of Scope

- Vulnerabilities inside Keycloak, Forgejo, the operating system, reverse proxy, container runtime, or network fabric, except where the gateway can reduce exposure.
- Recovery of a fully compromised gateway host or Forgejo/Keycloak administrator account.
- Generic execution of the complete generated Forgejo API.
- Protection against a maintainer who can modify source, CI, release credentials, and signing keys without review.
- Tenant isolation within a Forgejo instance beyond Forgejo's own ACL model.

## Deployment And Review Requirements

Operators must use HTTPS public URLs, keep the daemon bind private, apply proxy request and rate limits, restrict all local state files, use short-lived Keycloak tokens, use least-privilege Forgejo principals, enable durable audit export, monitor audit write failures, and follow the documented credential and JWKS rotation procedures.

Review this model whenever an executable operation is added, a trust boundary or credential flow changes, token validation or approval semantics change, audit behavior changes, a new deployment topology is supported, or a security incident invalidates an assumption. Security reports follow [SECURITY.md](../SECURITY.md).
