use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;

use crate::entities::datasets::{CpesContainer, Purl2Cpes, PurlsContainer};
use platform::persistence::mongodb::{Service, Store};

use crate::entities::tasks::Task;
use crate::services::purl2cpe::service::Purl2CpeService;
use crate::tasks::TaskProvider;
use crate::Error;

use serde_yaml::from_reader;

/// Synchronizes SBOMS for a GitHub Group with Harbor.
#[derive(Debug)]
pub struct ConstructionTask {
    pub(in crate::tasks::construction::dataset) service: Purl2CpeService,
}

impl ConstructionTask {
    /// Creates a new AnalyticExecutionTask
    pub fn new(service: Purl2CpeService) -> Self {
        Self { service }
    }
}

impl Service<Task> for ConstructionTask {
    fn store(&self) -> Arc<Store> {
        self.service.store()
    }
}

#[async_trait]
impl TaskProvider for ConstructionTask {
    async fn run(&self, task: &mut Task) -> Result<HashMap<String, String>, Error> {
        println!("==> Running ConstructionTask Provider to build purl2cpe dataset");

        // Start by dropping the collection. If we are unable
        // to do that, then we need to stop now.
        self.service.store().drop_collection::<Purl2Cpes>().await?;

        println!("==> Purl2Cpes collection dropped, rebuilding...");

        // Create an Errors map to return at the end of the task
        let mut errors = HashMap::<String, String>::new();

        // Save the directory we start in to change back at the end
        let orig_dir = env::current_dir().map_err(Error::Io)?;

        // Get the clone path for the repo we're getting the data from:
        // https://github.com/scanoss/purl2cpe
        let clone_path = self.service.clone_purl2cpe_repo()?;
        let file_path = Path::new(&clone_path);

        // Move to the clone's top level directory.  If we can't
        // do that, then there is something wrong with the filesystem
        // and we cannot go on.
        env::set_current_dir(file_path).map_err(Error::Io)?;

        // Find all of the purls.yaml files.
        let purl_file_names = self
            .service
            .find_purl_yaml_files()
            .map_err(Error::Purl2Cpe)?;

        // Update task to prepare for the data set creation
        let total = purl_file_names.len();
        println!("==> found {} files with a purl for lookup.", total);
        task.count = total as u64;
        self.store().update(task).await
            .map_err(|err| Error::Task(err.to_string()))?;

        for purl_file_name in purl_file_names {
            // The cpes.yaml files are in the same directory as the purls.
            // So, all we should have to do is change the name
            let cpe_file_name = purl_file_name.replace("purls", "cpes");

            // Should the purls.yml file somehow not be there, then we need to log the
            // error and go to the next purls.yml file
            let purl_yaml = match File::open(purl_file_name.clone()).map_err(Error::Io) {
                Ok(yaml) => yaml,
                Err(err) => {
                    task.err_total += 1;
                    errors.insert(purl_file_name.clone(), err.to_string());
                    continue;
                }
            };

            // Because we just changed the filename, it's more likely this file
            // might not actually be there.  In that case, we have nothing to do this
            // iteration and we log the error and go on.
            let cpe_yaml = match File::open(cpe_file_name.clone()).map_err(Error::Io) {
                Ok(yaml) => yaml,
                Err(err) => {
                    task.err_total += 1;
                    errors.insert(cpe_file_name.clone(), err.to_string());
                    continue;
                }
            };

            // Deserialize the purls
            let purls_cont: PurlsContainer =
                from_reader::<File, PurlsContainer>(purl_yaml).map_err(Error::SerdeYaml)?;

            // Deserialize the cpes
            let cpes_cont: CpesContainer =
                from_reader::<File, CpesContainer>(cpe_yaml).map_err(Error::SerdeYaml)?;

            for purl in purls_cont.purls {
                let mut purl2cpes: Purl2Cpes = Purl2Cpes::new(purl.clone(), cpes_cont.cpes.clone());

                match self.service.insert(&mut purl2cpes).await {
                    Err(err) => {
                        task.err_total += 1;
                        task.ref_errs(purl, err.to_string());
                        errors.insert(purl2cpes.id.clone(), err.to_string());
                    }
                    Ok(()) => {
                        println!("==> purl inserted: {}", purl);
                    }
                }
            }
        }

        // Move back to original directory before removing the clone
        env::set_current_dir(orig_dir).map_err(Error::Io)?;
        self.service
            .remove_purl2cpe_clone()
            .map_err(Error::Purl2Cpe)?;

        Ok(errors)
    }
}
