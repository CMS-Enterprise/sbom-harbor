use std::sync::{Arc, Mutex};
use mongodb::bson;
use mongodb::bson::{Document};
use mongodb::options::AggregateOptions;
use futures_util::StreamExt;

#[allow(unused_imports)]
use serde_json::{json, Value};

#[allow(unused_imports)]
use tracing::trace;
use crate::Error;
use crate::mongodb::Context;

use crate::mongodb::store::Store;

/// Analytic struct uses stages to execute a MongoDB Aggregate Pipeline
#[derive(Debug)]
pub struct Analytic {
    /// Store so we can read from MongoDB
    store: Arc<Store>,
    /// This variable holds all of teh Stages to be executed
    /// in the pipeline.
    pub pipeline: Mutex<Vec<Stage>>
}

impl Analytic {

    /// Creates a new Analytic type
    pub fn new(store: Arc<Store>) -> Self {
        Analytic {
            store,
            pipeline: Mutex::new(vec![]),
        }
    }

    /// This method adds a stage to the Analytic
    pub fn add_stage(&self, stage: Stage) {
        self.pipeline.lock().unwrap().push(stage);
    }

    /// Functions to clear out the pipeline after execution
    pub fn clear_pipeline(&self) {
        self.pipeline.lock().unwrap().clear();
    }

    /// This method executes the Analytic and returns the results as a Serde Value
    pub async fn execute_on(&self, collection: &str) -> Result<Value, Error> {

        if self.pipeline.lock().unwrap().len() == 0 {
            return Err(
                Error::Mongo(
                    String::from("The Aggregation Pipeline is empty")
                )
            )
        }

        let client = &self.store.client();
        let db = client.database(self.store.db_name());
        let collection = db.collection::<Document>(collection);

        // Set the options for the aggregation
        let options = AggregateOptions::builder().build();

        // Map the stages over to Documents
        let doc_pipeline = self.pipeline
            .lock()
            .unwrap()
            .iter()
            .map(|s| s.get_document())
            .collect::<Vec<Document>>();

        // Execute the aggregate pipeline
        let mut cursor = collection.aggregate(doc_pipeline, options).await?;

        // Process the results
        match cursor.next().await {
            Some(result) => {

                self.clear_pipeline();

                match result {
                    Ok(doc) => match serde_json::to_value(&doc) {
                        Ok(value) => Ok(value),
                        Err(err) => Err(
                            Error::Mongo(
                                format!("Error serializing to Json: {}", err)
                            )
                        )
                    },
                    Err(err) => Err(
                        Error::Mongo(
                            format!("Error extracting document from MongoDB Result: {}", err)
                        )
                    )
                }
            },
            None => {

                self.clear_pipeline();

                Err(
                    Error::Mongo(
                        String::from("No result from DocumentDB Aggregate")
                    )
                )
            }
        }
    }
}

/// Stage represents a Stage of a MongoDB Aggregation Pipeline
#[derive(Debug, PartialEq, Clone)]
pub struct Stage {
    json: Value
}

impl Stage {

    /// Method to create a new Stage
    pub fn new(json: Value) -> Self {
        Self { json }
    }

    fn get_document(&self) -> Document {
        match bson::to_document(&self.json) {
            Ok(doc) => doc,
            Err(err) => panic!("Failed to create BSON document: {}", err)
        }
    }
}

#[tokio::test]
async fn analytic_test() {

    let cx = match test_context(None) {
        Ok(cx) => cx,
        Err(e) => {
            trace!("unable to retrieve connection config: {}", e);
            return;
        }
    };

    let store = Arc::new(Store::new(&cx).await.unwrap());
    let analytic = Analytic::new(store);

    let json: Value = json!({
        "$match": {
            "purl": "pkg:npm/bic-api@1.0.0"
        }
    });

    let stage = Stage::new(json);

    analytic.add_stage(stage);

    match analytic.execute_on("Sbom").await {
        Ok(value) => println!("Value: {:#?}", value),
        Err(_) => assert!(false, "Test Failed, got error")
    };
}

/// Create a test context for the test above.
/// For the CODE REVIEW: How should this be done?
pub fn test_context(db_name: Option<&str>) -> Result<Context, Error> {
    let db_name = match db_name {
        None => "harbor",
        Some(db_name) => db_name,
    };

    Ok(
        Context {
        host: "mongo".to_string(),
        username: "root".to_string(),
        password: "harbor".to_string(),
        port: 27017,
        db_name: db_name.to_string(),
        key_name: "id".to_string(),
        connection_uri: None,
    })
}