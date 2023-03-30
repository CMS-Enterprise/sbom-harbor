use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub use service::*;
pub use store::*;

use crate::auth::*;

/// Provides MongoDB db migrations support.
pub mod migrations;
/// Provides a generics-based [Service] trait for handling CRUD based operations against a [Store].
pub mod service;
/// Provides a generics-based [Store] trait for handling CRUD based operations against a [Collection].
pub mod store;
/// Provides a row-level authorization mechanism for controlling access to entries in a [Collection].
pub mod auth;

/// Provides connection information and schema conventions for a MongoDB/DocumentDB backed [Store].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
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
}

// Used by tests. Assumes devenv is running when environment variable is not set.
impl Default for Context {
    fn default() -> Self {
        Self {
            host: "mongo".to_string(),
            username: "root".to_string(),
            password: "harbor".to_string(),
            port: 27017,
            db_name: "harbor".to_string(),
            key_name: "".to_string(),
        }
    }
}

impl Context {
    /// Returns a formatted MongoDB compliant URI for the MongoDB instance.
    pub fn connection_uri(&self) -> String {
        format!("mongodb://{}:{}@{}:{}", self.username, self.password, self.host, self.port)
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
