use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use async_trait::async_trait;

use serde::{
    Deserialize,
    Serialize
};
use uuid::Uuid;

use platform::errors::{
    Error as PlatformError
};

use platform::persistence::mongodb::{
    Service as MongoService,
    MongoDocument,
    Store as MongoStore,
    mongo_doc,
    Context
};
use crate::config::dev_context;
use crate::services::github::error::Error;

#[derive(Clone)]
/// Database service for the GitHubProvider
pub struct GitHubProviderMongoService {
    pub(crate) store: Arc<MongoStore>
}

impl GitHubProviderMongoService {

    /// Conventional Constructor
    pub fn new(store: Arc<MongoStore>) -> Self {
        GitHubProviderMongoService {
            store
        }
    }

    /// Given the Mongo Collection we want to put a document into
    /// and the Harbor Entities, put this document in Mongo
    pub(crate) async fn create_document(
        &self, url: String, last_hash: String
    ) -> Result<GitHubSbomProviderEntry, Error> {

        let mut document = GitHubSbomProviderEntry {
            id: url.to_string(),
            last_hash: last_hash.to_string(),
        };

        match self.insert(&mut document).await {
            Ok(result) => result,
            Err(err) => return Err(Error::MongoDb(err)),
        };

        println!("==> Added NEW Document to MongoDB: {:#?}", document);

        Ok(document)
    }
}

#[async_trait]
impl MongoService<GitHubSbomProviderEntry>
    for GitHubProviderMongoService {

    fn store(&self) -> Arc<MongoStore> {
        self.store.clone()
    }

    /// Insert a document into a [Collection].
    async fn insert<'a>(
        &self, doc: & mut GitHubSbomProviderEntry
    ) -> Result<(), PlatformError> {

        let id = doc.id();
        if id.is_empty() {
            doc.set_id(
                Uuid::new_v4().to_string()
            );
        }

        return match self.store().insert::<GitHubSbomProviderEntry>(doc).await {
            Ok(_rsp) => Ok(()),
            Err(err) => Err(err)
        }
    }
}

impl Debug for GitHubProviderMongoService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GitHubProvider")
    }
}

/// Struct to define a GitHub Provider document in Mongo
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GitHubSbomProviderEntry {
    /// Url of the GitHub Repository
    pub id: String,
    /// Last commit hash of the repository
    pub last_hash: String,
}
mongo_doc!(GitHubSbomProviderEntry);

#[tokio::test]
#[ignore = "manual_debug_test"]
async fn test_add_document() {

    let ctx = match dev_context(Some(harbor)) {
      Ok(ctx) => ctx,
        Err(err) => panic!("{}", err)
    };

    let svc = GitHubProviderMongoService {
        store: Arc::new(
            match MongoStore::new(&ctx).await {
                Ok(store) => store,
                Err(err) => panic!("Error getting store: {}", err) // test panic
            }
        )
    };

    let id = String::from("test-url-id");
    let last_hash = String::from("test-last-hash");

    let entry = &mut GitHubSbomProviderEntry {
        id: id.clone(),
        last_hash: last_hash.clone(),
    };

    match svc.insert(entry).await {
        Ok(_result) => {
            match svc.find(id.as_str()).await {
                Ok(opt) => match opt {
                    Some(doc) => {
                        assert_eq!(id, doc.id.clone());
                        assert_eq!(last_hash, doc.last_hash);
                    },
                    None => panic!("No value in Option: Missing GitHubSbomProviderEntry") // test panic
                },
                Err(err) => panic!("Error getting GitHubSbomProviderEntry: {}", err) // test panic
            }
        },
        Err(err) => panic!("Unable to insert document into Mongo: {}", err) // test panic
    }
}
