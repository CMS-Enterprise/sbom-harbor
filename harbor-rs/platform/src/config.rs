use std::collections::HashMap;
use std::env::VarError;
use std::iter::Iterator;
use aws_config as sdk;
use lazy_static::lazy_static;
use sdk::environment::region::EnvironmentVariableRegionProvider;
use sdk::meta::region::RegionProviderChain;
use sdk::SdkConfig;

use crate::Error;

lazy_static! {
    static ref REGION_SHORT_CODES: HashMap<&'static str, &'static str> = [
        ("us-east-1","use1"),
        ("us-east-2]", "use2"),
        ("us-west-1]", "usw1"),
        ("us-west-2]", "usw2")
    ].iter().cloned().collect();
}

pub fn environize(resource: &str) -> Result<String, Error> {
    let environment = std::env::var("ENVIRONMENT".to_string())?;
    let region = std::env::var("AWS_REGION".to_string())?;
    let short_code = REGION_SHORT_CODES.get(region.as_str());

    match short_code {
        None => Err(Error::Config(format!("unsupported region: {}", region))),
        Some(code) => Ok(format!("{}-{}-{}", environment, resource, code)),
    }
}

pub async fn sdk_config_from_env() -> Result<SdkConfig, Error> {
    let region_provider = RegionProviderChain::default_provider()
        .or_else(EnvironmentVariableRegionProvider::new());

    let config = sdk::from_env()
        .region(region_provider)
        .load()
        .await;

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
