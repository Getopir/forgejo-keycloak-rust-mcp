# Credential Rotation And Incident Response

This runbook covers credentials used to authenticate agents, delegate Forgejo
operations, and publish releases. Never copy a live secret into source control,
logs, issue comments, wiki pages, approval records, or incident tickets.

## Credential Inventory

Maintain an owner, purpose, scope, storage location, issue date, and next
rotation date for each of these credential classes:

- Keycloak client credentials used by agent token brokers.
- Keycloak realm signing keys used to sign gateway access tokens.
- Per-principal Forgejo tokens referenced by `token_env` in the principal map.
- Codeberg and crates.io credentials used by the release process.

The principal map stores environment-variable names, never Forgejo token
values. Production values belong in the approved runtime secret store and are
materialized into the service environment only for the process that needs them.

## Planned Rotation

1. Open a change record containing the owner, credential reference, reason,
   scope, and start time. Do not include the credential value.
2. Create the replacement with the minimum scopes and shortest practical
   lifetime. Keep the previous credential active only for a bounded overlap.
3. Update the approved secret store and service environment, then restart or
   reload the affected process.
4. Verify `/health` and `/capabilities`, then run one authorized, read-only MCP
   operation as the affected mapped principal. Confirm the audit event names
   the expected subject and operation without containing secret material.
5. Revoke the previous credential and repeat the authorized check. Record the
   revocation time and evidence in the change record.

For a mapped Forgejo token, preserve the existing `token_env` name where
possible and replace only its secret-store value. If the environment-variable
name must change, update the principal map and runtime environment atomically.
Verify both a permitted read and an intentionally denied operation after the
restart.

For a Keycloak client credential, update every authorized token broker before
revoking the previous client secret. Verify the resulting access token has the
expected issuer and audience and that the gateway accepts it. The gateway does
not need the token broker's client secret.

For a Keycloak realm signing key, publish the new public key in JWKS before
issuing tokens with it. Retain the previous public key until every token signed
by it has expired. A suspected signing-key compromise does not use this overlap
procedure; follow incident response and revoke affected sessions immediately.

Release credentials must be rotated independently. Confirm Codeberg access
with a non-mutating repository read and crates.io access using the registry's
credential-management controls. Do not use a publication as an authentication
test.

## Incident Response

1. Contain the incident. Revoke the affected Forgejo, Keycloak, Codeberg, or
   crates.io credential; disable the mapped principal or token broker when its
   scope is uncertain. Stop the gateway only when continued operation could
   extend the exposure.
2. Preserve evidence without secrets. Record the time window, Keycloak subject,
   mapped Forgejo account, operation, target, approval ID, audit-event ID, and
   relevant service or provider log references. Restrict evidence access.
3. Determine scope. Review gateway audit records, Forgejo activity, Keycloak
   session and client events, approval-store activity, and release-provider
   events. Treat credentials found in repositories, package archives, logs,
   prompts, browser history, or untrusted clients as compromised.
4. Replace affected credentials using the planned-rotation procedure, without
   retaining an overlap for a compromised credential. Rotate dependent
   credentials when their isolation cannot be proven.
5. Recover with a newly issued Keycloak token. Verify health and capabilities,
   an authorized read, an expected denial, Forgejo audit readback, and the
   absence of secret values in logs.
6. Close the incident with root cause, affected scope, containment time,
   revocation time, recovery evidence, and follow-up actions. Never include
   secret values in the incident record.

If a secret reached Git history or a published crate, revocation is mandatory;
deleting the visible file is not sufficient. Coordinate history cleanup where
appropriate, but assume every fetched copy remains exposed. A crates.io release
cannot be overwritten, so yank an affected version when necessary and publish
a clean patch version after credential revocation.

## Recovery Checks

Run the repository checks before redeployment:

```sh
cargo fmt --check
cargo check --workspace
cargo test --workspace
reuse lint
```

At runtime, verify the public health and capability routes and use a short-lived
Keycloak access token for the authenticated read. Do not paste that token into
the incident record or shell history. Rotation or recovery is complete only
after the replaced credential is revoked and the new credential succeeds.
