# Security Policy

## Supported Version

`1.2.7` is the current supported public release.

## Reporting

Report vulnerabilities privately to the project maintainers before public disclosure.

Do not include live bearer tokens, Forgejo tokens, Keycloak client secrets, private keys, or production host credentials in reports. Use redacted examples and share sensitive material only through an agreed secure channel.

Use the [credential-rotation and incident-response runbook](docs/credential-rotation-and-incident-response.md)
for containment, rotation, recovery, and evidence requirements.

The maintained [threat model](docs/threat-model.md) documents protected assets, trust boundaries, attacker profiles, controls, residual risks, deployment assumptions, and review triggers.

## Secret Handling

This repository must not contain:

- Access tokens or refresh tokens.
- Keycloak client secrets.
- Forgejo personal access tokens.
- Private SSH keys.
- Internal infrastructure credentials.
- Production-only hostnames, IPs, or project IDs.
