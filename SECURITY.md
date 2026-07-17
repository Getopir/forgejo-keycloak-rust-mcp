# Security Policy

## Supported Version

`1.2.10` is the current supported public release.

## Reporting

Report suspected vulnerabilities by email to the monitored project role address
[`info@getopir.com`](mailto:info@getopir.com) with the subject
`[forgejo-keycloak-rust-mcp security]`. This address avoids publishing a
maintainer's personal email address.

Do not open a public issue, pull request, discussion, or chat thread for an
undisclosed vulnerability. The initial report should include:

- The affected version or commit.
- The expected and observed behavior.
- Reproduction steps or a minimal proof of concept.
- The likely impact and affected deployment assumptions.
- Any known workaround or suggested remediation.
- A reply address and whether the issue has been disclosed elsewhere.

Do not include live bearer tokens, Forgejo tokens, Keycloak client secrets,
private keys, production host credentials, or personal data. Use redacted
examples. The project will acknowledge a report within seven calendar days. If
additional sensitive evidence is required, the security maintainer will agree
on a separate encrypted transfer method with the reporter before it is sent.

## Private Handling And Disclosure

Reports sent to the role address are handled as follows:

1. Access is limited to authorized GetOPIR maintainers responsible for security
   triage. Reports are not copied into public issue trackers or repositories.
2. Internal coordination uses a case identifier and redacted summaries. Full
   exploit details and credentials are shared only with people needed to
   investigate or remediate the issue.
3. The maintainer confirms severity, affected versions, containment needs, and
   a remediation owner. Reporters receive material status updates at least every
   14 calendar days while the case remains open.
4. Public disclosure is coordinated with the reporter after a fix or mitigation
   is available. Immediate disclosure may occur when active exploitation or
   user protection requires it, but secrets and personal data remain redacted.
5. The private report and investigation record are retained for 12 months after
   closure for incident follow-up, then deleted unless a legal, contractual, or
   active-incident requirement requires longer retention.

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
