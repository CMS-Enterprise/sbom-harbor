use anyhow::bail;
use aws_config::environment::EnvironmentVariableRegionProvider;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cognitoidentityprovider::Client;
use aws_sdk_cognitoidentityprovider::model::IdentityProviderTypeType;
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use lambda_http::{Body, Error, Request, RequestExt, Response};
use lambda_http::http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument, trace};
use url::Url;

use crate::http;
use crate::http::ContentType;

/// Implements the ServiceFn<T> interface expected by the Lambda runtime.
/// Provides protocol level validation, and then delegates to request handler.
#[instrument]
pub async fn authorizer_handler(req: Request) -> Result<Response<Body>, Error> {
    let params = req.path_parameters();
    let id = params.first("teamId");

    if let Err(err) = validate(id.clone()) {
        debug!("{}", err);
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text(err.to_string()))
            .map_err(Box::new)?);
    };

    let response = handle_request(id.unwrap()).await?;

    trace!("complete with response: {:?}", response);

    let json = serde_json::to_vec(&response)?;
    let body = Body::from(json);

    let response: Response<Body> = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)
        .map_err(Box::new)?;

    Ok(response)
}

pub async fn handle_request(id: &str) -> Result<(), Error> {
    debug!("authorizing for team {}", id);
    // TODO: Implement
    Ok(())
}

pub(crate) fn validate(id: Option<&str>) -> Result<(), Error> {
    if id.is_none() || id.unwrap().is_empty() {
        return Err(Error::from("teamId is required".to_string()));
    }

    Ok(())
}


/// Performs verification logic for authentication.
pub struct Authenticator {
    pub user_pool_id: String,
    pub identity_provider_name: String,
    pub identity_provider_type: IdentityProviderTypeType,
    pub okta: Option<OktaClient>,
}

impl Authenticator {
    pub async fn new(
        user_pool_id: String,
        identity_provider_name: String,
        identity_provider_type: IdentityProviderTypeType) -> Result<Authenticator, anyhow::Error> {

        let mut instance = Authenticator {
            user_pool_id,
            identity_provider_name,
            identity_provider_type,
            okta: None,
        };

        if instance.identity_provider_type.eq(&IdentityProviderTypeType::Oidc) {
            instance.load_okta().await?;
        }

        Ok(instance)
    }

    /// Gets the Okta identity provider client_id, client_secret, and issuer from configuration.
    async fn load_okta(&mut self) -> Result<(), anyhow::Error> {
        let region_provider = RegionProviderChain::default_provider()
                .or_else(EnvironmentVariableRegionProvider::new());

        let shared_config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&shared_config);

        let output = client
            .get_identity_provider_by_identifier()
            .user_pool_id(self.user_pool_id.clone())
            .idp_identifier(self.identity_provider_name.clone())
            .send()
            .await?;

        let provider = output
            .identity_provider()
            .expect("invalid identity provider type");

        let provider_type = provider.provider_type().expect("invalid provider type");
        if !provider_type.eq(&self.identity_provider_type) {
            bail!("invalid identity provider type")
        }

        let provider_details = provider.provider_details().expect("missing provider details");
        let client_id = provider_details.get("client_id").expect("missing client_id").to_string();
        let client_secret = provider_details.get("client_secret").expect("missing client_secret").to_string();
        let issuer = provider_details.get("oidc_issuer").expect("missing oidc_issuer").to_string();

        self.okta = Some(OktaClient::new(
            client_id,
            client_secret,
            issuer));

        Ok(())
    }


    pub async fn verify_token(&self, jwt: String) -> Result<bool, anyhow::Error> {
        match self.identity_provider_type {
            IdentityProviderTypeType::Oidc => self.verify_okta(jwt).await,
            _ => self.verify_cognito(jwt).await,
        }
    }

    async fn verify_okta(&self, jwt: String) -> Result<bool, anyhow::Error> {
        let user_id= "".to_string();
        let jwk = Jwk{
            kty: "".to_string(),
            alg: "".to_string(),
            kid: "".to_string(),
            uses: "".to_string(),
            e: "".to_string(),
            n: "".to_string(),
        };

        let okta = self.okta.expect("okta client not initialized");
        let algorithm = Algorithm::from(jwk.kty.as_str());

        let validation = Validation::new(algorithm);
        validation.sub = Some(user_id);
        validation.set_audience(&[okta.audience()]);

        let client_secret = okta.client_secret.as_ref();
        let token = decode(
            jwt.as_str(),
            &DecodingKey::from_secret(client_secret),
            &Validation::new(algorithm))?;

        Ok(true)
    }

    async fn verify_cognito(&self, _jwt: String) -> Result<bool, anyhow::Error> {
        todo!()
    }

    async fn okta_inspect(&self, jwt: String) -> Result<bool, anyhow::Error> {
        self.okta.clone().expect("okta client not initialized").introspect(jwt).await
    }
}

pub async fn get_user(_username: String, _user_pool_id: String) -> Result<String, Error> {
    let region_provider = RegionProviderChain::default_provider()
            .or_else(EnvironmentVariableRegionProvider::new());

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let output = client
        .get_user()
        .send().await?;

    Ok(output.username().unwrap().to_string())
}

const INTROSPECTION_ROUTE: &str = "/v1/introspect";

#[derive(Debug, Deserialize, Serialize)]
struct IntrospectRequest {
    token: String,
    token_type: String,
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct IntrospectResponse {
    active : bool,
    token_type: Option<String>,
    scope: Option<String>,
    client_id: Option<String>,
    username : Option<String>,
    exp : Option<u16>,
    iat: Option<u16>,
    sub: Option<String>,
    aud: Option<String>,
    iss: Option<String>,
    jti: Option<String>,
    uid: Option<String>,
}

const KEYS_ROUTE: &str = "/v1/keys";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Jwk {
    // "kty" (key type) identifies the algorithm used with the key (e.g. RSA, EC).
    kty: String,
    // "alg" (algorithm) identifies the algorithm intended to use with the key.
    alg: String,
    // The "kid" (key ID) is the unique identifier of the key.
    kid: String,
    // "use" (public key use) indicates whether a public key is used for encrypting
    // data or verifying a signature.
    #[serde(rename = "use")]
    uses: String,
    // RSA public exponent. Used on signed / encoded data to decode.
    e: String,
    // RSA is the product of two prime numbers used to generate the key pair
    n: String,
}

impl Jwk {

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeysResponse {
    pub keys: Vec<Jwk>,
}

/// The OktaClient is used to authenticate JWTs with the Okta OIDC service.
#[derive(Clone, Debug)]
pub struct OktaClient {
    pub client_id: String,
    pub client_secret: String,
    pub issuer: String,
}

impl OktaClient {
    pub fn new(client_id: String, client_secret: String, issuer: String) -> Self {
        Self {
            issuer,
            client_id,
            client_secret,
        }
    }

    pub async fn introspect(&self, jwt: String) -> Result<bool, anyhow::Error> {
        let url = format!("{}{}", &self.issuer, INTROSPECTION_ROUTE);
        let payload = IntrospectRequest {
            token: jwt,
            token_type: "access_token".to_string(),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
        };

        let result = http::post::<IntrospectRequest, IntrospectResponse>(
            url.as_str(),
            "",
            Some(payload),
        Some(ContentType::FormUrlEncoded)).await?;

        match result {
            None => bail!("invalid introspect response"),
            Some(r) => Ok(r.active),
        }
    }

    pub async fn keys(&self) -> Result<KeysResponse, anyhow::Error> {
        let url = format!("{}{}", &self.issuer, KEYS_ROUTE);

        let result = http::get(
            url.as_str(),
            "",
            None::<String>,
        Some(ContentType::Json)).await?;

        match result {
            None => bail!("invalid keys response"),
            Some(r) => Ok(r),
        }
    }

    pub fn audience(&self) -> Result<String, anyhow::Error> {
        let url = Url::parse(&self.issuer);

        if !url.scheme().eq("https") {
            bail!("invalid scheme");
        }

        Ok(format!("{}://{}", url.scheme(), url.host()))
    }
}
