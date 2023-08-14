use crate::hyper;
use crate::mongo_doc;
use crate::persistence::mongodb::MongoDocument;
use crate::persistence::mongodb::{Context, Store};
use crate::Error;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

mongo_doc!(DebugEntity);

/// Type to persist arbitrary data structures to mongo for development and debugging.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DebugEntity {
    /// Unique id required by framework.
    pub id: String,
    /// Free text discriminator to allow filtering types during local debugging.
    pub kind: Option<String>,
    /// Useful for keeping track of arbitrary context values for the request being debugged.
    pub context: Option<String>,
    /// The deserialized target entity.
    pub data: Option<String>,
    /// Provides a way to return the raw result string if conversion to the generic type fails.
    pub raw: Option<String>,
}

/// A client that can be used to debug or reverse engineer calls to external services. This
/// should never be used in production code, but is useful for saving response to a data store
/// for inspection.
#[derive(Debug)]
pub struct Client {
    token: Option<String>,
    kind: Option<String>,
    inner: hyper::Client,
    store: Arc<Store>,
}

impl Client {
    /// Factory method for creating new instances of a DebugClient.
    pub async fn new(
        cx: Context,
        token: Option<String>,
        kind: Option<String>,
    ) -> Result<Client, Error> {
        let inner = hyper::Client::new();
        let store = Arc::new(Store::new(&cx).await?);
        Ok(Self {
            token,
            kind,
            inner,
            store,
        })
    }

    fn token(&self) -> String {
        match &self.token {
            None => "".to_string(),
            Some(t) => t.clone(),
        }
    }
}
