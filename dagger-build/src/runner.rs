use dagger_sdk;
use dagger_sdk::DaggerConn;
use platform::secrets::aws::Store as SecretsStore;

use crate::Error;

pub struct Runner {
    client: DaggerConn,
    secrets: SecretsStore,
}

impl Runner {
    /// Factory method to create new instance of type.
    pub async fn new() -> Result<Self, Error> {
        let client = dagger_sdk::connect().await?;
        let secrets = SecretsStore::new_from_env().await?;

        Ok(Self { client, secrets })
    }

    pub async fn run(&self) -> Result<(), Error> {
        let version = self
            .client
            .container()
            .from("1.71.0-slim-bullseye")
            .with_exec(vec!["cargo", "version"])
            .stdout()
            .await?;

        println!("Hello from cargo and {}", version.trim());
    }
}
