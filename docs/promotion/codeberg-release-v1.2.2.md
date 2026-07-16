# Codeberg Release Text - v1.2.2

Title: `v1.2.2 - REUSE and security operations maintenance`

`v1.2.2` completes repository licensing attribution and adds practical
credential-rotation and incident-response guidance. The release does not
expand the MCP gateway's executable Forgejo capability surface.

## Highlights

- REUSE 3.3-compliant copyright and license metadata for maintained files.
- Separate MIT attribution for the vendored Forgejo API specification.
- Rotation procedures for Forgejo, Keycloak, and release credentials.
- Incident containment, evidence, recovery, and verification procedures.
- Updated source-controlled and hosted Codeberg wiki material.

## Install

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.2.2 --locked
```
