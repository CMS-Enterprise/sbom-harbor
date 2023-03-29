use mongodb::{Client as MongoClient, Collection, Cursor, Database};
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};

use platform::mongodb::{Context as MongoContext, mongo_doc, MongoDocument};
use platform::mongodb::service::Service;

use crate::commands::github::{
    DB_IDENTIFIER,
    KEY_NAME,
    GhProviderError,
};

/// Struct to define a GitHub Provider document in Mongo
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GitHubCrawlerMongoDocument {
    /// Unique id of the [GitHubCrawlerMongoDocument].
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
mongo_doc!(GitHubCrawlerMongoDocument);

/// LocalContext struct; probably going away after the code review.
///
pub struct LocalContext;

/// Tiny implementation for LocalContext
///
impl LocalContext {
    pub fn connection_string() -> String {
        String::from("mongodb://localhost:27017")
    }
}

/// Function to update the last commit hash in a document
///
pub async fn update_last_hash_in_mongo(
    document: GitHubCrawlerMongoDocument,
    collection: Collection<GitHubCrawlerMongoDocument>,
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
    let update_document = doc!{
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

    let ctx = MongoContext {
        connection_uri: LocalContext::connection_string(),
        db_name: DB_IDENTIFIER.to_string(),
        key_name: KEY_NAME.to_string(),
    };

    let result = MongoClient::with_uri_str(ctx.connection_uri.clone()).await;
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
