# Credential Rotation And Incident Response

This runbook covers Keycloak, mapped Forgejo, Codeberg, and crates.io
credentials. Never put a live secret in source control, logs, issues, wiki
pages, approval records, or incident tickets.

## Planned Rotation

1. Record the owner, credential reference, reason, scope, and start time without
   recording the secret value.
2. Create a least-privilege replacement and keep the old credential active only
   for a bounded deployment overlap.
3. Update the approved secret store and affected service environment, then
   restart or reload the process.
4. Verify gateway health and capabilities plus one authorized read-only MCP
   operation for the affected principal. Confirm audit output contains no
   secret values.
5. Revoke the old credential, repeat the authorized check, and record evidence.

Mapped Forgejo tokens are referenced by `token_env`; the principal map must not
contain token values. Preserve the environment-variable name when possible and
replace its value in the runtime secret store.

Publish a new Keycloak signing key in JWKS before using it. Keep the previous
public key available until its tokens expire. If compromise is suspected,
revoke affected sessions immediately instead of using an overlap.

Rotate Codeberg and crates.io release credentials independently. Verify access
with non-mutating account or repository checks; never publish solely to test a
credential.

## Incident Response

1. Revoke or disable the affected credential, client, session, or mapped
   principal. Stop the gateway only when continued operation extends exposure.
2. Preserve token-free evidence: time window, Keycloak subject, Forgejo account,
   operation, target, approval ID, audit-event ID, and restricted log references.
3. Review gateway audit records, Forgejo activity, Keycloak events, approval
   activity, and release-provider events to determine scope.
4. Replace affected and dependent credentials without overlap when isolation
   cannot be proven.
5. Verify health, capabilities, an authorized read, an expected denial, audit
   readback, and the absence of secrets in logs.
6. Record root cause, scope, containment, revocation, recovery evidence, and
   follow-up actions without credential values.

If a secret reached Git history or a published crate, revoke it immediately.
Deleting the visible file is insufficient. A crates.io version cannot be
overwritten; yank it if necessary and publish a clean patch release.

The repository source contains the full operator procedure in
`docs/credential-rotation-and-incident-response.md`.
