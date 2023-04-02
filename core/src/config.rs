use platform::config::{from_env, type_from_env};
use platform::mongodb::Context;
use crate::Error;

/// Returns the Mongo Connection URI from an environment variable. Defaults to local dev environment if not set.
pub fn db_connection() -> Result<Context, Error> {
    let mut cx: Context = type_from_env("DB_CONNECTION")?;

    cx.db_name = "harbor".to_string();
    cx.key_name = "id".to_string();

    Ok(cx)
}

/// Returns the Snyk API token from an environment variable.
pub fn snyk_token() -> Result<String, Error> {
    match from_env("SNYK_API_TOKEN") {
        None => Err(Error::Config("Snyk token not set".to_string())),
        Some(v) => Ok(v),
    }
}
