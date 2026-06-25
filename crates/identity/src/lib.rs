use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

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
        Ok(Self {
            issuer,
            audience,
            jwks,
        })
    }

    pub fn from_jwks(issuer: impl Into<String>, audience: impl Into<String>, jwks: JwkSet) -> Self {
        Self {
            issuer: issuer.into().trim_end_matches('/').to_string(),
            audience: audience.into(),
            jwks,
        }
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
        let key = DecodingKey::from_jwk(jwk).map_err(|e| IdentityError::Jwt(e.to_string()))?;

        let mut validation = Validation::new(header.alg);
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
}
