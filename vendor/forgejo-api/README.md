# Forgejo API Spec Pin

This directory contains the pinned Forgejo OpenAPI document used for generated
endpoint classification and coverage reporting.

## Current Pin

- Source instance: `http://192.168.87.91:3000`
- Version endpoint: `http://192.168.87.91:3000/api/v1/version`
- Version response: `15.0.3+gitea-1.22.0`
- OpenAPI source URL: `http://192.168.87.91:3000/swagger.v1.json`
- Pinned file: `forgejo-15.0.3-gitea-1.22.0-swagger.v1.json`
- SHA-256: `a90f2fe1266a7a08dfcf682cd28db96c364e18a7de2a4e559a26afe3485bb26f`
- Retrieved: `2026-06-25`

Forgejo documents the Swagger UI at `/api/swagger` and the OpenAPI document at
`/swagger.v1.json`.

## Refresh Rule

When refreshing this pin:

1. Read `/api/v1/version` from the target Forgejo instance.
2. Download `/swagger.v1.json`.
3. Update this file with the version, source URL, retrieval date, byte-for-byte
   filename, and SHA-256.
4. Run `python tools/generate_forgejo_api_catalog.py`.
5. Review classification changes before exposing any endpoint as an MCP tool.

Unknown endpoints are not executable. The generator is allowed to classify them
for review and reporting, but generated execution must stay blocked unless a
human-reviewed classification exists.
