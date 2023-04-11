use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use serde::{
    Deserialize,
    Serialize
};
use platform::Error;

use platform::mongodb::{
    Service as MongoService,
    Context as MongoContext,
    MongoDocument,
    Store,
    client_from_context,
    mongo_doc,
};

use crate::services::github::GhProviderError;
use crate::services::providers::github::GhProviderError;

pub struct GitHubProviderMongoService {
    pub(crate) name: String
}

impl MongoService<GitHubSbomProviderEntry>
    for GitHubProviderMongoService {
    fn store(&self) -> Arc<Store> {
        Arc::new(
            Store::new(
                &MongoContext {
                    host: "localhost".to_string(),
                    username: "root".to_string(),
                    password: "harbor".to_string(),
                    db_name: "harbor".to_string(),
                    key_name: "last_hash".to_string(),
                    port: 0,
                }
            )
        )
    }
}

impl Debug for GitHubProviderMongoService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GitHubProvider, name: {}", self.name)
    }
}

/// Struct to define a GitHub Provider document in Mongo
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GitHubSbomProviderEntry {

    /// Unique id of the [GitHubSbomProviderEntry].
    pub id: String,

    /// Url of the GitHub Repository
    pub repo_url: String,

    /// Last commit hash of the repository
    pub last_hash: String,

    /// Harbor v1 team id
    pub team_id: String,

    /// Harbor v1 project id
    pub project_id: String,

    /// Harbor v1 codebase id
    pub codebase_id: String,
}
mongo_doc!(GitHubSbomProviderEntry);

/// Function to update the last commit hash in a document
///
pub async fn update_last_hash_in_mongo(
    document: GitHubSbomProviderEntry,
    collection: Collection<GitHubSbomProviderEntry>,
    last_hash: String,
) {

    println!("Updating the last hash in Mongo!");

    // Create a filter to find the document we are looking for
    // by id.  Probably supposed to be using _id, but whatever for now.
    let filter = doc! {
        "id": document.id.to_string()
    };

    // This document is used by MongoDB to actually
    // set the new sha hash value on the record
    let update_document = doc! {
        "$set": {
            "last_hash": last_hash.clone()
        }
    };

    // Update the last hash in MongoDB
    match collection.update_one(filter, update_document, None).await {
        Ok(result) => result,
        Err(err) => panic!("Error attempting to insert a document: {}", err)
    };

    println!(
        "Updated EXISTING Document in MongoDB: {:#?}, with hash: {}",
        &document.id, last_hash
    );
}

/// Function to connect and return a MongoDB Database
///
pub async fn get_mongo_db() -> Result<Database, GhProviderError> {

    let ctx = MongoContext::default();

    let result = MongoClient::with_uri_str(ctx.connection_uri()).await;
    let client = match result {
        Ok(client) => client,
        Err(err) => return Err(
            GhProviderError::MongoDb(
                format!("Unable to get the Mongo Client: {}", err)
            )
        )
    };

    // Get a handle to the database.
    Ok(client.database(&ctx.db_name))
}

#[tokio::test]
async fn test_add_document() {

    let svc = GitHubProviderMongoService {
        name: String::from("test-name")
    };

    let id = String::from("test-id");
    let repo_url = String::from("test-url");
    let last_hash = String::from("test-last-hash");
    let team_id = String::from("test-team-id");
    let project_id = String::from("test-project-id");
    let codebase_id = String::from("test-codebase-id");

    svc.insert(
        &mut GitHubSbomProviderEntry {
            id,
            repo_url,
            last_hash,
            team_id,
            project_id,
            codebase_id,
        }
    );

    match svc.find(id.as_str()) {
        Ok(opt) => match opt {
            Some(doc) => {
                assert_eq!(id, doc.id);
                assert_eq!(repo_url, doc.repo_url);
                assert_eq!(last_hash, doc.last_hash);
                assert_eq!(team_id, doc.team_id);
                assert_eq!(project_id, doc.project_id);
                assert_eq!(codebase_id, doc.codebase_id);
            },
            None => panic!("No value in Option: Missing GitHubSbomProviderEntry")
        },
        Err(err) => panic!("Error getting GitHubSbomProviderEntry: {}", err)
    }
}
