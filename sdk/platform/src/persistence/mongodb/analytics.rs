use futures_util::StreamExt;
use mongodb::bson;
use mongodb::bson::Document;
use mongodb::options::AggregateOptions;
use std::sync::{Arc, Mutex};

#[allow(unused_imports)]
use serde_json::{json, Value};

use crate::persistence::mongodb::Store;
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
        let options = AggregateOptions::builder().build();

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

                Err(Error::Mongo(String::from(
                    "No result from DocumentDB Aggregate",
                )))
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
