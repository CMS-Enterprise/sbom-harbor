use std::env;
use std::error::Error;
use std::ops::Add;
use std::time::{Duration, SystemTime};
use aws_lambda_events::http_body::Body;

use aws_sdk_cognitoidentityprovider::model::IdentityProviderTypeType;
use futures::TryFutureExt;
use hmac::{Hmac, Mac};
use jsonwebtoken::{encode, get_current_timestamp, EncodingKey, Header};
use jwt::{Claims, RegisteredClaims, SignWithKey};
use sha2::Sha256;

use harbor::authorizer::{Authenticator, OktaClient};
use harbor::http;
use harbor::http::ContentType;

mod common;


async fn get_test_jwt() -> Result<String, anyhow::Error> {
    let user_pool_id = env::var("OIDC_USER_POOL_ID").unwrap();
    let idp_identifier = env::var("OIDC_IDENTIFIER").unwrap();

    let authenticator = Authenticator::new(
        user_pool_id,
        idp_identifier,
        IdentityProviderTypeType::Oidc).await?;

    let okta = authenticator.okta.expect("failed to load okta");
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("failed to get now");
    let expires = now.add(Duration::new(100, 0));

    let key: Hmac<Sha256> = Hmac::new_from_slice(okta.client_secret.as_bytes()).unwrap();

    let registered_claims = RegisteredClaims {
        issuer: Some(okta.issuer.clone()),
        subject: Some("sbom@harbor.app".to_string()),
        audience: Some("https://harbor.app".to_string()),
        expiration: Some(expires.as_secs()),
        not_before: Some(now.as_secs()),
        issued_at: Some(now.as_secs()),
        json_web_token_id: Some(uuid::Uuid::new_v4().to_string()),
    };

    let mut claims = Claims{ registered: registered_claims, private: Default::default() };

    let result = encode(&Header::default(), &claims, &EncodingKey::from_secret(okta.client_secret.as_ref()))?;

    Ok(result)
}

#[async_std::test]
async fn can_get_oidc_provider() -> Result<(), String> {
    let user_pool_id = env::var("OIDC_USER_POOL_ID").unwrap();
    let idp_identifier = env::var("OIDC_IDENTIFIER").unwrap();

    let authenticator = Authenticator::new(
        user_pool_id,
        idp_identifier,
        IdentityProviderTypeType::Oidc)
        .map_err(|e| e.to_string())
        .await?;

    let okta = authenticator.okta.expect("failed to load okta");

    assert!(!okta.client_id.is_empty());
    assert!(!okta.client_secret.is_empty());
    assert!(!okta.issuer.is_empty());

    Ok(())
}

#[test]
fn test_audience_parsing() -> Result<(), String> {
    let okta = OktaClient::new(
        "".to_string(),
        "".to_string(),
        "https://valid.com".to_string());

    assert!(!okta.audience().is_err());

    let okta = OktaClient::new(
        "".to_string(),
    "".to_string(),
    "http://invalid-http.com".to_string());

    assert!(okta.audience().is_err());

    let okta = OktaClient::new(
        "".to_string(),
    "".to_string(),
    "ftp://invalid-ftp.com".to_string());

    assert!(okta.audience().is_err());

    Ok(())
}

#[async_std::test]
async fn can_verify_token() -> Result<(), String> {
    let user_pool_id = env::var("OIDC_USER_POOL_ID").unwrap();
    let idp_identifier = env::var("OIDC_IDENTIFIER").unwrap();

    let authenticator = Authenticator::new(
        user_pool_id,
        idp_identifier,
        IdentityProviderTypeType::Oidc)
        .map_err(|e| e.to_string())
        .await?;

    let token = get_test_jwt()
        .map_err(|e| e.to_string())
        .await?;

    let result = authenticator
        .verify_token(token)
        .map_err(|e| e.to_string())
        .await?;

    assert!(!result);

    let token = String::from("00n3w4XaJKqsVRUEf8Q3hAt8Lyi0aBIt54Q1JaW2RW");
    let token = base64::encode(token);

    let result = authenticator.verify_token(token)
        .map_err(|e| e.to_string())
        .await?;

    assert!(result);

    Ok(())
}

#[async_std::test]
async fn can_introspect_token() -> Result<(), String> {
    let user_pool_id = env::var("OIDC_USER_POOL_ID").unwrap();
    let idp_identifier = env::var("OIDC_IDENTIFIER").unwrap();

    let authenticator = Authenticator::new(
        user_pool_id,
        idp_identifier,
        IdentityProviderTypeType::Oidc)
        .map_err(|e| e.to_string())
        .await?;

    let okta = authenticator.okta.expect("failed to load okta");

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let token = get_test_jwt().map_err(|e| e.to_string()).await?;

    let result = okta.introspect(token)
        .map_err(|e| e.to_string())
        .await?;

    assert!(!result);

    let token = String::from("00n3w4XaJKqsVRUEf8Q3hAt8Lyi0aBIt54Q1JaW2RW");
    let token = base64::encode(token);

    let result = okta.introspect(token)
        .map_err(|e| e.to_string())
        .await?;

    assert!(result);

    Ok(())
}

#[async_std::test]
async fn can_get_keys() -> Result<(), String> {
    let user_pool_id = env::var("OIDC_USER_POOL_ID").unwrap();
    let idp_identifier = env::var("OIDC_IDENTIFIER").unwrap();

        let authenticator = Authenticator::new(
                user_pool_id,
                idp_identifier,
                IdentityProviderTypeType::Oidc)
            .map_err(|e| e.to_string())
            .await?;

    let okta = authenticator.okta.expect("failed to load okta");

    let result = okta.keys()
        .map_err(|e| e.to_string())
        .await?;

    assert!(!result.keys.is_empty());

    Ok(())
}
