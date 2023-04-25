use aws_config::environment::EnvironmentVariableRegionProvider;
use aws_config::meta::region::RegionProviderChain;
use aws_config::SdkConfig;
pub use platform::mongodb::*;
use platform::Error;

pub mod mongodb;

#[allow(dead_code)]
pub async fn config_from_env() -> Result<SdkConfig, Error> {
    let region_provider =
        RegionProviderChain::default_provider().or_else(EnvironmentVariableRegionProvider::new());

    let config = aws_config::from_env().region(region_provider).load().await;

    Ok(config)
}
