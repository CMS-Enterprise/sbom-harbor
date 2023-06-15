use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use async_trait::async_trait;

#[allow(unused_imports)]
use tracing::trace;
use platform::mongodb::{
    Service as MongoService,
    Store as MongoStore,
};
use platform::mongodb::analytics::{Pipeline, Stage};
use serde_json::json;
use crate::entities::analytics::detail::Manifest;
use crate::Error;
use crate::services::analytics::StorageProvider;

/// Get Purls Analytic
fn get_match_primaries_stage() -> Stage {
    Stage::new(
        json!({
            "$match": {
                "kind": "primary"
            }
        })
    )
}

/// Get Purls Analytic
fn get_primary_purls_stage() -> Stage {
    Stage::new(
        json!({
            "$project": {
              "_id": 0,
              "purl": 1,
            }
        })
    )
}

/// Get Purls Analytic
fn get_group_purls_stage() -> Stage {
    Stage::new(
        json!({
            "$group": {
                "_id": "",
                "purls": {
                  "$push": {
                    "purl": "$purl"}
                    }
                }
            }
        )
    )
}

/// Stage 1 in the Report analytic
fn report_analytic_stage_1(purl: String) -> Stage {

    let purl_no_quotes = purl.replace('"', "");

    Stage::new(
        json!({
            "$match": {
                "purl": purl_no_quotes
            }
        })
    )
}


/// Stage 2 in the Report analytic
fn report_analytic_stage_2() -> Stage {
    Stage::new(
        json!({
            "$project": {
                "_id": 0,
                "name": "$componentName",
                "version": 1,
                "purl": 1,
                "timestamp": 1,
                "packageManager": 1,
                "provider": 1,
                "dependencyRefs": 1
            }
        })
    )
}

/// Stage 3 in the Report analytic
fn report_analytic_stage_3() -> Stage {
    Stage::new(
        json!({
            "$lookup": {
                "from": "Package",
                "localField": "dependencyRefs",
                "foreignField": "purl",
                "as": "report",
            }
        })
    )
}

/// Stages 4 and 7 in the Report analytic
fn report_analytic_stage_4_and_7() -> Stage {
    Stage::new(
        json!({
            "$unwind": {
                "path": "$report",
                "preserveNullAndEmptyArrays": true
            }
        })
    )
}

/// Stage 5 in the Report analytic
fn report_analytic_stage_5() -> Stage {
    Stage::new(
        json!({
            "$addFields": {
              "report": {
                "name": "$report.cdx.name",
                "kind": "$report.kind",
                "packageManager": "$report.packageManager",
                "purl": "$report.purl",
                "version": "$report.version",
              },
            }
        })
    )
}

/// Stage 6 in the Report analytic
fn report_analytic_stage_6() -> Stage {
    Stage::new(
        json!({
            "$group": {
              "_id": "$_id",
              "report": {
                "$push": "$report",
              },
            },
        })
    )
}

/// Stage 8 in the Report analytic
fn report_analytic_stage_8() -> Stage {
    Stage::new(
        json!({
            "$addFields": {
              "report.snyk_enrichment": {
                "provider": "Snyk",
              },
            }
        })
    )
}

/// Stage 9 in the Report analytic
fn report_analytic_stage_9() -> Stage {
    Stage::new(
        json!({
            "$lookup": {
                "from": "Vulnerability",
                "localField": "report.purl",
                "foreignField": "purl",
                "as": "report.snyk_enrichment.results",
            }
        })
    )
}

/// Stage 10 in the Report analytic
fn report_analytic_stage_10() -> Stage {
    Stage::new(
        json!({
           "$addFields":{
              "report.snyk_enrichment.results":{
                 "$map":{
                    "input":"$report.snyk_enrichment.results",
                    "in":{
                       "severity":"$$this.severity",
                       "cve":"$$this.cve",
                       "description":"$$this.description",
                       "cvss":"$$this.cvss",
                       "cwes":"$$this.cwes",
                       "remediation":"$$this.remediation"
                    }
                 }
              }
           }
        })
    )
}

/// Stage 11 in the Report analytic
fn report_analytic_stage_11() -> Stage {
    Stage::new(
        json!({
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
                    "$first": {
                        "$dateFromString": {
                            "dateString": {
                                "$concat": [
                                    {
                                        "$concat": [
                                            {
                                                "$multiply": [
                                                    "$timestamp",
                                                    1000
                                                ]
                                            },
                                            ""
                                        ]
                                    },
                                    "Z"
                                ]
                            }
                        }
                    }
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
        })
    )
}

/// Service to create and run analytics on DocumentDB
pub struct AnalyticService {
    pub(crate) store: Arc<MongoStore>,
    pub(crate) storage: Arc<dyn StorageProvider>,
    pub(crate) analytic: Pipeline,
}

impl AnalyticService {
    /// Creates a new AnalyticService
    pub fn new(store: Arc<MongoStore>, storage: Arc<dyn StorageProvider>) -> Self {

        let analytic = Pipeline::new(store.clone());

        AnalyticService {
            store,
            storage,
            analytic,
        }
    }
}

impl AnalyticService {

    /// Queries MongoDB to get all of the purls for the primary SBOMs
    pub(crate) async fn get_primary_purls(&self) -> Result<Option<Vec<String>>, Error> {

        self.analytic.add_stage(
            get_match_primaries_stage());

        self.analytic.add_stage(
            get_primary_purls_stage());

        self.analytic.add_stage(
            get_group_purls_stage());

        // TODO Using Platform's Errors.  Need to transform them
        //  into core errors
        match self.analytic.execute_on("Package").await {

            Ok(json) => {

                match json.get("purls") {
                    Some(purls) => return match purls.as_array() {
                        Some(value_array) => {

                            let mut purls: Vec<String> = vec![];
                            for value in value_array {
                                let purl = value.get("purl").unwrap().to_string();
                                purls.push(purl);
                            }

                            Ok(Some(purls))
                        },
                        None => Err(
                            Error::Analytic(
                                String::from(
                                    "Unable to convert array of Values to array of purls"
                                )
                            )
                        )
                    },
                    None => Err(
                        Error::Analytic(
                            String::from(
                                "Getting primary SBOM purls: No 'purls' key in the JSON"
                            )
                        )
                    )
                }
            },
            Err(err) => Err(
                Error::Analytic(
                    format!("Problem executing analytic: {}", err)
                )
            )
        }
    }

    /// Generates a Detail Analytic Report. Specification is here:
    pub(crate) async fn generate_detail(
        &self, purl: String
    ) -> Result<Option<String>, Error> {

        self.analytic.add_stage(
            report_analytic_stage_1(purl.clone()));

        self.analytic.add_stage(
            report_analytic_stage_2());

        self.analytic.add_stage(
            report_analytic_stage_3());

        self.analytic.add_stage(
            report_analytic_stage_4_and_7());

        self.analytic.add_stage(
            report_analytic_stage_5());

        self.analytic.add_stage(
            report_analytic_stage_6());

        self.analytic.add_stage(
            report_analytic_stage_4_and_7());

        self.analytic.add_stage(
            report_analytic_stage_8());

        self.analytic.add_stage(
            report_analytic_stage_9());

        self.analytic.add_stage(
            report_analytic_stage_10());

        self.analytic.add_stage(
            report_analytic_stage_11());

        let json = match self.analytic.execute_on("Sbom").await {
            Ok(json) => {
                json
            },
            Err(err) => {
                return Err(
                    Error::Analytic(
                        format!("Problem executing analytic: {}", err)
                    )
                )
            }
        };

        match self.storage.write(purl.as_str(), json, "detailed-report").await {
            Ok(path) => Ok(Some(path)),
            Err(e) => Err(
                Error::Analytic(
                    format!("vulnerability::store_by_purl::write::{}", e)
                )
            )
        }
    }
}

#[async_trait]
impl MongoService<Manifest> for AnalyticService {
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
    use platform::mongodb::{Context, Store as MongoStore};
    use platform::mongodb::analytics::test_context;

    // Mock StorageProvider implementation for testing
    #[derive(Debug)]
    struct MockStorageProvider;

    #[async_trait]
    impl StorageProvider for MockStorageProvider {
        async fn write(&self, purl: &str, _data: serde_json::Value, report_type: &str) -> Result<String, Error> {
            // Mock implementation to return a path
            Ok(format!("/path/to/{}_{}.json", purl, report_type))
        }
    }

    #[tokio::test]
    async fn test_get_primary_purls() {
        // Mock store and storage provider
        let cxt: &Context = &test_context(Some("harbor"))
            .expect("Unable to create a test context");
        let raw_store = MongoStore::new(cxt).await.expect("Unable to unwrap MongoStore");
        let store = Arc::new(raw_store);
        let storage = Arc::new(MockStorageProvider);

        // Create AnalyticService
        let analytic_service = AnalyticService::new(store, storage);

        // Execute get_primary_purls
        let result = analytic_service.get_primary_purls().await;

        // Check the result
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_detail() {
        // Mock store and storage provider
        let cxt: &Context = &test_context(Some("harbor"))
            .expect("Unable to create a test context");
        let raw_store = MongoStore::new(cxt).await.expect("Unable to unwrap MongoStore");
        let store = Arc::new(raw_store);
        let storage = Arc::new(MockStorageProvider);

        // Create AnalyticService
        let analytic_service = AnalyticService::new(store, storage);

        // Execute generate_detail
        let result = analytic_service.generate_detail("pkg:npm/bic-api@1.0.0".to_string()).await;

        // Check the result
        assert!(result.is_ok());
        let path = result.unwrap().unwrap();
        assert!(path.contains("pkg:npm/bic-api@1.0.0"));
    }
}
