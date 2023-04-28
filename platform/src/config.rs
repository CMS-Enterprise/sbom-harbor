use aws_config as sdk;
use sdk::environment::region::EnvironmentVariableRegionProvider;
use sdk::meta::region::RegionProviderChain;
use sdk::SdkConfig;
use serde::Deserialize;
use std::env::VarError;

use crate::Error;

/// Retrieve an arbitrary value from the environment and cast it to a type.
pub fn type_from_env<T>(key: &str) -> Result<T, Error>
where
    for<'a> T: Default + Deserialize<'a>,
{
    let raw = std::env::vars().find(|(k, _)| k == key);

    match raw {
        None => Ok(T::default()),
        Some(v) => {
            let result =
                serde_json::from_str(v.1.as_str()).map_err(|e| Error::Config(e.to_string()))?;

            Ok(result)
        }
    }
}

pub fn from_env(key: &str) -> Option<String> {
    match std::env::vars().find(|(k, _)| k == key) {
        None => None,
        Some((_, v)) => Some(v),
    }
}

/// Retrieves the AWS SDK config using the default [RegionProviderChain].
pub async fn sdk_config_from_env() -> Result<SdkConfig, Error> {
    let region_provider =
        RegionProviderChain::default_provider().or_else(EnvironmentVariableRegionProvider::new());

    let config = sdk::from_env().region(region_provider).load().await;

    Ok(config)
}

// pub async fn user_pool_id(config: &SdkConfig) -> Result<String, Error> {
//     let provider = Provider::new(config.clone());
//     let user_pools = provider.list_user_pools().await?;
//
//     Ok("not implemented".to_string())
// }
//
// pub async fn identity_provider_description(config: &SdkConfig, user_pool_id: Option<String>) -> Result<ProviderDescription, Error> {
//     let provider = Provider::new(config.clone());
//
//     let providers = provider.list_identity_providers(user_pool_id)?;
//     // TODO: Implement algorithm to decide which provider to use.
//     Ok(providers.next())
// }

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        Error::Config(format!("{:?}", err))
    }
}
