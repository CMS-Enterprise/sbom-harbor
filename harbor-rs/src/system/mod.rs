use std::fmt::{Display, Formatter};

use okta;
use serde::{Deserialize, Serialize};
pub mod authorizer;

/// Discriminator field to determine which provider to invoke
/// during authentication workflows.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AuthenticationProvider {
    Harbor,
    OIDC(String),
}

impl Display for AuthenticationProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthenticationProvider::Harbor => write!(f, "harbor"),
            AuthenticationProvider::OIDC(provider_name) => write!(f, "{}", provider_name),
        }
    }
}

/// Contains all OIDC specific configuration values for the OIDC integration.
pub struct OIDCContext {
    client_id: String,
    client_secret: String,
    issuer: String,
}

/// Performs the OIDC authorization logic for OIDC authentication.
pub struct OIDCProvider {
    discriminator: AuthenticationProvider,
}

impl OIDCProvider {
    pub fn verify_token(ctx: OIDCContext, jwt: String) -> Result<(), Error> {
        let okta_client = okta::Client(ctx.client_id, ctx.client_secret, ctx.issuer);

        let okta_result = okta_client.verify_token(jwt)?;



        Ok(())
    }
}

/// Configuration related to then entire system.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    authentication_provider: AuthenticationProvider,
}

/// Singleton entity that contains configuration information that applies
/// to the entire system.
pub struct System {
    partition_key: String,
    sort_key: String,
    config: Config,
}

const SYSTEM_ID: &str = "harbor";

impl System {
    pub fn new(config: Config) -> Self {
        Self{
            partition_key: SYSTEM_ID.to_string(),
            sort_key: SYSTEM_ID.to_string(),
            config
        }
    }
}

pub struct User {
    partition_key: String,
    sort_key: String,
    parent_id: String,
    email: String,
    given_name: Option<String>,
    family_name: Option<String>,
    name: Option<String>,
    username: Option<String>,
    roles: Option<Vec<String>>,
}

impl User {
    pub fn new(email: String,
               given_name: Option<String>,
               family_name: Option<String>,
               name: Option<String>,
               username: Option<String>) -> Self {
        Self {
            partition_key: SYSTEM_ID.to_string(),
            sort_key: format!("user#{}", email),
            parent_id: SYSTEM_ID.to_string(),
            email,
            given_name,
            family_name,
            name,
            username,
            roles: None,
        }
    }

    // pub fn add_role(&mut self, role: String) {
    //     let has_role = self.has_role(role.clone());
    //     if has_role {
    //         return;
    //     }
    //
    //     if *&self.roles.is_none() {
    //         self.roles = Some(vec![]);
    //     }
    //
    //     self.roles.unwrap().push(role);
    // }
    //
    // pub fn has_role(&mut self, role: String) -> bool {
    //     if self.roles.is_none() {
    //         return false;
    //     }
    //
    //     self.roles.unwrap().contains(&role)
    // }
}
