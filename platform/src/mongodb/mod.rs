use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub use service::*;
pub use store::*;

use crate::auth::*;

pub mod migrations;
pub mod service;
pub mod store;
pub mod auth;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: u32,
    pub db_name: String,
    pub key_name: String,
}

// Used by tests. Assumes devenv is running when environment variable is not set.
impl Default for Context {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            username: "root".to_string(),
            password: "harbor".to_string(),
            port: 27017,
            db_name: "".to_string(),
            key_name: "".to_string(),
        }
    }
}

impl Context {
    pub fn connection_uri(&self) -> String {
        format!("mongodb://{}:{}@{}:{}", self.username, self.password, self.host, self.port)
    }
}

/// Opinionated interface for Mongo Documents. Allows callers to avoid a direct dependency on the Mongo Driver.
pub trait MongoDocument: Clone + Debug + for<'a> Deserialize<'a> + DeserializeOwned + Send + Serialize + Sized + Sync + Unpin {
    fn id(&self) -> String;
    fn set_id(&mut self, id: String);
    fn type_name() -> String;
    fn collection() -> String;
}

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
