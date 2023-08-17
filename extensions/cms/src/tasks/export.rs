use async_trait::async_trait;
use harbcore::entities::tasks::{Task, TaskStatus};
use harbcore::services::analytics::StorageProvider;
use harbcore::tasks::TaskProvider;
use harbcore::Error;
use platform::persistence::mongodb::analytics::{Pipeline, Stage};
use platform::persistence::mongodb::{Service, Store};
use serde_json::json;

use std::collections::HashMap;
use std::sync::Arc;

/// Task to export data to the CMS data store.
#[derive(Debug)]
pub struct ExportTask {
    service: ExportService,
}

impl ExportTask {
    /// Factory method for new instance of type.
    pub fn new(service: ExportService) -> Self {
        Self { service }
    }
}

#[async_trait]
impl TaskProvider for ExportTask {
    async fn run(&self, task: &mut Task) -> Result<HashMap<String, String>, Error> {
        let mut errors = HashMap::<String, String>::new();
        let mut report_paths = Vec::<String>::new();

        let primary_purls = match self.service.get_primary_purls().await {
            Ok(opt) => match opt {
                Some(purls) => purls,
                None => {
                    return Err(Error::Analytic(
                        "Error attempting to get primary purls, none found!".to_string(),
                    ))
                }
            },
            Err(err) => {
                return Err(Error::Analytic(format!(
                    "Error attempting to get primary purls: {err}"
                )))
            }
        };

        task.count = primary_purls.len() as u64;
        self.update(task).await?;

        println!("==> processing {} sboms for detail report", task.count);
        let mut iteration = 0;

        for purl in primary_purls {
            iteration += 1;
            println!(
                "==> generating detail report for iteration {} for purl {}",
                iteration, purl
            );

            match self.service.export_by_purl(purl.as_str()).await {
                Ok(file_path_option) => {
                    if let Some(file_path) = file_path_option {
                        println!("==> Sbom detail report complete for {}", purl);
                        report_paths.push(file_path)
                    }
                }
                Err(err) => {
                    println!("==> Sbom detail report complete for {}", purl);
                    errors.insert(purl, format!("{}", err));
                }
            }
        }

        task.status = TaskStatus::Complete;

        Ok(errors)
    }
}

impl Service<Task> for ExportTask {
    fn store(&self) -> Arc<Store> {
        self.service.store.clone()
    }
}

/// Provides business logic for the daily CMS SDL export.
#[derive(Debug)]
pub struct ExportService {
    pub(crate) store: Arc<Store>,
    pub(crate) storage: Arc<dyn StorageProvider>,
}

impl ExportService {
    /// Factory method for new instance of type.
    pub fn new(store: Arc<Store>, storage: Arc<dyn StorageProvider>) -> Self {
        ExportService { store, storage }
    }

    /// Queries MongoDB to get all of the purls for the primary SBOMs
    pub(crate) async fn get_primary_purls(&self) -> Result<Option<Vec<String>>, Error> {
        let pipeline = Pipeline::new(self.store.clone());
        pipeline.add_stage(Stage::new(json!({
            "$match": {
                "kind": "primary"
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$project": {
              "_id": 0,
              "purl": 1,
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$group": {
                "_id": "",
                "purls": {
                  "$push": {
                    "purl": "$purl"}
                    }
                }
            }
        )));

        match pipeline.execute_on("Package").await {
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
    /// Generate the export document for a given purl.
    pub(crate) async fn export_by_purl(&self, purl: &str) -> Result<Option<String>, Error> {
        let pipeline = Pipeline::new(self.store.clone());
        let purl = purl.replace('"', "");

        pipeline.add_stage(Stage::new(json!({
            "$match": {
                "purl": purl
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$group": {
                "_id": 0,
                "id": {
                    "$first": "$id"
                },
                "name": {
                    "$first": "$componentName"
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
                "xrefs": {
                    "$first": "$xrefs"
                },
                "dependencyRefs": {
                    "$first": "$dependencyRefs"
                },
                "timestamp": {
                    "$max": "$timestamp"
                }
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$addFields": {
                "fismaXref": {
                    "$filter": {
                        "input": "$xrefs",
                        "as": "xref",
                        "cond": {
                            "$eq": [
                                "$$xref.kind",
                                "external::fisma"
                            ]
                        }
                    }
                }
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$addFields": {
                "fismaId": {
                    "$arrayElemAt": [
                        "$fismaXref",
                        0
                    ]
                }
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$project": {
                "_id": 1,
                "id": 1,
                "name": 1,
                "version": 1,
                "purl": 1,
                "created": 1,
                "packageManager": 1,
                "provider": 1,
                "fismaId": "$fismaId.map.id",
                "dependencyRefs": 1
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$unwind": {
                "path": "$dependencyRefs",
                "preserveNullAndEmptyArrays": true
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$lookup": {
                "from": "Package",
                "localField": "dependencyRefs",
                "foreignField": "purl",
                "as": "report"
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$unwind": {
                "path": "$report",
                "preserveNullAndEmptyArrays": true
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$addFields": {
                "report": {
                    "name": "$report.cdx.name",
                    "kind": "$report.kind",
                    "packageManager": "$report.packageManager",
                    "purl": "$report.purl",
                    "version": "$report.version"
                }
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$group": {
                "_id": 0,
                "id": {
                    "$first": "$id"
                },
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
                "fismaId": {
                    "$first": "$fismaId"
                },
                "report": {
                    "$push": "$report"
                }
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$unwind": {
                "path": "$report",
                "preserveNullAndEmptyArrays": true
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$addFields": {
                "report.snyk_enrichment": {
                    "provider": "Snyk"
                }
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$lookup": {
                "from": "Vulnerability",
                "localField": "report.purl",
                "foreignField": "purl",
                "as": "report.snyk_enrichment.results"
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$addFields": {
                "report.snyk_enrichment.results": {
                    "$map": {
                        "input": "$report.snyk_enrichment.results",
                        "in": {
                            "severity": "$$this.severity",
                            "cve": "$$this.cve",
                            "description": "$$this.description",
                            "epssScore": "$$this.epssScore",
                            "cvss": "$$this.cvss",
                            "cwes": "$$this.cwes",
                            "remediation": "$$this.remediation"
                        }
                    }
                }
            }
        })));

        pipeline.add_stage(Stage::new(json!({
            "$group": {
                "_id": 0,
                "id": {
                    "$first": "$id"
                },
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
                "fismaId": {
                    "$first": "$fismaId"
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
        })));

        let json = match pipeline.execute_on("Sbom").await {
            Ok(json) => json,
            Err(err) => {
                return Err(Error::Analytic(format!(
                    "Problem executing analytic: {}",
                    err
                )))
            }
        };

        println!("==> pipeline stages after execute {}", pipeline.len());

        match self
            .storage
            .write(purl.as_str(), json, "detailed-report")
            .await
        {
            Ok(path) => Ok(Some(path)),
            Err(e) => Err(Error::Analytic(format!(
                "export::export_by_purl::write::{}",
                e
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use harbcore::config::*;
    use harbcore::entities::tasks::TaskKind;
    use harbcore::services::analytics::FileSystemStorageProvider;
    use harbcore::Error;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_run() -> Result<(), Error> {
        let storage = Arc::new(FileSystemStorageProvider::new(
            "/tmp/harbor-debug/extensions/export".to_string(),
        ));
        let cx = dev_context(None).map_err(|e| Error::Config(e.to_string()))?;
        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Analytic(e.to_string()))?,
        );

        let service = ExportService::new(store, storage);

        let mut task: Task = Task::new(TaskKind::Extension("export".to_string()))
            .map_err(|e| Error::Analytic(e.to_string()))?;

        let provider = ExportTask::new(service);

        provider
            .execute(&mut task)
            .await
            .map_err(|e| Error::Analytic(e.to_string()))
    }
}
