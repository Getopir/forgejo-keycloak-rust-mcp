# Codeberg Release: v0.11.0

Tag: `v0.11.0`

Title: `v0.11.0 - Generated Forgejo API classification coverage`

## Summary

`v0.11.0` starts Phase 3 by pinning the Forgejo API metadata and generating a
safe classification catalog. This release does not add generic endpoint
forwarding; it gives humans and agents a bounded way to inspect which Forgejo
API operations exist, how they are classified, and which ones are currently
exposed through reviewed semantic MCP tools.

## Highlights

- Pins the Forgejo `15.0.3+gitea-1.22.0` Swagger document with SHA-256
  provenance.
- Classifies all 491 pinned Forgejo API operations by risk, target type,
  approval requirement, and exposure.
- Adds `forgejo_api_coverage` as a metadata-only MCP operation.
- Adds `forgejo-mcpctl api-coverage` for CLI readback.
- Keeps 482 non-reviewed generated endpoints disabled.

## Safety Notes

Generated coverage is not generic API execution. Destructive, admin,
secret-bearing, network-egress, and unknown endpoints remain unavailable unless
future releases add a reviewed semantic operation with explicit policy,
approval, output limits, and audit behavior.
