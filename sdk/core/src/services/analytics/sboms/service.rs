use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use crate::entities::datasets::PurlPlusId;
use crate::services::analytics::StorageProvider;
use crate::Error;
use platform::mongo_doc;
use platform::persistence::mongodb::analytics::{Pipeline, Stage};
use platform::persistence::mongodb::{MongoDocument, Service as MongoService, Store as MongoStore};
use serde_json::json;
#[allow(unused_imports)]
use tracing::trace;

/// Get Purls Analytic
fn get_match_primaries_stage() -> Stage {
    Stage::new(json!({
        "$match": {
            "kind": "primary"
        }
    }))
}

/// Get Purls Analytic
fn get_primary_purls_stage() -> Stage {
    Stage::new(json!({
        "$project": {
          "_id": 0,
          "purl": 1,
        }
    }))
}

/// Get Purls Analytic
fn get_group_purls_stage() -> Stage {
    Stage::new(json!({
        "$group": {
            "_id": "",
            "purls": {
              "$push": {
                "purl": "$purl"}
                }
            }
        }
    ))
}

/// Stage 1 in the Report analytic
fn report_analytic_stage_1(purl: String) -> Stage {
    let purl_no_quotes = purl.replace('"', "");

    Stage::new(json!({
        "$match": {
            "purl": purl_no_quotes
        }
    }))
}

/// True Stage 2 in the analytic - sort by timestamp desc to get latest Sbom at the top.
fn sort_by_timestamp_desc() -> Stage {
    Stage::new(json!({
        "$sort": {
            "timestamp": -1,
        }
    }))
}

/// True Stage 3 in the analytic - take only latest Sbom instead of all instances.
fn limit_1() -> Stage {
    Stage::new(json!({
    "$limit": 1,
    }))
}

/// Stage 2 in the Report analytic
fn report_analytic_stage_2() -> Stage {
    // Notice in that we are completing the extraction of the FISMA ID from the previous step, in
    // this projection.
    Stage::new(json!({
        "$project": {
            "_id": 0,
            "name": "$componentName",
            "version": 1,
            "purl": 1,
            "created": 1,
            "packageManager": 1,
            "provider": 1,
            "dependencyRefs": 1,
        }
    }))
}

/// Stage 3 in the Report analytic
fn report_analytic_stage_3() -> Stage {
    Stage::new(json!({
        "$lookup": {
            "from": "Package",
            "localField": "dependencyRefs",
            "foreignField": "purl",
            "as": "report",
        }
    }))
}

/// Stages 4 and 7 in the Report analytic
fn report_analytic_stage_4_and_7() -> Stage {
    Stage::new(json!({
        "$unwind": {
            "path": "$report",
            "preserveNullAndEmptyArrays": true
        }
    }))
}

/// Stage 5 in the Report analytic
fn report_analytic_stage_5() -> Stage {
    Stage::new(json!({
        "$addFields": {
          "report": {
            "name": "$report.cdx.name",
            "kind": "$report.kind",
            "packageManager": "$report.packageManager",
            "purl": "$report.purl",
            "version": "$report.version",
          },
        }
    }))
}

/// Stage 6 in the Report analytic
fn report_analytic_stage_6() -> Stage {
    Stage::new(json!({
        "$group": {
          "_id": "$_id",
          "name": {
            "$first": "$name"
          },
          "packageManager": {
            "$first": "$packageManager"
          },
          "purl": {
            "$first": "$purl"
          },
          "provider": {
            "$first": "$provider"
          },
          "version": {
            "$first": "$version"
          },
          "created": {
            "$first": "$created"
          },
          "report": {
            "$push": "$report",
          },
        },
    }))
}

/// Stage 8 in the Report analytic
fn report_analytic_stage_8() -> Stage {
    Stage::new(json!({
        "$addFields": {
          "report.snyk_enrichment": {
            "provider": "Snyk",
          },
        }
    }))
}

/// Stage 9 in the Report analytic
fn report_analytic_stage_9() -> Stage {
    Stage::new(json!({
        "$lookup": {
            "from": "Vulnerability",
            "localField": "report.purl",
            "foreignField": "purl",
            "as": "report.snyk_enrichment.results",
        }
    }))
}

/// Stage 10 in the Report analytic
fn report_analytic_stage_10() -> Stage {
    Stage::new(json!({
       "$addFields":{
          "report.snyk_enrichment.results":{
             "$map":{
                "input":"$report.snyk_enrichment.results",
                "in":{
                   "severity":"$$this.severity",
                   "cve":"$$this.cve",
                   "description":"$$this.description",
                   "epssScore":"$$this.epssScore",
                   "cvss":"$$this.cvss",
                   "cwes":"$$this.cwes",
                   "remediation":"$$this.remediation"
                }
             }
          }
       }
    }))
}

/// Stage 11 in the Report analytic
fn report_analytic_stage_11() -> Stage {
    Stage::new(json!({
        "$group": {
            "_id": "$_id",
            "name": {
                "$first": "$name"
            },
            "packageManager": {
                "$first": "$packageManager"
            },
            "purl": {
                "$first": "$purl"
            },
            "provider": {
                "$first": "$provider"
            },
            "version": {
                "$first": "$version"
            },
            "created": {
                "$first": "$created"
            },
            "report": {
                "$push": {
                    "name": "$report.name",
                    "version": "$report.version",
                    "purl": "$report.purl",
                    "packageManager": "$report.packageManager",
                    "enrichments": [
                        "$report.snyk_enrichment"
                    ]
                }
            }
        }
    }))
}

/// Service to create and run analytics on DocumentDB
pub struct AnalyticService {
    pub(crate) store: Arc<MongoStore>,
    pub(crate) storage: Option<Arc<dyn StorageProvider>>,
    pub(crate) pipeline: Pipeline,
}

impl AnalyticService {
    /// Creates a new AnalyticService
    pub fn new(store: Arc<MongoStore>, storage: Option<Arc<dyn StorageProvider>>) -> Self {
        let pipeline = Pipeline::new(store.clone());

        AnalyticService {
            store,
            storage,
            pipeline,
        }
    }
}

impl AnalyticService {
    /// Queries MongoDB to get all of the purls for the primary SBOMs
    pub(crate) async fn get_primary_purls(&self) -> Result<Option<Vec<String>>, Error> {
        self.pipeline.add_stage(get_match_primaries_stage());

        self.pipeline.add_stage(get_primary_purls_stage());

        self.pipeline.add_stage(get_group_purls_stage());

        match self.pipeline.execute_on("Package").await {
            Ok(json) => match json.get("purls") {
                Some(purls) => {
                    return match purls.as_array() {
                        Some(value_array) => {
                            let mut purls: Vec<String> = vec![];
                            for value in value_array {
                                let purl = value.get("purl").unwrap().to_string();
                                purls.push(purl);
                            }

                            Ok(Some(purls))
                        }
                        None => Err(Error::Analytic(String::from(
                            "Unable to convert array of Values to array of purls",
                        ))),
                    }
                }
                None => Err(Error::Analytic(String::from(
                    "Getting primary SBOM purls: No 'purls' key in the JSON",
                ))),
            },
            Err(err) => Err(Error::Analytic(format!(
                "Problem executing analytic: {}",
                err
            ))),
        }
    }

    /// Method to get dependent packages that have a null cpe.
    pub async fn get_dependant_package_purls_with_null_cpe(
        &self,
    ) -> Result<Vec<PurlPlusId>, Error> {
        // Analytic is executed on the "Package" collection
        let collection = "Package";

        // Make sure the pipeline is clear. Is this necessary?
        self.pipeline.clear();

        // ==> Add the stages

        // Get all of the documents that are dependencies and have a null `cpe` field.
        // from the "Package" collection.
        self.pipeline.add_stage(Stage::new(
            json!({ "$match": { "kind": "dependency", "cpe": null } }),
        ));

        // Project only the purl field and ignore the rest of the other fields
        self.pipeline
            .add_stage(Stage::new(json!({ "$project": { "purl": 1, "id": 1 } })));

        // Group all of the documents into a single document by pushing all of the
        // purls into an array called "purls"
        self.pipeline.add_stage(Stage::new(json!({ "$group":
            { "_id": "", "purls": { "$push": { "id": "$id", "purl": "$purl" } } }
        })));

        // Execute the Analytic and return the Serde Value object
        let json = match self.pipeline.execute_on(collection).await {
            Ok(json) => json,
            Err(err) => {
                return Err(Error::Analytic(format!(
                    "Problem executing analytic: {}",
                    err
                )))
            }
        };

        if json.as_object().unwrap().is_empty() {
            return Ok(vec![])
        }

        // Deserialize the Serde Value object into this ad-hoc struct
        #[derive(Deserialize, Debug)]
        struct DeserializedValue {
            purls: Vec<PurlPlusId>,
        }
        let deserialized_value =
            serde_json::from_value::<DeserializedValue>(json).map_err(Error::Serde)?;

        // Return the Vector of purls
        Ok(deserialized_value.purls)
    }

    /// Generates a Detail Analytic Report. Specification is here:
    pub(crate) async fn generate_detail(&self, purl: String) -> Result<Option<String>, Error> {
        println!("==> pipeline stages on enter {}", self.pipeline.len());
        self.pipeline.clear();

        self.pipeline
            .add_stage(report_analytic_stage_1(purl.clone()));

        self.pipeline.add_stage(sort_by_timestamp_desc());

        self.pipeline.add_stage(limit_1());

        self.pipeline.add_stage(report_analytic_stage_2());

        self.pipeline.add_stage(report_analytic_stage_3());

        self.pipeline.add_stage(report_analytic_stage_4_and_7());

        self.pipeline.add_stage(report_analytic_stage_5());

        self.pipeline.add_stage(report_analytic_stage_6());

        self.pipeline.add_stage(report_analytic_stage_4_and_7());

        self.pipeline.add_stage(report_analytic_stage_8());

        self.pipeline.add_stage(report_analytic_stage_9());

        self.pipeline.add_stage(report_analytic_stage_10());

        self.pipeline.add_stage(report_analytic_stage_11());

        let json = match self.pipeline.execute_on("Sbom").await {
            Ok(json) => json,
            Err(err) => {
                return Err(Error::Analytic(format!(
                    "Problem executing analytic: {}",
                    err
                )))
            }
        };

        println!("==> pipeline stages after execute {}", self.pipeline.len());

        let storage = match self.storage.clone() {
            Some(storage) => storage,
            None => return Err(Error::Analytic(String::from("No storage provider set"))),
        };

        match storage.write(purl.as_str(), json, "detailed-report").await {
            Ok(path) => Ok(Some(path)),
            Err(e) => Err(Error::Analytic(format!(
                "vulnerability::store_by_purl::write::{}",
                e
            ))),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct NoModel {
    id: String,
}
mongo_doc!(NoModel);

#[async_trait]
impl MongoService<NoModel> for AnalyticService {
    fn store(&self) -> Arc<MongoStore> {
        self.store.clone()
    }
}

impl Debug for AnalyticService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnalyticService")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use platform::persistence::mongodb::analytics::test_context;
    use platform::persistence::mongodb::{Context, Store as MongoStore};

    // Mock StorageProvider implementation for testing
    #[derive(Debug)]
    struct MockStorageProvider;

    #[async_trait]
    impl StorageProvider for MockStorageProvider {
        async fn write(
            &self,
            purl: &str,
            _data: serde_json::Value,
            report_type: &str,
        ) -> Result<String, Error> {
            // Mock implementation to return a path
            Ok(format!("/path/to/{}_{}.json", purl, report_type))
        }
    }

    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_get_dependant_package_purls_with_null_cpe() {
        // Mock store and storage provider
        let cxt: &Context = &test_context(Some("harbor")).expect("Unable to create a test context");
        let raw_store = MongoStore::new(cxt)
            .await
            .expect("Unable to unwrap MongoStore");
        let store = Arc::new(raw_store);
        let storage = Arc::new(MockStorageProvider);

        // Create AnalyticService
        let analytic_service = AnalyticService::new(store, Some(storage));

        let purls: Vec<PurlPlusId> = analytic_service
            .get_dependant_package_purls_with_null_cpe()
            .await
            .expect("Error getting package deps with null cpe");

        println!(">>>>> Purls: {:#?}", purls)
    }

    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_get_primary_purls() {
        // Mock store and storage provider
        let cxt: &Context = &test_context(Some("harbor")).expect("Unable to create a test context");
        let raw_store = MongoStore::new(cxt)
            .await
            .expect("Unable to unwrap MongoStore");
        let store = Arc::new(raw_store);
        let storage = Arc::new(MockStorageProvider);

        // Create AnalyticService
        let analytic_service = AnalyticService::new(store, Some(storage));

        // Execute get_primary_purls
        let result = analytic_service.get_primary_purls().await;

        // Check the result
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn test_generate_detail() {
        // Mock store and storage provider
        let cxt: &Context = &test_context(Some("harbor")).expect("Unable to create a test context");
        let raw_store = MongoStore::new(cxt)
            .await
            .expect("Unable to unwrap MongoStore");
        let store = Arc::new(raw_store);
        let storage = Arc::new(MockStorageProvider);

        // Create AnalyticService
        let analytic_service = AnalyticService::new(store, Some(storage));

        // Execute generate_detail
        let result = analytic_service
            .generate_detail("pkg:npm/bic-api@1.0.0".to_string())
            .await;

        // Check the result
        assert!(result.is_ok());
        let path = result.unwrap().unwrap();
        assert!(path.contains("pkg:npm/bic-api@1.0.0"));
    }

    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn can_clear_pipeline() -> Result<(), Error> {
        // Mock store and storage provider
        let cxt: &Context = &test_context(Some("harbor")).expect("Unable to create a test context");
        let raw_store = MongoStore::new(cxt)
            .await
            .expect("Unable to unwrap MongoStore");
        let store = Arc::new(raw_store);
        let storage: Option<Arc<dyn StorageProvider>> = Some(Arc::new(MockStorageProvider));

        let service = AnalyticService {
            store: store.clone(),
            storage,
            pipeline: Pipeline::new(store),
        };

        let primary_purls = service.get_primary_purls().await?.unwrap();

        for purl in primary_purls {
            println!("==> generating detail report for purl {}", purl);

            service.pipeline.add_stage(report_analytic_stage_1(purl));
            service.pipeline.add_stage(sort_by_timestamp_desc());
            service.pipeline.add_stage(limit_1());
            service.pipeline.add_stage(report_analytic_stage_2());
            service.pipeline.add_stage(report_analytic_stage_3());
            service.pipeline.add_stage(report_analytic_stage_4_and_7());
            service.pipeline.add_stage(report_analytic_stage_5());
            service.pipeline.add_stage(report_analytic_stage_6());
            service.pipeline.add_stage(report_analytic_stage_4_and_7());
            service.pipeline.add_stage(report_analytic_stage_8());
            service.pipeline.add_stage(report_analytic_stage_9());
            service.pipeline.add_stage(report_analytic_stage_10());
            service.pipeline.add_stage(report_analytic_stage_11());

            assert_eq!(service.pipeline.len(), 15);
            service.pipeline.clear();
            assert!(service.pipeline.is_empty());
        }

        Ok(())
    }
}
