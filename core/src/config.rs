use platform::config::from_env;
use platform::mongodb::Context;
use crate::Error;

pub const COLLECTION: &str = "pilot";
pub const TEAM_ID_KEY: &str = "team_id";
pub const CF_DOMAIN_KEY: &str = "CF_DOMAIN";
pub const PROJECT_ID_KEY: &str = "project_id";
pub const CODEBASE_ID_KEY: &str = "codebase_id";
pub const GH_FT_KEY: &str = "GH_FETCH_TOKEN";
pub const V1_TEAM_ID_KEY: &str = "V1_CMS_TEAM_ID";
pub const V1_TEAM_TOKEN_KEY: &str = "V1_CMS_TEAM_TOKEN";
pub const V1_HARBOR_USERNAME_KEY: &str = "V1_HARBOR_USERNAME";
pub const V1_HARBOR_PASSWORD_KEY: &str = "V1_HARBOR_PASSWORD";

/// Returns the Mongo Connection URI as an environment variable.
/// Defaults to local dev environment if not set.
///
pub fn db_connection() -> Result<Context, Error> {
    let mut cx: Context = from_env("DB_CONNECTION")?;

    cx.db_name = "harbor".to_string();
    cx.key_name = "id".to_string();

    Ok(cx)
}

pub fn get_cms_team_token() -> Result<String, Error> {
    match from_env::<String>(V1_TEAM_TOKEN_KEY) {
        Ok(value) => Ok(value),
        Err(_err) => return Err(
            Error::Config(
                String::from("Missing Team token of V1 Team")
            )
        )
    }
}

pub fn get_cf_domain() -> Result<String, Error> {
    match from_env::<String>(CF_DOMAIN_KEY) {
        Ok(value) => Ok(value),
        Err(_err) => return Err(
            Error::Config(
                String::from("Missing CloudFront Domain")
            )
        )
    }
}

pub fn get_v1_username() -> Result<String, Error> {
    match from_env::<String>(V1_HARBOR_USERNAME_KEY) {
        Ok(value) => Ok(value),
        Err(_err) => return Err(
            Error::Config(
                String::from("Missing Cognito Username")
            )
        )
    }
}

pub fn get_v1_password() -> Result<String, Error> {
    match from_env::<String>(V1_HARBOR_PASSWORD_KEY) {
        Ok(value) => Ok(value),
        Err(_err) => return Err(
            Error::Config(
                String::from("Missing Cognito Password")
            )
        )
    }
}
