# JWKS Cache Limits And Key Rotation

## Current Behaviour

The gateway fetches OIDC discovery and JWKS data once at startup and holds that key set in memory for the process lifetime.

- There is no cache TTL, scheduled refresh, refresh on an unknown `kid`, stale-key fallback, or startup retry loop.
- There is no configured maximum key count or JWKS response-size limit beyond the HTTP client and available memory.
- Discovery or JWKS fetch failure prevents startup.
- A token with an unknown `kid` is unauthorized until the gateway restarts with that key present.
- A removed key remains accepted by an already-running gateway until restart.

## Rotation

1. Publish the new public key in Keycloak JWKS before signing tokens with it.
2. Verify the new `kid` from the gateway network.
3. Restart every gateway instance and test a token signed with the new key.
4. Switch Keycloak signing to the new key.
5. Keep the old public key published for the maximum access-token lifetime plus clock skew and propagation time.
6. Retire the old key, restart every gateway again, and verify the retired key is rejected.

For a suspected compromise, stop issuing with the key immediately, replace it, restart every gateway, and preserve the durable audit log. See [Credential Rotation And Incident Response](Credential-Rotation-And-Incident-Response.md).
