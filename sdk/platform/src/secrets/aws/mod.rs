use aws_sdk_secretsmanager::Client;
use aws_types::SdkConfig;

use crate::config::sdk_config_from_env;
use crate::Error;

/// Provides a coarse-grained abstraction over S3 that conforms to the conventions of this crate.
#[derive(Debug)]
pub struct Store {
    client: Client,
}

impl Store {
    /// Factory method for creating new instance of type.
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    /// Factory method for creating new instance of type. SDK Configuration is retrieved from the
    /// environment.
    pub async fn new_from_env() -> Result<Self, Error> {
        let config = sdk_config_from_env()
            .await
            .map_err(|e| Error::Config(e.to_string()))?;
        let client = Client::new(&config);

        Ok(Self { client })
    }

    /// Gets a secret value by id.
    pub async fn get(&self, secret_id: &str) -> Result<Option<String>, Error> {
        let result = self
            .client
            .get_secret_value()
            .secret_id(secret_id)
            .send()
            .await
            .map_err(|e| Error::Secrets(e.to_string()))?;

        if let Some(secret) = result.secret_string() {
            return Ok(Some(secret.to_string()));
        }

        Ok(None)
    }
}
