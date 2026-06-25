# Codeberg Release: v1.0.0

Tag: `v1.0.0`

Title: `v1.0.0 - Stable Forgejo Keycloak Rust MCP gateway`

## Summary

`v1.0.0` is the first stable public release of the Forgejo Keycloak Rust MCP
gateway. It combines Keycloak identity, explicit policy checks, Forgejo
principal mapping, curated MCP tools, approval-gated high-risk actions, and
generated Forgejo API classification coverage.

## Highlights

- Keycloak-authenticated MCP gateway for Forgejo agents.
- Explicit operation policy registry with scopes, risk classes, and approval
  requirements.
- Curated bounded tools for repositories, issues, pull requests, reviews,
  releases, notifications, and API coverage.
- File-backed, short-lived, exact-payload-bound approval records.
- Approval-backed pull-request merge and release creation.
- Generated classification of all 491 operations in the pinned Forgejo
  `15.0.3+gitea-1.22.0` Swagger document.
- `forgejo-mcpctl` CLI for shell-based human and agent workflows.

## Safety Notes

This release does not expose generic Forgejo API forwarding. Admin,
destructive, secret-bearing, network-egress, and unreviewed generated endpoints
remain disabled unless future releases add reviewed semantic operations with
explicit policy, approval, output limits, and audit behavior.
