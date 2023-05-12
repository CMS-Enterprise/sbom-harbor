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

/// Trait to implement when injecting a custom uri provider.
pub trait ConnectionUriProvider: Debug + Send + Sync {
    /// Implement this to provide a custom uri.
    fn connection_uri(&self) -> Result<String, Error>;
}

// TODO: Make Context a trait and split these up in to purpose build contexts instead of a single type.
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
    /// Allows dynamically injecting connection uri rather than depending on default logic.
    pub connection_uri: Option<String>,
}

impl Context {
    /// Returns a formatted MongoDB compliant URI for the MongoDB instance.
    pub fn connection_uri(&self) -> Result<String, Error> {
        match &self.connection_uri {
            None => Ok(format!(
                "mongodb://{}:{}@{}:{}",
                self.username, self.password, self.host, self.port
            )),
            Some(connection_uri) => Ok(connection_uri.clone()),
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
    use crate::mongodb::{Context, Store};
    use crate::Error;
    use std::collections::HashMap;
    use uuid::Uuid;

    fn context() -> Context {
        Context {
            host: "mongo".to_string(),
            username: "root".to_string(),
            password: "harbor".to_string(),
            port: 27017,
            db_name: "platform".to_string(),
            key_name: "id".to_string(),
            connection_uri: None,
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
