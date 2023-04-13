use std::env;
use platform::auth::get_secret;
use platform::config::from_env;
use platform::mongodb::Context;

use crate::Error;

/// Id for team
pub const TEAM_ID_KEY: &str = "team_id";

/// Key for getting the cloudfront domain
pub const CF_DOMAIN_KEY: &str = "CF_DOMAIN";

/// Key for the Project Id
pub const PROJECT_ID_KEY: &str = "project_id";

/// Key for the codebase Id
pub const CODEBASE_ID_KEY: &str = "codebase_id";

/// Key to get the GitHub token
pub const GH_FT_KEY: &str = "GH_FETCH_TOKEN";

/// Key to get the team Id from the env
pub const V1_TEAM_ID_KEY: &str = "V1_CMS_TEAM_ID";

/// Key to get the team token from the env
pub const V1_TEAM_TOKEN_KEY: &str = "V1_CMS_TEAM_TOKEN";

/// Key to get the v1 user from the env
pub const V1_HARBOR_USERNAME_KEY: &str = "V1_HARBOR_USERNAME";

/// Key to get the v1 password from the env
pub const V1_HARBOR_PASSWORD_KEY: &str = "V1_HARBOR_PASSWORD";

/// Key to get Snyk access key from AWS
pub const SNYK_TOKEN_KEY: &str = "dev-sbom-harbor-snyk-token-use1"; //TODO: This needs to become an env var

/// Returns the Mongo Connection URI as an environment variable.
/// Defaults to local dev environment if not set.
///
pub fn db_connection() -> Result<Context, Error> {
    let mut cx: Context = from_env("DB_CONNECTION")?;

    cx.db_name = "harbor".to_string();
    cx.key_name = "id".to_string();

    Ok(cx)
}

/// Function to get the team id from the environment
///
pub fn get_cms_team_id() -> Result<String, Error> {
    match env::var(V1_TEAM_ID_KEY) {
        Ok(value) => Ok(value),
        Err(_err) => return Err(
            Error::Config(
                String::from("Missing Team Id of V1 Team")
            )
        ),
    }
}

/// Function to get the team token from the environment
///
pub fn get_cms_team_token() -> Result<String, Error> {
    match env::var(V1_TEAM_TOKEN_KEY) {
        Ok(value) => Ok(value),
        Err(_err) => return Err(
            Error::Config(
                String::from("Missing Team token of V1 Team")
            )
        )
    }
}

/// Function to get the cloudfront domain from the environment
///
pub fn get_cf_domain() -> Result<String, Error> {
    match env::var(CF_DOMAIN_KEY) {
        Ok(value) => Ok(value),
        Err(_err) => return Err(
            Error::Config(
                String::from("Missing CloudFront Domain")
            )
        )
    }
}

/// Function to get the v1 username from the environment
///
pub fn get_v1_username() -> Result<String, Error> {
    match env::var(V1_HARBOR_USERNAME_KEY) {
        Ok(value) => Ok(value),
        Err(_err) => return Err(
            Error::Config(
                String::from("Missing Cognito Username")
            )
        )
    }
}

/// Function to get the v1 password from the environment
///
pub fn get_v1_password() -> Result<String, Error> {
    match env::var(V1_HARBOR_PASSWORD_KEY) {
        Ok(value) => Ok(value),
        Err(_err) => return Err(
            Error::Config(
                String::from("Missing Cognito Password")
            )
        )
    }
}

/// Function to get the Snyk Access token from AWS.
/// TODO: this needs to get the token from an env variable instead
pub async fn get_snyk_access_token() -> String {
    println!("Obtaining SNYK access key...");
    let response = get_secret(SNYK_TOKEN_KEY).await;
    return match response {
        Ok(secret) => {
            match secret {
                Some(s) => s,
                None => panic!("No AWS token retrieved for secret: {}", SNYK_TOKEN_KEY), //Stop everything if we dont get an access key
            }
        },
        Err(err) => panic!("Failed to retrieve token for secret: {}, with error: {}", SNYK_TOKEN_KEY, err), //Stop everything if we dont get an access key
    };
}
