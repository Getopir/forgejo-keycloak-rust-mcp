// SPDX-License-Identifier: AGPL-3.0-or-later

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use jsonwebtoken::jwk::{
    AlgorithmParameters, EllipticCurve, Jwk, JwkSet, KeyAlgorithm, KeyOperations, PublicKeyUse,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

const MIN_RSA_BITS: usize = 2048;

#[derive(Debug, Clone)]
pub struct JwtValidator {
    issuer: String,
    audience: String,
    jwks: JwkSet,
}

#[derive(Debug, Clone, Deserialize)]
struct OidcDiscovery {
    jwks_uri: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrincipalClaims {
    pub iss: String,
    pub sub: String,
    #[serde(default)]
    pub aud: Value,
    pub exp: u64,
    #[serde(default)]
    pub nbf: Option<u64>,
    #[serde(default)]
    pub iat: Option<u64>,
    #[serde(default)]
    pub jti: Option<String>,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub azp: Option<String>,
    #[serde(default)]
    pub client_id: Option<String>,
    #[serde(default)]
    pub preferred_username: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Principal {
    pub issuer: String,
    pub subject: String,
    pub oauth_client: Option<String>,
    pub scopes: BTreeSet<String>,
    pub preferred_username: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("missing bearer token")]
    MissingBearer,
    #[error("invalid authorization header")]
    InvalidAuthorizationHeader,
    #[error("OIDC discovery failed: {0}")]
    Discovery(String),
    #[error("JWT header has no key id")]
    MissingKid,
    #[error("JWT key id is unknown")]
    UnknownKid,
    #[error("JWT validation failed: {0}")]
    Jwt(String),
    #[error("OIDC signing keys are invalid: {0}")]
    InvalidJwks(String),
    #[error("JWT algorithm is not allowed")]
    UnsupportedJwtAlgorithm,
    #[error("JWT header algorithm does not match the signing key")]
    JwtAlgorithmMismatch,
    #[error("issuer mismatch")]
    IssuerMismatch,
    #[error("audience mismatch")]
    AudienceMismatch,
}

impl JwtValidator {
    pub async fn discover(
        issuer: impl Into<String>,
        audience: impl Into<String>,
    ) -> Result<Self, IdentityError> {
        let issuer = issuer.into().trim_end_matches('/').to_string();
        let discovery_url = format!("{issuer}/.well-known/openid-configuration");
        Self::discover_with_url(issuer, discovery_url, audience).await
    }

    pub async fn discover_with_url(
        issuer: impl Into<String>,
        discovery_url: impl Into<String>,
        audience: impl Into<String>,
    ) -> Result<Self, IdentityError> {
        let issuer = issuer.into().trim_end_matches('/').to_string();
        let audience = audience.into();
        let discovery_url = discovery_url.into();
        let discovery: OidcDiscovery = reqwest::get(&discovery_url)
            .await
            .map_err(|e| IdentityError::Discovery(e.to_string()))?
            .error_for_status()
            .map_err(|e| IdentityError::Discovery(e.to_string()))?
            .json()
            .await
            .map_err(|e| IdentityError::Discovery(e.to_string()))?;
        let jwks: JwkSet = reqwest::get(&discovery.jwks_uri)
            .await
            .map_err(|e| IdentityError::Discovery(e.to_string()))?
            .error_for_status()
            .map_err(|e| IdentityError::Discovery(e.to_string()))?
            .json()
            .await
            .map_err(|e| IdentityError::Discovery(e.to_string()))?;
        validate_jwks(&jwks)?;
        Ok(Self {
            issuer,
            audience,
            jwks,
        })
    }

    pub fn from_jwks(issuer: impl Into<String>, audience: impl Into<String>, jwks: JwkSet) -> Self {
        Self::try_from_jwks(issuer, audience, jwks)
            .expect("OIDC signing keys must satisfy the enforced policy")
    }

    pub fn try_from_jwks(
        issuer: impl Into<String>,
        audience: impl Into<String>,
        jwks: JwkSet,
    ) -> Result<Self, IdentityError> {
        validate_jwks(&jwks)?;
        Ok(Self {
            issuer: issuer.into().trim_end_matches('/').to_string(),
            audience: audience.into(),
            jwks,
        })
    }

    pub fn validate_authorization_header(
        &self,
        value: Option<&str>,
    ) -> Result<Principal, IdentityError> {
        let token = bearer_token(value)?;
        self.validate_token(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<Principal, IdentityError> {
        let header = decode_header(token).map_err(|e| IdentityError::Jwt(e.to_string()))?;
        let kid = header.kid.ok_or(IdentityError::MissingKid)?;
        let jwk = self.jwks.find(&kid).ok_or(IdentityError::UnknownKid)?;
        let key_algorithm = validate_signing_jwk(jwk)?;
        if header.alg != key_algorithm {
            return Err(IdentityError::JwtAlgorithmMismatch);
        }
        let key = DecodingKey::from_jwk(jwk).map_err(|e| IdentityError::Jwt(e.to_string()))?;

        let mut validation = Validation::new(key_algorithm);
        // Keycloak may emit `aud` as either a string or an array depending on
        // client configuration, so audience matching is performed explicitly
        // after signature, issuer, expiry, and required-claim validation.
        validation.validate_aud = false;
        validation.set_required_spec_claims(&["exp", "iss", "sub"]);

        let data = decode::<PrincipalClaims>(token, &key, &validation)
            .map_err(|e| IdentityError::Jwt(e.to_string()))?;
        let claims = data.claims;
        if claims.iss.trim_end_matches('/') != self.issuer {
            return Err(IdentityError::IssuerMismatch);
        }
        if !audience_contains(&claims.aud, &self.audience) {
            return Err(IdentityError::AudienceMismatch);
        }
        let scopes = claims
            .scope
            .as_deref()
            .unwrap_or_default()
            .split_whitespace()
            .map(ToOwned::to_owned)
            .collect();
        Ok(Principal {
            issuer: claims.iss,
            subject: claims.sub,
            oauth_client: claims.azp.or(claims.client_id),
            scopes,
            preferred_username: claims.preferred_username,
        })
    }
}

fn validate_jwks(jwks: &JwkSet) -> Result<(), IdentityError> {
    let mut key_ids = BTreeSet::new();
    let mut signing_key_count = 0usize;

    for jwk in &jwks.keys {
        if !is_signature_key(jwk) {
            continue;
        }
        signing_key_count += 1;
        let key_id = jwk
            .common
            .key_id
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| IdentityError::InvalidJwks("signing key has no kid".to_string()))?;
        if !key_ids.insert(key_id) {
            return Err(IdentityError::InvalidJwks(format!(
                "duplicate signing key id: {key_id}"
            )));
        }
        validate_signing_jwk(jwk)?;
    }

    if signing_key_count == 0 {
        return Err(IdentityError::InvalidJwks(
            "no supported signing keys were published".to_string(),
        ));
    }
    Ok(())
}

fn is_signature_key(jwk: &Jwk) -> bool {
    if matches!(jwk.common.public_key_use, Some(PublicKeyUse::Encryption)) {
        return false;
    }
    match jwk.common.key_operations.as_ref() {
        Some(operations) => operations.contains(&KeyOperations::Verify),
        None => true,
    }
}

fn validate_signing_jwk(jwk: &Jwk) -> Result<Algorithm, IdentityError> {
    let algorithm = jwk
        .common
        .key_algorithm
        .ok_or_else(|| IdentityError::InvalidJwks("signing key has no alg".to_string()))?;

    match (&jwk.algorithm, algorithm) {
        (AlgorithmParameters::RSA(parameters), KeyAlgorithm::RS256) => {
            validate_rsa_modulus(&parameters.n, Algorithm::RS256)
        }
        (AlgorithmParameters::RSA(parameters), KeyAlgorithm::RS384) => {
            validate_rsa_modulus(&parameters.n, Algorithm::RS384)
        }
        (AlgorithmParameters::RSA(parameters), KeyAlgorithm::RS512) => {
            validate_rsa_modulus(&parameters.n, Algorithm::RS512)
        }
        (AlgorithmParameters::RSA(parameters), KeyAlgorithm::PS256) => {
            validate_rsa_modulus(&parameters.n, Algorithm::PS256)
        }
        (AlgorithmParameters::RSA(parameters), KeyAlgorithm::PS384) => {
            validate_rsa_modulus(&parameters.n, Algorithm::PS384)
        }
        (AlgorithmParameters::RSA(parameters), KeyAlgorithm::PS512) => {
            validate_rsa_modulus(&parameters.n, Algorithm::PS512)
        }
        (AlgorithmParameters::EllipticCurve(parameters), KeyAlgorithm::ES256)
            if parameters.curve == EllipticCurve::P256 =>
        {
            Ok(Algorithm::ES256)
        }
        (AlgorithmParameters::EllipticCurve(parameters), KeyAlgorithm::ES384)
            if parameters.curve == EllipticCurve::P384 =>
        {
            Ok(Algorithm::ES384)
        }
        (AlgorithmParameters::OctetKeyPair(parameters), KeyAlgorithm::EdDSA)
            if parameters.curve == EllipticCurve::Ed25519 =>
        {
            Ok(Algorithm::EdDSA)
        }
        _ => Err(IdentityError::UnsupportedJwtAlgorithm),
    }
}

fn validate_rsa_modulus(
    encoded_modulus: &str,
    algorithm: Algorithm,
) -> Result<Algorithm, IdentityError> {
    let modulus = URL_SAFE_NO_PAD
        .decode(encoded_modulus)
        .map_err(|_| IdentityError::InvalidJwks("RSA modulus is not base64url".to_string()))?;
    let bits = bit_length(&modulus);
    if bits < MIN_RSA_BITS {
        return Err(IdentityError::InvalidJwks(format!(
            "RSA signing key is {bits} bits; minimum is {MIN_RSA_BITS}"
        )));
    }
    Ok(algorithm)
}

fn bit_length(bytes: &[u8]) -> usize {
    let Some(first_nonzero) = bytes.iter().position(|byte| *byte != 0) else {
        return 0;
    };
    let significant = &bytes[first_nonzero..];
    significant.len() * 8 - significant[0].leading_zeros() as usize
}

pub fn bearer_token(value: Option<&str>) -> Result<&str, IdentityError> {
    let value = value.ok_or(IdentityError::MissingBearer)?.trim();
    let (scheme, token) = value
        .split_once(' ')
        .ok_or(IdentityError::InvalidAuthorizationHeader)?;
    if !scheme.eq_ignore_ascii_case("bearer") || token.trim().is_empty() {
        return Err(IdentityError::InvalidAuthorizationHeader);
    }
    Ok(token.trim())
}

pub fn audience_contains(aud: &Value, expected: &str) -> bool {
    match aud {
        Value::String(value) => value == expected,
        Value::Array(values) => values.iter().any(|value| value.as_str() == Some(expected)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audience_accepts_string_or_array() {
        assert!(audience_contains(&Value::String("mcp".to_string()), "mcp"));
        assert!(audience_contains(
            &serde_json::json!(["account", "mcp"]),
            "mcp"
        ));
        assert!(!audience_contains(&serde_json::json!(["account"]), "mcp"));
    }

    #[test]
    fn bearer_header_is_strict() {
        assert_eq!(bearer_token(Some("Bearer abc")).unwrap(), "abc");
        assert!(bearer_token(Some("Basic abc")).is_err());
        assert!(bearer_token(Some("Bearer ")).is_err());
    }

    #[test]
    fn accepts_rsa_signing_keys_at_or_above_2048_bits() {
        let jwks = rsa_jwks("RS256", 2048);
        assert!(validate_jwks(&jwks).is_ok());
        let validator = JwtValidator::from_jwks("https://issuer.example", "mcp", jwks);
        assert_eq!(validator.issuer, "https://issuer.example");
    }

    #[test]
    fn rejects_rsa_signing_keys_below_2048_bits() {
        let jwks = rsa_jwks("RS256", 1024);
        let error = JwtValidator::try_from_jwks("https://issuer.example", "mcp", jwks)
            .expect_err("weak signing keys must be rejected");
        assert!(error.to_string().contains("minimum is 2048"));
    }

    #[test]
    fn rejects_symmetric_or_undeclared_signing_algorithms() {
        let symmetric = serde_json::from_value(serde_json::json!({
            "keys": [{"kty": "oct", "kid": "shared", "alg": "HS256", "use": "sig", "k": "c2VjcmV0"}]
        }))
        .unwrap();
        assert!(matches!(
            validate_jwks(&symmetric),
            Err(IdentityError::UnsupportedJwtAlgorithm)
        ));

        let mut undeclared = rsa_jwks("RS256", 2048);
        undeclared.keys[0].common.key_algorithm = None;
        assert!(validate_jwks(&undeclared).is_err());
    }

    #[test]
    fn accepts_only_matching_approved_elliptic_curves() {
        let p256: JwkSet = serde_json::from_value(serde_json::json!({
            "keys": [{
                "kty": "EC", "kid": "p256", "alg": "ES256", "use": "sig",
                "crv": "P-256", "x": "AA", "y": "AA"
            }]
        }))
        .unwrap();
        assert!(validate_jwks(&p256).is_ok());

        let mismatch: JwkSet = serde_json::from_value(serde_json::json!({
            "keys": [{
                "kty": "EC", "kid": "mismatch", "alg": "ES256", "use": "sig",
                "crv": "P-384", "x": "AA", "y": "AA"
            }]
        }))
        .unwrap();
        assert!(matches!(
            validate_jwks(&mismatch),
            Err(IdentityError::UnsupportedJwtAlgorithm)
        ));
    }

    #[test]
    fn ignores_encryption_keys_but_requires_a_valid_signing_key() {
        let mut mixed = rsa_jwks("RS256", 2048);
        let encryption_key: Jwk = serde_json::from_value(serde_json::json!({
            "kty": "RSA", "kid": "encryption", "alg": "RSA-OAEP", "use": "enc",
            "n": URL_SAFE_NO_PAD.encode(vec![0x80; 256]), "e": "AQAB"
        }))
        .unwrap();
        mixed.keys.push(encryption_key.clone());
        assert!(validate_jwks(&mixed).is_ok());
        assert!(
            validate_jwks(&JwkSet {
                keys: vec![encryption_key]
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_duplicate_signing_key_ids() {
        let mut jwks = rsa_jwks("RS256", 2048);
        jwks.keys.push(jwks.keys[0].clone());
        let error = validate_jwks(&jwks).unwrap_err();
        assert!(error.to_string().contains("duplicate signing key id"));
    }

    fn rsa_jwks(algorithm: &str, bits: usize) -> JwkSet {
        serde_json::from_value(serde_json::json!({
            "keys": [{
                "kty": "RSA",
                "kid": "rsa-signing",
                "alg": algorithm,
                "use": "sig",
                "n": URL_SAFE_NO_PAD.encode(rsa_modulus(bits)),
                "e": "AQAB"
            }]
        }))
        .unwrap()
    }

    fn rsa_modulus(bits: usize) -> Vec<u8> {
        let mut modulus = vec![0; bits.div_ceil(8)];
        modulus[0] = 0x80;
        modulus
    }
}
