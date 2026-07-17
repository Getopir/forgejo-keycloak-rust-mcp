# JWKS Cache Limits And Key Rotation

## Current Cache Behaviour

`forgejo-keycloak-rust-mcpd` fetches the OIDC discovery document and the referenced JWKS during startup. The resulting key set is held in process memory for the lifetime of that daemon instance.

Current limits are explicit:

- No cache TTL or scheduled refresh.
- No configured maximum key count or JWKS response-size limit beyond the HTTP client and available memory.
- No refresh when a JWT contains an unknown `kid`.
- No stale-key fallback or retry loop after startup.
- A discovery or JWKS fetch failure prevents startup.

Signing-key policy is enforced before startup completes:

- RSA signing keys must be at least 2048 bits.
- Allowed RSA algorithms are RS256/384/512 and PS256/384/512.
- ES256 requires P-256, ES384 requires P-384, and EdDSA requires Ed25519.
- Symmetric JWT keys, missing algorithms or key IDs, duplicate signing key IDs,
  mismatched algorithm/key types, weak RSA keys, and unsupported curves are
  rejected.
- Encryption-only keys may remain in the JWKS but cannot validate access tokens.

An unknown `kid` is rejected as unauthorized until the daemon restarts with a JWKS containing that key. Removing an old key from Keycloak does not remove it from an already-running daemon's in-memory set; restart the daemon after the retirement window to stop accepting that key.

## Safe Rotation Procedure

1. Add the new signing key to Keycloak's published JWKS before using it to sign access tokens.
2. Confirm the new `kid` is present at the issuer's `jwks_uri` from the gateway network.
3. Confirm the new key satisfies the signing-key policy, then restart every
   gateway instance so each validates and loads the expanded key set.
4. Confirm health and validate a token signed by the new key against every gateway instance.
5. Switch Keycloak to sign new tokens with the new key.
6. Keep the old public key published for at least the maximum access-token lifetime plus allowed clock skew and deployment propagation time.
7. Remove or disable the old signing key in Keycloak only after that overlap window.
8. Restart every gateway instance again so the retired key is removed from memory, then verify new-key tokens still work and old-key tokens are rejected.

If a key is compromised, stop issuing with it immediately, revoke or shorten affected sessions where the deployment supports that action, publish a replacement, and restart all gateway instances. Availability may need to yield to containment because the current validator cannot refresh keys without restart. Follow the broader [Credential Rotation And Incident Response](credential-rotation-and-incident-response.md) procedure and preserve audit records from the incident window.
