use std::collections::HashMap;
use std::sync::Arc;

use platform::persistence::mongodb::{Service, Store};

use crate::entities::sboms::{Author, CdxFormat, Sbom, SbomProviderKind};
use crate::entities::tasks::Task;
use crate::entities::xrefs::Xref;
use crate::services::packages::PackageService;
use crate::services::sboms::StorageProvider;
use crate::services::xrefs::XrefService;
use crate::Error;

/// Invoke [sbom-scorecard](https://github.com/eBay/sbom-scorecard) and return the results.
pub fn score(_path: &str) -> Result<String, Error> {
    Ok("not implemented".to_string())
}

/// Compare 2 SBOM scores.
pub fn compare(first_path: &str, second_path: &str) -> Result<String, Error> {
    let first_score = score(first_path)?;
    let second_score = score(second_path)?;

    let mut result = format!("----------------{} score-------------------", first_path);
    result.push_str(first_score.as_str());
    result.push_str(format!("----------------{} score-------------------", second_path).as_str());
    result.push_str(second_score.as_str());

    Ok(result)
}

// Implement Xref Service so that xrefs can be managed for Sboms.
impl XrefService<Sbom> for SbomService {}

/// Provides SBOM related capabilities.
#[derive(Debug)]
pub struct SbomService {
    store: Arc<Store>,
    storage: Box<dyn StorageProvider>,
    packages: PackageService,
}

impl Service<Sbom> for SbomService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl SbomService {
    /// Factory method for creating new instance of type.
    pub fn new(
        store: Arc<Store>,
        storage: Box<dyn StorageProvider>,
        packages: PackageService,
    ) -> Self {
        Self {
            store,
            storage,
            packages,
        }
    }

    /// Processes a raw SBOM and loads it into the Harbor data and file stores.
    pub async fn ingest(
        &self,
        raw: String,
        package_manager: Option<String>,
        provider: SbomProviderKind,
        xref: Xref,
        task: &Task,
    ) -> Result<Sbom, Error> {
        // Load the raw SBOM into the Harbor model.
        let mut sbom = match Sbom::from_raw_cdx(
            raw.as_str(),
            CdxFormat::Json,
            Author::Harbor(provider),
            package_manager,
            xref.clone(),
            task,
        ) {
            Ok(sbom) => sbom,
            Err(e) => {
                let msg = format!("ingest::from_raw_cdx::{}", e);
                println!("{}", msg);
                return Err(Error::Sbom(msg));
            }
        };

        // Determine how many times this Sbom has been synced.
        match self.set_instance_by_purl(&mut sbom).await {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("ingest::set_instance_by_purl::{}", e);
                println!("{}", msg);
                return Err(Error::Sbom(msg));
            }
        }

        // Write the sbom to the storage provider.
        match self
            .write_to_storage(raw.as_bytes().to_vec(), &mut sbom, Some(xref.clone()))
            .await
        {
            Ok(()) => {
                // TODO: Emit Metric.
                println!("ingest::write_to_storage::success");
            }
            Err(e) => {
                // TODO: Emit Metric.
                let msg = format!("ingest::write_to_storage::{}", e);
                println!("{}", msg);
                return Err(Error::Sbom(msg));
            }
        };

        // Ensure the timestamp is set if successful.
        sbom.timestamp = platform::time::timestamp()?;

        // Commit the Sbom.
        self.commit(&mut sbom, xref).await?;

        Ok(sbom)
    }

    /// [Transaction script](https://martinfowler.com/eaaCatalog/transactionScript.html) for saving
    /// Sbom results to data store. If change_set None, indicates Sbom has errors and dependent
    /// entities should not be committed.
    async fn commit(&self, sbom: &mut Sbom, xref: Xref) -> Result<(), Error> {
        let mut errs = HashMap::<String, String>::new();

        // Should always insert Sbom. It should never be a duplicate, but a new instance from task.
        match self.insert(sbom).await {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Sbom(e.to_string()));
            }
        }

        let mut package = match sbom.package.clone() {
            Some(package) => package,
            None => {
                return Err(Error::Sbom("sbom_package_none".to_string()));
            }
        };

        // Upsert the Package for the SBOM target
        match self
            .packages
            .upsert_package_by_purl(&mut package, Some(&xref))
            .await
        {
            Ok(_) => {}
            Err(e) => {
                errs.insert("package".to_string(), e.to_string());
            }
        }

        if !errs.is_empty() {
            let errs = match serde_json::to_string(&errs) {
                Ok(errs) => errs,
                Err(e) => format!("error serializing errs {}", e),
            };
            return Err(Error::Sbom(errs));
        }

        // Upsert packages for each dependency.
        for dependency in package.dependencies.iter_mut() {
            match self
                .packages
                .upsert_package_by_purl(dependency, Some(&xref))
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    errs.insert("package".to_string(), e.to_string());
                }
            }
        }

        if !errs.is_empty() {
            let errs = match serde_json::to_string(&errs) {
                Ok(errs) => errs,
                Err(e) => format!("error serializing errs {}", e),
            };
            return Err(Error::Sbom(errs));
        }

        Ok(())
    }

    /// Stores the SBOM to the configured persistence provider using the Purl as the unique
    /// identifier.
    pub async fn write_to_storage(
        &self,
        raw: Vec<u8>,
        sbom: &mut Sbom,
        xref: Option<Xref>,
    ) -> Result<(), Error> {
        // Persist to some sort of permanent storage.
        self.storage.write(raw, sbom, &xref).await?;

        Ok(())
    }

    /// Sets the forward only instance counter using the Purl as the unique identifier.
    pub async fn set_instance_by_purl(&self, sbom: &mut Sbom) -> Result<(), Error> {
        // TODO: As more enrichment sources are added this may need to constrain by Xref too.
        let existing = self.find_by_purl(&sbom.purl).await?;

        sbom.instance = match existing.iter().max_by_key(|s| s.instance) {
            None => 1,
            Some(most_recent) => most_recent.instance + 1,
        };

        Ok(())
    }

    /// Find an [Sbom] by its Package URL.
    pub async fn find_by_purl(&self, purl: &Option<String>) -> Result<Vec<Sbom>, Error> {
        match purl {
            None => Err(Error::Entity("sbom_purl_none".to_string())),
            Some(purl) => self
                .query(HashMap::from([("purl", purl.as_str())]))
                .await
                .map_err(|e| Error::Entity(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    #[async_std::test]
    async fn can_compare_sboms() -> Result<(), Error> {
        Ok(())
    }
}
