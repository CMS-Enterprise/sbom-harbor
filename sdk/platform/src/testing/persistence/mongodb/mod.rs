use crate::hyper;
use crate::mongo_doc;
use crate::persistence::mongodb::MongoDocument;
use crate::persistence::mongodb::{Context, Store};
use crate::Error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

mongo_doc!(DebugEntity);

/// Type to persist arbitrary data structures to mongo for development and debugging.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DebugEntity {
    pub id: String,
    pub kind: Option<String>,
    pub data: Option<String>,
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
