# Security Model

Security layers:

1. Keycloak authenticates the caller.
2. The Rust gateway validates issuer, audience, signature, and expiry.
3. The policy registry checks the operation scope and risk class.
4. Forgejo ACL enforcement is the final authority for future repository operations.

The gateway rejects missing tokens, invalid tokens, wrong-audience tokens, unknown operations, and valid tokens that lack required scopes.
