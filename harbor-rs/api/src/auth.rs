use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},

    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt,
};
// use axum::extract::State;
// use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};

use crate::Error;

// use harbor_core::auth::Authenticator;

// TODO: Figure out how to move the JWT aspect of the Authenticator to this crate.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    kid: String,
    aud: String,
    iss: String,
    sub: String,
    client_id: String,
    iat: u64,
    exp: u64,
    nbf: u64,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _authenticator: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(_bearer)) = parts
            .extract::<TypedHeader::<Authorization<Bearer>>>()
            .await
            .map_err(|e| Error::InvalidToken(e.to_string()))?;

        // let _token = authenticator
        //     .verify_token(bearer.token())
        //     .await
        //     .map_err(|e| Error::InvalidToken(e.to_string()))?;

        Ok(Claims{
            kid: "".to_string(),
            aud: "".to_string(),
            iss: "".to_string(),
            sub: "".to_string(),
            client_id: "".to_string(),
            iat: 0,
            exp: 0,
            nbf: 0,
        })
    }
}
