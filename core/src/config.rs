use platform::config;
use platform::config::from_env;
use platform::mongodb::Context;
use crate::Error;

/// Returns the Mongo Connection URI as an environment variable. Defaults to local dev environment if not set.
pub fn db_connection() -> Result<Context, Error> {
    let mut cx: Context = from_env("DB_CONNECTION")?;

    cx.db_name = "harbor".to_string();
    cx.key_name = "id".to_string();

    Ok(cx)
}
