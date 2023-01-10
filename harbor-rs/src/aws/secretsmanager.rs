use aws_config::environment::EnvironmentVariableRegionProvider;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_secretsmanager::{Client, Error};


pub async fn get_secret(name: &str) -> Result<Option<String>, Error> {
    let region_provider = RegionProviderChain::default_provider()
            .or_else(EnvironmentVariableRegionProvider::new());

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let result = client.get_secret_value().secret_id(name).send().await?;

    if let Some(secret) = result.secret_string() {
        return Ok(Some(secret.to_string()));
    }

    Ok(None)
}
