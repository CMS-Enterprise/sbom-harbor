use futures_util::TryStreamExt;
use mongodb::bson::{doc, Bson, Document, SerializerOptions};
use mongodb::{bson, Client, Collection, Database};
use std::collections::HashMap;
use std::fmt::Debug;
use tracing::instrument;

use crate::mongodb::{Context, MongoDocument};
use crate::Error;

/// Default client factory method. Allows callers to avoid a direct dependency on the Mongo Driver.
pub async fn client_from_context(cx: &Context) -> Result<Client, Error> {
    let connection_uri = cx.connection_uri()?;
    let client = Client::with_uri_str(connection_uri.as_str()).await?;
    Ok(client)
}

/// Facade that provides access to a MongoDB compliant data store.
#[derive(Clone, Debug)]
pub struct Store {
    client: Client,
    db_name: String,
    key_name: String,
}

impl Store {
    /// Factory method for creating a new [Store] instance.
    pub async fn new(ctx: &Context) -> Result<Store, Error> {
        let client = client_from_context(ctx).await?;

        let store = Store {
            client,
            db_name: ctx.db_name.clone(),
            key_name: ctx.key_name.clone(),
        };

        Ok(store)
    }

    pub(crate) fn client(&self) -> Client {
        // Always clone the client
        // For explanation, see https://mongodb.github.io/mongo-rust-driver/manual/performance.html
        self.client.clone()
    }

    pub(crate) fn db_name(&self) -> &str {
        self.db_name.as_str()
    }

    fn database(&self) -> Database {
        let client = self.client();
        client.database(self.db_name())
    }

    fn collection<D>(&self) -> Collection<D>
    where
        D: MongoDocument,
    {
        let database = self.database();
        database.collection::<D>(D::collection().as_str())
    }

    /// Find the first item that matches the id and is allowed.
    #[instrument]
    pub async fn find<D>(&self, id: &str) -> Result<Option<D>, Error>
    where
        D: MongoDocument,
    {
        let collection = self.collection();
        let result = collection
            .find_one(doc! { self.key_name.clone(): id }, None)
            .await?;

        Ok(result)
    }

    /// TODO: Review and determine what FindOptions we should support and how we can abstract this.
    /// See [driver docs](https://docs.rs/mongodb/latest/mongodb/options/struct.FindOptions.html)
    /// Likely candidates include:
    /// - limit
    /// - skip
    /// - allow_partial_results
    /// - comment (trace_id support)
    /// - hint (use index advice)
    /// - max_time (timeout)
    /// - return_key (return keys only)
    /// - sort
    /// - show_record_id
    /// List the items from the collection
    #[instrument]
    pub async fn list<D>(&self) -> Result<Vec<D>, Error>
    where
        D: MongoDocument,
    {
        let collection = self.collection();
        let mut cursor = collection.find(None, None).await?;

        let mut result = Vec::new();

        while let Some(item) = cursor.try_next().await? {
            result.push(item)
        }

        Ok(result)
    }

    /// Insert an item in Mongo.
    #[instrument]
    pub async fn insert<D>(&self, doc: &D) -> Result<(), Error>
    where
        D: MongoDocument,
    {
        let collection = self.collection::<D>();
        let result = collection.insert_one(doc, None).await?;

        match result.inserted_id {
            Bson::ObjectId(_) => Ok(()),
            _ => Err(Error::Insert("invalid result id format".to_string())),
        }
    }

    /// Update an item in Mongo.
    #[instrument]
    pub async fn update<D>(&self, doc: &D) -> Result<(), Error>
    where
        D: MongoDocument,
    {
        let collection = self.collection::<D>();
        let id = doc.id();
        let opts = SerializerOptions::builder().human_readable(false).build();

        let doc = bson::to_document_with_options(&doc, opts)
            .map_err(|e| Error::Mongo(format!("error generating document for update: {}", e)))
            .unwrap();

        collection
            .update_one(
                doc! {self.key_name.clone(): id },
                doc! { "$set": doc },
                None,
            )
            .await?;

        Ok(())
    }

    /// Update an embedded document.
    #[instrument]
    pub async fn update_ad_hoc<D>(
        &self,
        key: &str,
        key_name: Option<&str>,
        operator: &str,
        expression: HashMap<&str, &str>,
    ) -> Result<(), Error>
    where
        D: MongoDocument,
    {
        let collection = self.collection::<D>();

        let key_name = match key_name {
            None => self.key_name.as_str(),
            Some(key_name) => key_name,
        };

        if let true = key_name.is_empty() {
            return Err(Error::Mongo("key_name_empty".to_string()));
        }

        let query = doc! { key_name: key };

        // TODO: Documentation says to use estimated_document_count in most cases but that has no
        // filter.
        let doc_count = collection.count_documents(query.clone(), None).await?;
        match doc_count {
            0 => {
                return Err(Error::Mongo("update_ad_hoc_invalid_key".to_string()));
            }
            1 => {}
            _ => {
                return Err(Error::Mongo("update_ad_hoc_duplicate_key".to_string()));
            }
        }

        let mut modifications = Document::new();

        for e in expression {
            modifications.insert(e.0, e.1);
        }

        collection
            .update_one(query, doc! { operator: modifications }, None)
            .await?;

        Ok(())
    }

    /// Delete an item from Mongo.
    #[instrument]
    pub async fn delete<D>(&self, id: &str) -> Result<(), Error>
    where
        D: MongoDocument,
    {
        let collection = self.collection::<D>();
        let result = collection
            .delete_one(doc! {self.key_name.clone(): id }, None)
            .await?;

        if result.deleted_count != 1 {
            return Err(Error::Delete(format!("failed to delete document: {}", id)));
        }

        Ok(())
    }

    /// Query the items that match the filter expression.
    #[instrument]
    pub async fn query<D>(&self, filter_map: HashMap<&str, &str>) -> Result<Vec<D>, Error>
    where
        D: MongoDocument,
    {
        let collection = self.collection();

        let mut filter = Document::new();

        for f in filter_map {
            filter.insert(f.0, f.1);
        }

        let mut cursor = collection.find(Some(filter), None).await?;

        let mut result = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            result.push(item)
        }

        Ok(result)
    }
}
