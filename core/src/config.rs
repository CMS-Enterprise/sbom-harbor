use platform::config;
use platform::config::from_env;
use platform::mongodb::Context;
use crate::Error;

pub fn db_connection() -> Result<Context, Error> {
    let mut cx: Context = from_env("DB_CONNECTION")?;

    cx.db_name = "harbor".to_string();
    cx.key_name = "id".to_string();

    Ok(cx)
}
