# Security Policy

## Supported Version

`0.5.0` is the current public beta release.

## Reporting

Report vulnerabilities privately to the project maintainers before public disclosure.

Do not include live bearer tokens, Forgejo tokens, Keycloak client secrets, private keys, or production host credentials in reports. Use redacted examples and share sensitive material only through an agreed secure channel.

## Secret Handling

This repository must not contain:

- Access tokens or refresh tokens.
- Keycloak client secrets.
- Forgejo personal access tokens.
- Private SSH keys.
- Internal infrastructure credentials.
- Production-only hostnames, IPs, or project IDs.
