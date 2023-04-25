use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

use crate::mongodb::{MongoDocument, Store};
use crate::Error;

/// [Service] provides consistent, generic persistence capabilities for types that implement the
/// [MongoDocument] trait. It is specialized to the opinionated conventions defined in this crate.
/// It can be thought of as a pre-processor that ensures mandatory generic logic is consistently applied
/// across all operations against a [Store]. Applications should not be aware of the [Store] and should
/// instead leverage a [Service].
#[async_trait]
pub trait Service<D>: Debug + Send + Sync
where
    D: MongoDocument,
{
    // TODO: Refactor this away with a [Context].
    /// Provides access to the [Store] instance for the [Service].
    fn store(&self) -> Arc<Store>;

    /// Find a document within a [Collection] by its id.
    #[instrument]
    async fn find(&self, id: &str) -> Result<Option<D>, Error> {
        self.store().find::<D>(id).await
    }

    /// List all documents within a [Collection].
    #[instrument]
    async fn list(&self) -> Result<Vec<D>, Error> {
        self.store().list::<D>().await
    }

    /// Insert a document into a [Collection].
    #[instrument]
    async fn insert<'a>(&self, doc: &mut D) -> Result<(), Error> {
        let id = doc.id();
        if !id.is_empty() {
            return Err(Error::Insert(
                "client generated ids not supported".to_string(),
            ));
        }

        let id = Uuid::new_v4().to_string();
        doc.set_id(id);

        self.store().insert::<D>(doc).await?;
        Ok(())
    }

    /// Update a document within a [Collection].
    #[instrument]
    async fn update(&self, doc: &D) -> Result<(), Error> {
        let existing = self.store().find::<D>(doc.id().as_str()).await?;
        if existing.is_none() {
            return Err(Error::Update("item does not exists".to_string()));
        }

        self.store().update::<D>(doc).await
    }

    /// Delete a document from a [Collection].
    #[instrument]
    async fn delete(&self, id: &str) -> Result<(), Error> {
        self.store().delete::<D>(id).await
    }

    /// Perform and ad-hoc query against all documents within a [Collection].
    #[instrument]
    async fn query(&self, filter: HashMap<&str, &str>) -> Result<Vec<D>, Error> {
        self.store().query::<D>(filter).await
    }
}
