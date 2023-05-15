use crate::config::{
    get_api_url,
    get_cms_team_id,
    get_cms_team_token,
    get_v1_password,
    get_v1_username
};
use crate::Error;
use crate::services::providers::github::error::Error;

/// Args for generating one ore more SBOMs from a GitHub Organization.
///
#[derive(Clone, Debug)]
pub struct GitHubProviderEnvironmentConfig {

    /// This is the GUID that is in DynamoDB that
    /// belongs to the team we are using.
    pub(crate) cms_team_id: String,

    /// This is the token from that team
    pub(crate) cms_team_token: String,

    /// This is the Cloudfront Domain of the API endpoints
    pub(crate) api_url: String,

    /// The username we use to get the JWT and make API calls
    pub(crate) cognito_username: String,

    /// The password we use to get the JWT and make API calls
    pub(crate) cognito_password: String,
}

impl GitHubProviderEnvironmentConfig {

    /// Snag a bunch of environment variables and put them into a struct
    ///
    pub(crate) fn extract() -> Result<GitHubProviderEnvironmentConfig, Error> {

        let cms_team_id = match get_cms_team_id() {
            Ok(value) => value,
            Err(err) => return Err(
                Error::Configuration(err)
            )
        };

        let cms_team_token = match get_cms_team_token() {
            Ok(value) => value,
            Err(err) => return Err(
                Error::Configuration(err)
            )
        };

        let api_url = match get_api_url() {
            Ok(value) => value,
            Err(err) => return Err(
                Error::Configuration(err)
            )
        };

        let cognito_username = match get_v1_username() {
            Ok(value) => value,
            Err(err) => return Err(
                Error::Configuration(err)
            )
        };

        let cognito_password = match get_v1_password() {
            Ok(value) => value,
            Err(err) => return Err(
                Error::Configuration(err)
            )
        };

        Ok(
            GitHubProviderEnvironmentConfig {
                cms_team_id,
                cms_team_token,
                api_url,
                cognito_username,
                cognito_password,
            }
        )
    }
}

#[tokio::test]
async fn test_get_env_vars() {
    match GitHubProviderEnvironmentConfig::extract() {
        Ok(config) => println!("{:#?}", config),
        Err(err) => panic!("Extracting env failed: {}", err)
    }
}