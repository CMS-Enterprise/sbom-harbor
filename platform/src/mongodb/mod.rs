use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub use service::*;
pub use store::*;

use crate::auth::*;
use crate::Error;

/// Provides MongoDB db migrations support.
pub mod migrations;
/// Provides a generics-based [Service] trait for handling CRUD based operations against a [Store].
pub mod service;
/// Provides a generics-based [Store] trait for handling CRUD based operations against a [Collection].
pub mod store;
/// Provides a row-level authorization mechanism for controlling access to entries in a [Collection].
pub mod auth;

/// Specifies the backend data store for the [Context].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ContextKind {
    /// Specifies the data store is provided by Cosmos DB for MongoDB.
    CosmosDB,
    /// Specifies the data store is provided by DocumentDB.
    DocumentDB,
    /// Specifies the data store is a native MongoDB instance.
    Mongo,
}

// TODO: Make Context a trait and split these up in to purpose build contexts instead of a single type.
/// Provides connection information and schema conventions for a MongoDB/DocumentDB backed [Store].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    /// Backend provider for the [Context].
    pub kind: ContextKind,
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
    /// Returns a formatted MongoDB compliant URI for the MongoDB instance.
    pub fn connection_uri(&self) -> Result<String, Error> {
        match self.kind {
            ContextKind::CosmosDB => {
                match &self.account_name {
                    None => Err(Error::Config("account name required".to_string())),
                    Some(account_name) => {
                        Ok(format!("mongodb://{}:{}@{}.documents.azure.com:10255/?ssl=true", self.username, self.password, account_name))
                    }
                }
            }
            ContextKind::DocumentDB => {
                let cluster_name = match &self.cluster_name {
                    None => {
                        return Err(Error::Config("cluster name required".to_string()));
                    }
                    Some(c) => c
                };
                let region = match &self.region {
                    None => {
                        return Err(Error::Config("region required".to_string()));
                    }
                    Some(r) => r
                };

                Ok(format!("mongodb://{}:{}@{}.node.{}.docdb.amazonaws.com:27017/?tls=true&tlsCAFile=global-bundle.pem", self.username, self.password, cluster_name, region))
            }
            ContextKind::Mongo => {
                Ok(format!("mongodb://{}:{}@{}:{}", self.username, self.password, self.host, self.port))
            }
        }
    }
}

/// Opinionated interface for Mongo Documents. Allows callers to avoid a direct dependency on the Mongo Driver.
pub trait MongoDocument: Clone + Debug + for<'a> Deserialize<'a> + DeserializeOwned + Send + Serialize + Sized + Sync + Unpin {
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
    }
}

pub use mongo_doc;

mongo_doc!(Group);
mongo_doc!(User);
mongo_doc!(Policy);
mongo_doc!(Role);
mongo_doc!(Resource);
mongo_doc!(migrations::LogEntry);
