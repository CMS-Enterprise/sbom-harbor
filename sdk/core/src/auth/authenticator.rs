// use platform::hyper::{get, post, ContentType};
// use serde::{Deserialize, Serialize};
// use url::Url;
//
// use crate::Error;
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     kid: String,
//     aud: String,
//     iss: String,
//     sub: String,
//     client_id: String,
//     iat: u64,
//     exp: u64,
//     nbf: u64,
// }
//
// /// Performs verification logic for authentication.
// pub struct Authenticator {
//     pub user_pool_id: String,
//     pub identity_provider_name: String,
//     pub identity_provider_type: IdentityProviderTypeType,
//     pub okta: Option<OktaClient>,
// }
//
// impl Authenticator {
//     pub async fn from_env(config: &SdkConfig) -> Result<Authenticator, Error> {
//         Ok(Self {
//             user_pool_id: "".to_string(),
//             identity_provider_name: "".to_string(),
//             identity_provider_type: IdentityProviderTypeType::Oidc,
//             okta: None,
//         })
//     }
//
//     pub async fn verify_token(&self, jwt: String) -> Result<bool, Error> {
//         match self.identity_provider_type {
//             IdentityProviderTypeType::Oidc => self.verify_okta(jwt).await,
//             _ => self.verify_cognito(jwt).await,
//         }
//     }
//
//     async fn verify_okta(&self, jwt: String) -> Result<bool, Error> {
//         let okta = self
//             .okta
//             .clone()
//             .expect("okta client required to inspect token");
//         okta.introspect(jwt).await
//     }
//
//     async fn verify_cognito(&self, jwt: String) -> Result<bool, Error> {
//         Ok(false)
//     }
// }
//
// const INTROSPECTION_ROUTE: &str = "/v1/introspect";
//
// #[derive(Debug, Deserialize, Serialize)]
// struct IntrospectRequest {
//     token: String,
//     token_type: String,
//     client_id: String,
//     client_secret: String,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// struct IntrospectResponse {
//     active: bool,
//     token_type: Option<String>,
//     scope: Option<String>,
//     client_id: Option<String>,
//     username: Option<String>,
//     exp: Option<u16>,
//     iat: Option<u16>,
//     sub: Option<String>,
//     aud: Option<String>,
//     iss: Option<String>,
//     jti: Option<String>,
//     uid: Option<String>,
// }
//
// const KEYS_ROUTE: &str = "/v1/keys";
//
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Jwk {
//     // "kty" (key type) identifies the algorithm used with the key (e.g. RSA, EC).
//     kty: String,
//     // "alg" (algorithm) identifies the algorithm intended to use with the key.
//     alg: String,
//     // The "kid" (key ID) is the unique identifier of the key.
//     kid: String,
//     // "use" (public key use) indicates whether a public key is used for encrypting
//     // data or verifying a signature.
//     #[serde(rename = "use")]
//     uses: String,
//     // RSA public exponent. Used on signed / encoded data to decode.
//     e: String,
//     // RSA is the product of two prime numbers used to generate the key pair
//     n: String,
// }
//
// impl Jwk {}
//
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct KeysResponse {
//     pub keys: Vec<Jwk>,
// }
//
// /// The OktaClient is used to authenticate JWTs with the Okta OIDC service.
// #[derive(Clone, Debug)]
// pub struct OktaClient {
//     pub client_id: String,
//     pub client_secret: String,
//     pub issuer: String,
// }
//
// impl OktaClient {
//     pub fn new(client_id: String, client_secret: String, issuer: String) -> Self {
//         Self {
//             issuer,
//             client_id,
//             client_secret,
//         }
//     }
//
//     pub async fn introspect(&self, jwt: String) -> Result<bool, Error> {
//         let url = format!("{}{}", &self.issuer, INTROSPECTION_ROUTE);
//         let payload = IntrospectRequest {
//             token: jwt,
//             token_type: "access_token".to_string(),
//             client_id: self.client_id.clone(),
//             client_secret: self.client_secret.clone(),
//         };
//
//         let result = post::<IntrospectRequest, IntrospectResponse>(
//             url.as_str(),
//             ContentType::FormUrlEncoded,
//             "",
//             Some(payload),
//         )
//         .await?;
//
//         match result {
//             None => Err(Error::InternalServerError(
//                 "no provider response".to_string(),
//             )),
//             Some(r) => Ok(r.active),
//         }
//     }
//
//     pub async fn keys(&self) -> Result<KeysResponse, Error> {
//         let url = format!("{}{}", &self.issuer, KEYS_ROUTE);
//
//         let result = get(url.as_str(), ContentType::Json, "", None::<String>).await?;
//
//         match result {
//             None => Err(Error::InternalServerError(
//                 "invalid keys response".to_string(),
//             )),
//             Some(r) => Ok(r),
//         }
//     }
//
//     pub fn audience(&self) -> Result<String, Error> {
//         let url = Url::parse(&self.issuer)?;
//
//         if !url.scheme().eq("https") {
//             return Err(Error::InternalServerError("invalid scheme".to_string()));
//         }
//
//         Ok(format!(
//             "{}://{}",
//             url.scheme(),
//             url.host().expect("invalid host")
//         ))
//     }
// }
