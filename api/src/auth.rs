use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt,
};

use std::fmt::{Debug, Formatter};
// use axum::extract::State;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use platform::hyper::{ContentType, Method, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

use crate::Error;

/// Represents an OIDC claim appropriate for runtime authentication and authorization.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iss: String,
    client_id: String,
    token_use: String,
    scope: String,
    auth_time: u64,
    exp: u64,
    iat: u64,
    jti: String,
    username: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _authenticator: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| Error::InvalidToken(e.to_string()))?;

        let keys = keys().await;
        let mut validation = Validation::new(Algorithm::RS256);
        let required = required_spec_claims();
        validation.set_required_spec_claims(required);

        let token_data = decode::<Claims>(bearer.token(), &keys.decoding, &validation)
            .map_err(|e| Error::InvalidToken(e.to_string()))?;

        Ok(token_data.claims)
    }
}

static KEYS: OnceCell<Keys> = OnceCell::const_new();
/// Gets the keys for decoding JWT tokens.
pub async fn keys() -> &'static Keys {
    KEYS.get_or_try_init(|| async {
        let key_url = std::env::var("JWT_KEY_URL").expect("JWT_KEY_URL not set");
        let client = platform::hyper::Client::new();

        let (status_code, raw) = client
            .raw(
                Method::GET,
                key_url.as_str(),
                ContentType::Json,
                "".to_string(),
                None::<String>,
            )
            .await
            .expect("failed to get keys");

        if status_code != StatusCode::OK {
            return Err(Error::InternalServerError(format!(
                "get keys returned: {status_code} - {raw}"
            )));
        }

        Keys::new(raw.as_bytes())
    })
    .await
    .expect("failed to get keys")
}

/// Gets the required spec claims for validating JWT tokens.
pub fn required_spec_claims() -> &'static Vec<&'static str> {
    static REQUIRED_SPEC_CLAIMS: once_cell::sync::OnceCell<Vec<&'static str>> =
        once_cell::sync::OnceCell::new();
    REQUIRED_SPEC_CLAIMS.get_or_init(|| vec!["exp", "iss", "sub"])
}

/// Encapsulates raw keys for JWT processing.
#[derive(Clone)]
pub struct Keys {
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Result<Self, Error> {
        let jwk_set: JwkSet = serde_json::from_slice(secret).expect("failed to parse jwk set");
        let jwk = jwk_set.keys.last().expect("invalid jwk set");

        Ok(Self {
            decoding: DecodingKey::from_jwk(jwk).expect("invalid decoding key"),
        })
    }
}

impl Debug for Keys {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Keys")
    }
}
