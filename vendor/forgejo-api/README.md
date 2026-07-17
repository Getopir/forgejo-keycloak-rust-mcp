# Forgejo API Spec Pin

This directory contains the pinned Forgejo OpenAPI document used for generated
endpoint classification and coverage reporting.

## Current Pin

- Source tag: `https://codeberg.org/forgejo/forgejo/src/tag/v16.0.0`
- Source commit: `07ba27a02b8db81be78e336f8597a355c04adb18`
- OpenAPI template: `templates/swagger/v1_json.tmpl`
- Rendered version: `16.0.0`
- Rendered base path: `/api/v1`
- Pinned file: `forgejo-16.0.0-swagger.v1.json`
- SHA-256: `a41f976f1d616e273c0a1855a625928e59e758f324f0b02fc247a25a5469be84`
- Retrieved: `2026-07-17`

Forgejo documents the Swagger UI at `/api/swagger` and the OpenAPI document at
`/swagger.v1.json`.

## Refresh Rule

When refreshing this pin:

1. Resolve and record the intended Forgejo release tag and commit.
2. Render the release's `templates/swagger/v1_json.tmpl` with the release
   version and an empty application sub-URL.
3. Update this file with the version, source URL, retrieval date, byte-for-byte
   filename, and SHA-256.
4. Run `python tools/generate_forgejo_api_catalog.py`.
5. Review classification changes before exposing any endpoint as an MCP tool.

Unknown endpoints are not executable. The generator is allowed to classify them
for review and reporting, but generated execution must stay blocked unless a
human-reviewed classification exists.
