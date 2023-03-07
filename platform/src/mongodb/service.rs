use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

use crate::Error;
use crate::mongodb::{MongoDocument, Store};

#[async_trait]
pub trait Service<D>: Debug + Send + Sync
    where
        D: MongoDocument,
{
    fn store(&self) -> Arc<Store>;

    #[instrument]
    async fn find(&self, id: &str) -> Result<Option<D>, Error> {
        self.store().find::<D>(id).await
    }

    #[instrument]
    async fn list(&self) -> Result<Vec<D>, Error> {
        self.store().list::<D>().await
    }

    #[instrument]
    async fn insert<'a>(&self, doc: & mut D) -> Result<(), Error> {
        let id = doc.id();
        if !id.is_empty() {
            return Err(Error::Insert("client generated ids not supported".to_string()));
        }

        let id = Uuid::new_v4().to_string();
        doc.set_id(id);

        self.store().insert::<D>(doc).await?;
        Ok(())
    }

    #[instrument]
    async fn update(&self, doc: &D) -> Result<(), Error> {
        let existing = self.store().find::<D>(doc.id().as_str()).await?;
        if existing.is_none() {
            return Err(Error::Update("item does not exists".to_string()));
        }

        self.store().update::<D>(doc).await
    }

    #[instrument]
    async fn delete(&self, id: &str) -> Result<(), Error> {
        self.store().delete::<D>(id).await
    }

    #[instrument]
    async fn query(&self, filter: HashMap<&str, &str>) -> Result<Vec<D>, Error> {
        self.store().query::<D>(filter).await
    }
}
