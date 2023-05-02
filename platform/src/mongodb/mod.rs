use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub use service::*;
pub use store::*;

use crate::auth::*;
use crate::Error;

/// Provides a row-level authorization mechanism for controlling access to entries in a [Collection].
pub mod auth;
/// Provides MongoDB db migrations support.
pub mod migrations;
/// Provides a generics-based [Service] trait for handling CRUD based operations against a [Store].
pub mod service;
/// Provides a generics-based [Store] trait for handling CRUD based operations against a [Collection].
pub mod store;

/// Specifies the backend data store for the [Context].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ContextKind {
    /// Specifies the data store is provided by Cosmos DB for MongoDB.
    CosmosDB,
    /// Specifies the data store is provided by DocumentDB.
    DocumentDB,
    /// Specifies the data store is a native MongoDB instance.
    Mongo,
    // /// Specifies that the data store uses an injected custom connection uri provider.
    // Custom,
}

/// Trait to implement when injecting a custom uri provider.
pub trait ConnectionUriProvider: Debug + Send + Sync + Sized {
    /// Implement this to provide a custom uri.
    fn connection_uri(&self) -> Result<String, Error>;
}

// TODO: Make Context a trait and split these up in to purpose build contexts instead of a single type.
/// Provides connection information and schema conventions for a MongoDB/DocumentDB backed [Store].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    /// Kind of backend provider for the [Context].
    pub kind: ContextKind,
    /// Optionally injected custom [ConnectionUriProvider].
    // pub connection_uri_provider: Option<Box<dyn ConnectionUriProvider>>,
    /// Instance host name which must be a resolvable DNS name.
    pub host: String,
    /// MongoDB username.
    pub username: String,
    /// MongoDB password.
    pub password: String,
    /// Port that MongoDB is listening on.
    pub port: u32,
    /// The database name hosted at the instance.
    pub db_name: String,
    /// The conventional name of document key fields.
    pub key_name: String,
    /// The name of the cluster when connecting to a DocumentDB instance.
    pub region: Option<String>,
    /// The name of the cluster when connecting to a DocumentDB instance.
    pub cluster_name: Option<String>,
    /// The name of the account when connecting to a Cosmos DB instance.
    pub account_name: Option<String>,
}

impl Context {
    // pub fn new_custom(custom: Box<dyn ConnectionUriProvider>) -> Self {
    //     Self {
    //         kind: ContextKind::Custom,
    //         connection_uri_provider: Some(custom),
    //         host: "".to_string(),
    //         username: "".to_string(),
    //         password: "".to_string(),
    //         port: 0,
    //         db_name: "".to_string(),
    //         key_name: "".to_string(),
    //         region: None,
    //         cluster_name: None,
    //         account_name: None,
    //     }
    // }

    /// Returns a formatted MongoDB compliant URI for the MongoDB instance.
    pub fn connection_uri(&self) -> Result<String, Error> {
        match self.kind {
            ContextKind::CosmosDB => match &self.account_name {
                None => Err(Error::Config("account name required".to_string())),
                Some(account_name) => Ok(format!(
                    "mongodb://{}:{}@{}.documents.azure.com:10255/?ssl=true",
                    self.username, self.password, account_name
                )),
            },
            ContextKind::DocumentDB => {
                let cluster_name = match &self.cluster_name {
                    None => {
                        return Err(Error::Config("cluster name required".to_string()));
                    }
                    Some(c) => c,
                };
                let region = match &self.region {
                    None => {
                        return Err(Error::Config("region required".to_string()));
                    }
                    Some(r) => r,
                };

                Ok(format!("mongodb://{}:{}@{}.node.{}.docdb.amazonaws.com:27017/?tls=true&tlsCAFile=global-bundle.pem", self.username, self.password, cluster_name, region))
            }
            ContextKind::Mongo => Ok(format!(
                "mongodb://{}:{}@{}:{}",
                self.username, self.password, self.host, self.port
            )),
            // ContextKind::Custom => match *self.connection_uri_provider {
            //     None => {
            //         return Err(Error::Config(
            //             "provider required for custom context".to_string(),
            //         ));
            //     }
            //     Some(provider) => provider.connection_uri(),
            // },
        }
    }
}

/// Opinionated interface for Mongo Documents. Allows callers to avoid a direct dependency on the Mongo Driver.
pub trait MongoDocument:
    Clone + Debug + for<'a> Deserialize<'a> + DeserializeOwned + Send + Serialize + Sized + Sync + Unpin
{
    /// The unique identifier for the document.
    fn id(&self) -> String;
    /// Allows a store to set the id on a document instance.
    fn set_id(&mut self, id: String);
    /// The struct instance type for the document.
    fn type_name() -> String;
    /// The name of the [Collection] that stores documents for this type.
    fn collection() -> String;
}

/// Macro to expand a struct so that it can be consumed by a Mongo [Store].
#[macro_export]
macro_rules! mongo_doc {
    ($t:ty) => {
        impl MongoDocument for $t {
            fn id(&self) -> String {
                self.id.clone()
            }

            fn set_id(&mut self, id: String) {
                self.id = id;
            }

            fn type_name() -> String {
                format!("{}", std::any::type_name::<$t>())
            }

            fn collection() -> String {
                let type_name = Self::type_name();
                type_name.split(':').next_back().unwrap().to_string()
            }
        }
    };
}

pub use mongo_doc;

mongo_doc!(Group);
mongo_doc!(User);
mongo_doc!(Policy);
mongo_doc!(Role);
mongo_doc!(Resource);
mongo_doc!(migrations::LogEntry);

#[cfg(test)]
mod tests {
    use crate::auth::Group;
    use crate::mongodb::{Context, ContextKind, Store};
    use crate::Error;
    use std::collections::HashMap;
    use uuid::Uuid;

    fn context() -> Context {
        Context {
            kind: ContextKind::Mongo,
            // connection_uri_provider: None,
            host: "localhost".to_string(),
            username: "root".to_string(),
            password: "harbor".to_string(),
            port: 27017,
            db_name: "platform".to_string(),
            key_name: "id".to_string(),
            region: None,
            cluster_name: None,
            account_name: None,
        }
    }

    #[async_std::test]
    async fn can_push_embedded_array_member() -> Result<(), Error> {
        let cx = context();
        let store = Store::new(&cx).await?;

        let group = Group {
            id: Uuid::new_v4().to_string(),
            name: "embedded_users".to_string(),
            users: vec!["existing_value".to_string()],
            roles: vec![],
        };

        store.insert(&group).await?;

        let persisted = store.find::<Group>(group.id.as_str()).await?;
        let persisted = persisted.unwrap();
        assert_eq!(group.id, persisted.id);

        store
            .update_ad_hoc::<Group>(
                group.id.as_str(),
                None,
                "$push",
                HashMap::from([("users", "new_value")]),
            )
            .await?;

        let persisted = store.find::<Group>(group.id.as_str()).await?;
        let persisted = persisted.unwrap();

        assert!(persisted.users.contains(&"existing_value".to_string()));
        assert!(persisted.users.contains(&"new_value".to_string()));

        store.delete::<Group>(group.id.as_str()).await?;

        Ok(())
    }

    #[async_std::test]
    async fn cannot_update_ad_hoc_by_non_unique_field() -> Result<(), Error> {
        let cx = context();
        let store = Store::new(&cx).await?;

        let group = Group {
            id: Uuid::new_v4().to_string(),
            name: "duplicate_name".to_string(),
            users: vec!["existing_value".to_string()],
            roles: vec![],
        };
        store.insert(&group).await?;

        let duplicate_name = Group {
            id: Uuid::new_v4().to_string(),
            name: "duplicate_name".to_string(),
            users: vec!["existing_value".to_string()],
            roles: vec![],
        };
        store.insert(&duplicate_name).await?;

        assert!(store
            .update_ad_hoc::<Group>(
                group.name.as_str(),
                Some("name"),
                "$push",
                HashMap::from([("users", "new_value")]),
            )
            .await
            .is_err());

        store.delete::<Group>(group.id.as_str()).await?;
        store.delete::<Group>(duplicate_name.id.as_str()).await?;

        Ok(())
    }
}
