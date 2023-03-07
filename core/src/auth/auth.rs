// use std::str::FromStr;
// use aws_config::SdkConfig;
// use aws_sdk_cognitoidentityprovider::model::IdentityProviderTypeType;
// use chrono::{TimeZone, Utc};
// use iron_aws_cognitoidentityprovider::Provider as CognitoProvider;
// use iron_hyper::{ContentType, get, post};
// use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
// use serde::{Deserialize, Serialize};
// use tracing::{debug, instrument, trace};
// use url::Url;
//
// // use crate::config::{identity_provider_description, user_pool_id};
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
//
// impl Authenticator {
//     pub async fn from_env(config: &SdkConfig) -> Result<Authenticator, Error> {
//         // let user_pool_id = user_pool_id(config).await?;
//         // let provider_description = identity_provider_description(config, Some(user_pool_id.clone())).await?;
//         //
//         // let mut instance = Self {
//         //     user_pool_id,
//         //     identity_provider_name: provider_description.name,
//         //     identity_provider_type: provider_description.provider_type,
//         //     okta: None,
//         // };
//         //
//         // if instance.identity_provider_type.eq(&IdentityProviderTypeType::Oidc) {
//         //     instance.load_okta().await?;
//         // }
//
//         Ok(Self{
//             user_pool_id: "".to_string(),
//             identity_provider_name: "".to_string(),
//             identity_provider_type: IdentityProviderTypeType::Oidc,
//             okta: None,
//         })
//     }
//
//     /// Gets the Okta identity provider client_id, client_secret, and issuer from configuration.
//     async fn load_okta(&mut self) -> Result<(), Error> {
//         // let output = client
//         //     .get_identity_provider_by_identifier()
//         //     .user_pool_id(self.user_pool_id.clone())
//         //     .idp_identifier(self.identity_provider_name.clone())
//         //     .send()
//         //     .await?;
//         //
//         // let provider = output
//         //     .identity_provider()
//         //     .expect("invalid identity provider type");
//         //
//         // let provider_type = provider.provider_type().expect("invalid provider type");
//         // if !provider_type.eq(&self.identity_provider_type) {
//         //     bail!("invalid identity provider type")
//         // }
//         //
//         // let provider_details = provider.provider_details().expect("missing provider details");
//         // let client_id = provider_details.get("client_id").expect("missing client_id").to_string();
//         // let client_secret = provider_details.get("client_secret").expect("missing client_secret").to_string();
//         // let issuer = provider_details.get("oidc_issuer").expect("missing oidc_issuer").to_string();
//         //
//         // self.okta = Some(OktaClient::new(
//         //     client_id,
//         //     client_secret,
//         //     issuer));
//
//         Ok(())
//     }
//
//
//     pub async fn verify_token(&self, jwt: String) -> Result<bool, Error> {
//         match self.identity_provider_type {
//             IdentityProviderTypeType::Oidc => self.verify_okta(jwt).await,
//             _ => self.verify_cognito(jwt).await,
//         }
//     }
//
//     async fn verify_okta(&self, jwt: String) -> Result<bool, Error> {
//         let user_id= "".to_string();
//         let jwk = Jwk{
//             kty: "".to_string(),
//             alg: "".to_string(),
//             kid: "".to_string(),
//             uses: "".to_string(),
//             e: "".to_string(),
//             n: "".to_string(),
//         };
//
//         let algorithm = Algorithm::from_str(jwk.kty.as_str()).expect("unsupported algorithm");
//
//         let okta = self.okta.clone().expect("okta client required to verify token");
//         let audience = okta.audience()?;
//         let mut validation = Validation::new(algorithm);
//         validation.sub = Some(user_id);
//         validation.set_audience(&[audience]);
//         validation.set_issuer(&[okta.issuer.clone()]);
//         validation.set_required_spec_claims(&["exp", "nbf", "aud", "iss", "sub"]);
//
//         let client_secret = okta.client_secret.as_ref();
//         let token_data = decode::<Claims>(
//             jwt.as_str(),
//             &DecodingKey::from_secret(client_secret),
//             &Validation::new(algorithm))?;
//
//         self.validate_claims(&token_data.claims)
//     }
//
//     async fn verify_cognito(&self, _jwt: String) -> Result<bool, Error> {
//         todo!()
//     }
//
//
//     fn validate_claims(&self, claims: &Claims) -> Result<bool, Error> {
//         let now = chrono::Utc::now();
//         let okta = self.okta.clone().expect("okta client required to validate claims");
//         let audience = okta.audience().unwrap_or("".to_string());
//         // Adapted from these rules.
//         // https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-jwt-authorizer.html
//         // kid – The token must have a header claim that matches the key in the jwks_uri that signed the token.
//         // iss – Must match the issuer that is configured for the authorizer.
//         // aud or client_id – Must match one of the audience entries that is configured for the authorizer.
//         // exp – Must be after the current time in UTC.
//         // nbf – Must be before the current time in UTC.
//         // iat – Must be before the current time in UTC.
//         // scope or scp – The token must include at least one of the scopes in the route's authorizationScopes.
//         let result = claims.kid.eq(&claims.kid)
//             && okta.issuer.eq(&claims.iss)
//             && (claims.aud.eq(&audience) || claims.client_id.eq(&okta.client_id))
//             && Utc.timestamp_millis_opt(claims.exp as i64).unwrap() > now
//             && Utc.timestamp_millis_opt(claims.nbf as i64).unwrap() <= now
//             && Utc.timestamp_millis_opt(claims.iat as i64).unwrap() < now;
//
//         Ok(result)
//     }
//
//     // async fn okta_inspect(&self, jwt: String) -> Result<bool, Error> {
//     //     let okta = self.okta.clone().expect("okta client required to inspect token");
//     //     okta.introspect(jwt).await
//     // }
// }
//
// // pub async fn get_user(config: SdkConfig, _username: String, _user_pool_id: String) -> Result<String, Error> {
// //     let client = Client::new(&shared_config);
// //
// //     let output = client
// //         .get_user()
// //         .send().await?;
// //
// //     Ok(output.username().unwrap().to_string())
// // }
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
//     active : bool,
//     token_type: Option<String>,
//     scope: Option<String>,
//     client_id: Option<String>,
//     username : Option<String>,
//     exp : Option<u16>,
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
// impl Jwk {
//
// }
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
//             Some(payload)).await?;
//
//         match result {
//             None => Err(Error::InternalServerError("no provider response".to_string())),
//             Some(r) => Ok(r.active),
//         }
//     }
//
//     pub async fn keys(&self) -> Result<KeysResponse, Error> {
//         let url = format!("{}{}", &self.issuer, KEYS_ROUTE);
//
//         let result = get(
//             url.as_str(),
//             ContentType::Json,
//             "",
//             None::<String>).await?;
//
//         match result {
//             None => Err(Error::InternalServerError("invalid keys response".to_string())),
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
//         Ok(format!("{}://{}", url.scheme(), url.host().expect("invalid host")))
//     }
// }
//
// impl From<jsonwebtoken::errors::Error> for Error {
//     fn from(err: jsonwebtoken::errors::Error) -> Self {
//         Error::InternalServerError(err.to_string())
//     }
// }
//
// impl From<iron_hyper::Error> for Error {
//     fn from(err: iron_hyper::Error) -> Self {
//         Error::InternalServerError(err.to_string())
//     }
// }
//
// impl From<url::ParseError> for Error {
//     fn from(err: url::ParseError) -> Self {
//         Error::InternalServerError(err.to_string())
//     }
// }
