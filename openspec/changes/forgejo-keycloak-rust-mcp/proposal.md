# Forgejo Keycloak Rust MCP

## Why

Agents need structured Forgejo access without sharing a process-wide Forgejo token. Keycloak must authenticate humans and agents, the Rust gateway must enforce operation-class policy, and Forgejo must remain the final authority for repository and organization permissions.

## What Changes

- Build a clean-room Rust MCP gateway, not a port of Sqcows or goern.
- Implement Keycloak JWT validation for a dedicated MCP audience.
- Map immutable Keycloak subjects to Forgejo accounts before executing Forgejo operations.
- Add a policy registry for operation scope, risk, approval, output limits, egress limits, and admin separation.
- Use trusted reverse-proxy Forgejo API delegation as the preferred backend, with sudo/PAT fallback interfaces isolated and disabled by default.
- Provide curated MCP tools first, then generated full API coverage after every endpoint is classified.
- Provide deployment packaging for a blank Forgejo VM lab using local Keycloak.

## Source Inputs

- `docs/original-inputs/recommended-direction.md`
- Sqcows as behavioral API coverage checklist only.
- goern as behavioral MCP surface checklist only.
- Local Forgejo, Keycloak, OPIR-O, and Neutrino VM readbacks recorded in `docs/operations/live-readback-2026-06-25.md`.

## Acceptance

- Unauthenticated `/mcp` requests return `401`.
- Valid Keycloak tokens with the wrong audience are rejected.
- Incoming caller-supplied identity headers do not influence the mapped Forgejo user.
- The daemon never falls back to a shared Forgejo token for remote callers.
- Operation policy denies missing scopes before any Forgejo call.
- Every exposed operation has a risk class, required scope, output budget, and approval policy.
- Audit records contain issuer, subject, OAuth client, operation, target, risk, decision, and no tokens or secrets.
- The lab VM proves Keycloak human or agent token to Rust policy to mapped Forgejo ACL to audit event.
