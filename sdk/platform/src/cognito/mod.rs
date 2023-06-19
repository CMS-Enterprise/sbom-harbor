use aws_config::SdkConfig;
use aws_sdk_cognitoidentityprovider as cognitoidp;
use aws_sdk_cognitoidentityprovider::model::{
    ProviderDescription, UserPoolDescriptionType, UserType,
};
use cognitoidp::Client;
use tracing::instrument;

use crate::Error;

/// Coarse-grained abstraction over the AWS Cognito-related SDKs.
#[derive(Debug)]
pub struct Provider {
    config: SdkConfig,
}

impl Provider {
    /// Factory method for creating a new Provider instance.
    pub fn new(config: SdkConfig) -> Self {
        Self { config }
    }

    /// List user pools viewable for the current AWS context.
    #[instrument]
    pub async fn list_user_pools(&self) -> Result<Vec<UserPoolDescriptionType>, Error> {
        let client = Client::new(&self.config);

        let output = client
            .list_user_pools()
            .send()
            .await
            .map_err(|err| Error::Cognito(err.to_string()))?;

        Ok(output.user_pools().unwrap().to_vec())
    }

    /// List users associated with the specified user pool.
    #[instrument]
    pub async fn list_users(&self, user_pool_id: Option<String>) -> Result<Vec<UserType>, Error> {
        let client = Client::new(&self.config);

        let output = client
            .list_users()
            .set_user_pool_id(user_pool_id)
            .send()
            .await
            .map_err(|err| Error::Cognito(err.to_string()))?;

        Ok(output.users().unwrap().to_vec())
    }

    /// List identity providers associated with the specified user pool.
    #[instrument]
    pub async fn list_identity_providers(
        &self,
        user_pool_id: Option<String>,
    ) -> Result<Vec<ProviderDescription>, Error> {
        let client = Client::new(&self.config);

        let output = client
            .list_identity_providers()
            .set_user_pool_id(user_pool_id)
            .send()
            .await
            .map_err(|err| Error::Cognito(err.to_string()))?;

        Ok(output.providers().unwrap().to_vec())
    }
}
