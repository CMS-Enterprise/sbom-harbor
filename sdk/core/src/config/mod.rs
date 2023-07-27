use std::collections::HashMap;

use platform::config::from_env;
use platform::encoding::url_encode;
use platform::persistence::mongodb::Context;
use serde::{Deserialize, Serialize};

use crate::Error;

pub struct Settings {
    pub process: Option<Vec<String>>,
    pub cx: Option<Context>,
    pub log_level: Option<String>,
    pub jwt_config: Option<HashMap<String, String>>,
    pub file_store: Option<String>,
    pub snyk_token: Option<String>,
    pub github_token: Option<String>,
    pub ion_channel_token: Option<String>,
}

impl Settings {
    pub fn new(file_path: Option<String>) -> Result<Self, Error> {
        match from_file() {}
        Ok(Self {
            cx: harbor_context()?,
            log_level: log_level()?,
            jwt_config: jwt_config()?,
            file_store: file_store()?,
            snyk_token: snyk_token()?,
            github_token: github_token()?,
            ion_channel_token: ion_channel_token()?,
        })
    }
}

fn from_file(file_path: Option<String>) -> Result<Option<Settings>, Error> {
    let file_path = match file_path {
        None => "config.toml",
        Some(f) => f.as_str(),
    };

    let
}

/// Returns the Snyk API token from an environment variable.
pub fn snyk_token() -> Result<String, Error> {
    match from_env("SNYK_TOKEN") {
        None => Err(Error::Config("Snyk token not set".to_string())),
        Some(v) => Ok(v),
    }
}

/// Returns the GitHub token from an environment variable.
pub fn github_token() -> Result<String, Error> {
    match from_env("GITHUB_PAT") {
        None => Err(Error::Config("GITHUB_PAT token not set".to_string())),
        Some(v) => Ok(v),
    }
}

/// Returns the Ion Channel token from an environment variable.
pub fn ion_channel_token() -> Result<String, Error> {
    match from_env("ION_CHANNEL_TOKEN") {
        None => Err(Error::Config("ION_CHANNEL_TOKEN token not set".to_string())),
        Some(v) => Ok(v),
    }
}

/// Returns the file store path for the configured provider.
pub fn file_store() -> Result<String, Error> {
    match from_env("HARBOR_FILE_STORE") {
        None => Err(Error::Config("Harbor bucket not set".to_string())),
        Some(v) => Ok(v),
    }
}

/// Returns a Mongo Context for used with the local devenv. Used by tests or for local development.
pub fn dev_context(db_name: Option<&str>) -> Result<Context, Error> {
    let db_name = match db_name {
        None => "harbor",
        Some(db_name) => db_name,
    };

    Ok(Context {
        host: "mongo".to_string(),
        username: "root".to_string(),
        password: "harbor".to_string(),
        port: 27017,
        db_name: db_name.to_string(),
        key_name: "id".to_string(),
        connection_uri: None,
    })
}

/// Returns a Context specific to the Harbor teams deployment environment.
pub fn harbor_context() -> Result<Context, Error> {
    let raw_config = match from_env("DOCDB_CONFIG") {
        None => {
            return Err(Error::Config("DocumentDB config not set".to_string()));
        }
        Some(raw_config) => raw_config,
    };

    let cfg: DocDbConfig = serde_json::from_str(raw_config.as_str())
        .map_err(|e| Error::Serde(format!("invalid DocumentDB config::{}", e)))?;

    Ok(cfg.to_context())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct DocDbConfig {
    username: String,
    password: String,
    engine: String,
    port: u32,
    host: String,
    ssl: bool,
    #[serde(rename = "dbInstanceIdentifier")]
    db_instance_identifier: String,
}

impl DocDbConfig {
    fn to_context(&self) -> Context {
        let mut connection_uri = format!(
            "mongodb://{}:{}@{}:{}",
            url_encode(self.username.as_str()),
            url_encode(self.password.as_str()),
            self.host,
            self.port
        );

        connection_uri = match self.ssl {
            true => format!(
                "{}/?ssl=true&tlsCAFile=rds-combined-ca-bundle.pem&retryWrites=false",
                connection_uri
            ),
            false => format!("{}/?ssl=false&retryWrites=false", connection_uri),
        };

        Context {
            host: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            port: 0,
            db_name: "harbor".to_string(),
            key_name: "id".to_string(),
            connection_uri: Some(connection_uri),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_local_config() -> Result<(), Error> {}

    #[test]
    fn can_load_dev_config() -> Result<(), Error> {}

    #[test]
    fn can_load_prod_config() -> Result<(), Error> {}

    #[test]
    fn can_load_user_config() -> Result<(), Error> {}
}
