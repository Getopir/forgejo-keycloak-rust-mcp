# Codeberg Release: v0.8.0

Tag: `v0.8.0`

Title: `v0.8.0 - Phase 2 approval-store hardening`

Body:

```markdown
`v0.8.0` hardens the Phase 2 approval gate for high-risk Forgejo operations.

Highlights:

- Adds `create_approval`, protected by `forgejo:approval:grant`.
- Adds a file-backed append-only JSONL approval store.
- Adds configurable approval TTL with a 900 second default.
- Binds approvals to the exact operation, target, state, request body hash, Keycloak principal, OAuth client, and mapped Forgejo account.
- Rejects fake, expired, mismatched, revoked, or wrong-principal approval IDs.

Important:

- High-risk operations still do not execute in this release.
- A valid approval now proves the gate can validate authority and exact payload before future mutation tools are added.
- Store approval files outside the repository and restrict them to the gateway service account.
```
