use futures_util::StreamExt;
use mongodb::bson;
use mongodb::bson::Document;
use mongodb::options::AggregateOptions;
use std::sync::{Arc, Mutex};

#[allow(unused_imports)]
use serde_json::{json, Value};

use crate::persistence::mongodb::{Context, Store};
use crate::Error;
#[allow(unused_imports)]
use tracing::trace;

/// Analytic struct uses stages to execute a MongoDB Aggregate Pipeline
#[derive(Debug)]
pub struct Pipeline {
    /// Store so we can read from MongoDB
    store: Arc<Store>,
    /// This variable holds all of the Stages to be executed
    /// in the pipeline.
    stages: Mutex<Vec<Stage>>,
}

impl Pipeline {
    /// Creates a new Analytic type
    pub fn new(store: Arc<Store>) -> Self {
        Pipeline {
            store,
            stages: Mutex::new(vec![]),
        }
    }

    /// This method adds a stage to the Analytic
    pub fn add_stage(&self, stage: Stage) {
        self.stages.lock().unwrap().push(stage);
    }

    /// Functions to clear out the pipeline after execution
    pub fn clear(&self) {
        self.stages.lock().unwrap().clear();
    }

    /// Gets the current count of stages in the pipeline.
    pub fn len(&self) -> usize {
        self.stages.lock().unwrap().len()
    }

    /// Indicates whether the current pipeline has any queued stages.
    pub fn is_empty(&self) -> bool {
        self.stages.lock().unwrap().is_empty()
    }

    /// This method executes the Analytic and returns the results as a Serde Value
    pub async fn execute_on(&self, collection: &str) -> Result<Value, Error> {
        if self.stages.lock().unwrap().len() == 0 {
            return Err(Error::Mongo(String::from(
                "The Aggregation Pipeline is empty",
            )));
        }

        let client = &self.store.client();
        let db = client.database(self.store.db_name());
        let collection = db.collection::<Document>(collection);

        // Set the options for the aggregation
        let options = AggregateOptions::builder().allow_disk_use(true).build();

        // Map the stages over to Documents
        let doc_pipeline = self
            .stages
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
                self.clear();

                match result {
                    Ok(doc) => match serde_json::to_value(&doc) {
                        Ok(value) => Ok(value),
                        Err(err) => {
                            Err(Error::Mongo(format!("Error serializing to Json: {}", err)))
                        }
                    },
                    Err(err) => Err(Error::Mongo(format!(
                        "Error extracting document from MongoDB Result: {}",
                        err
                    ))),
                }
            }
            None => {
                self.clear();

                // Return an Empty value - this is temporary
                // TODO Refactor this method to return an Option<Value> rather than
                //  an actual Value.  This way we can know fore sure that there wasn't
                //  an Error, but an actual empty result.
                Ok(json!({}))
            }
        }
    }
}

/// Stage represents a Stage of a MongoDB Aggregation Pipeline
#[derive(Debug, PartialEq, Clone)]
pub struct Stage {
    json: Value,
}

impl Stage {
    /// Method to create a new Stage
    pub fn new(json: Value) -> Self {
        Self { json }
    }

    fn get_document(&self) -> Document {
        match bson::to_document(&self.json) {
            Ok(doc) => doc,
            Err(err) => panic!("Failed to create BSON document: {}", err),
        }
    }
}

#[tokio::test]
#[ignore = "debug manual only"]
async fn analytic_test() {
    let cx = match test_context(None) {
        Ok(cx) => cx,
        Err(e) => {
            println!("unable to retrieve connection config: {}", e);
            return;
        }
    };

    let store = Arc::new(Store::new(&cx).await.unwrap());
    let analytic = Pipeline::new(store);

    let json: Value = json!({
        "$match": {
            "purl": "pkg:npm/bic-api@1.0.0"
        }
    });

    let stage = Stage::new(json);

    analytic.add_stage(stage);

    match analytic.execute_on("Sbom").await {
        Ok(value) => println!("Value: {:#?}", value),
        Err(_) => assert!(false, "Test Failed, got error"),
    };
}

/// Create a test context for the test above.
/// For the CODE REVIEW: How should this be done?
pub fn test_context(db_name: Option<&str>) -> Result<Context, Error> {
    let db_name = match db_name {
        None => "harbor",
        Some(db_name) => db_name,
    };

    Ok(Context {
        host: "localhost".to_string(),
        username: "root".to_string(),
        password: "harbor".to_string(),
        port: 27017,
        db_name: db_name.to_string(),
        key_name: "id".to_string(),
        connection_uri: None,
    })
}
