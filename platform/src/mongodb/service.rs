use std::collections::HashMap;
use std::fmt::Debug;

use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

use crate::mongodb::{Context, MongoDocument, Store};
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
    fn cx(&self) -> &Context;

    /// Provides access to the [Store] instance for the [Service].
    #[instrument]
    async fn store(&self) -> Result<Store, Error> {
        Store::new(self.cx()).await
    }

    /// Find a document within a [Collection] by its id.
    #[instrument]
    async fn find(&self, id: &str) -> Result<Option<D>, Error> {
        let store = self.store().await?;
        store.find::<D>(id).await
    }

    /// List all documents within a [Collection].
    #[instrument]
    async fn list(&self) -> Result<Vec<D>, Error> {
        let store = self.store().await?;
        store.list::<D>().await
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

        let store = self.store().await?;
        store.insert::<D>(doc).await
    }

    /// Update a document within a [Collection].
    #[instrument]
    async fn update(&self, doc: &D) -> Result<(), Error> {
        let store = self.store().await?;
        let existing = store.find::<D>(doc.id().as_str()).await?;
        if existing.is_none() {
            return Err(Error::Update("item does not exists".to_string()));
        }

        store.update::<D>(doc).await
    }

    // TODO: Constrain to a set of known supported/tested operations.
    /// Update a document within a [Collection] using ad hoc expressions and filters.
    #[instrument]
    async fn update_ad_hoc(
        &self,
        key: &str,
        key_name: Option<&str>,
        operator: &str,
        expression: HashMap<&str, &str>,
    ) -> Result<(), Error> {
        let store = self.store().await?;
        store
            .update_ad_hoc::<D>(key, key_name, operator, expression)
            .await
    }

    /// Delete a document from a [Collection].
    #[instrument]
    async fn delete(&self, id: &str) -> Result<(), Error> {
        let store = self.store().await?;
        store.delete::<D>(id).await
    }

    /// Perform an ad-hoc query against all documents within a [Collection].
    #[instrument]
    async fn query(&self, filter: HashMap<&str, &str>) -> Result<Vec<D>, Error> {
        let store = self.store().await?;
        store.query::<D>(filter).await
    }
}
