use crate::entities::enrichments::Vulnerability;
use crate::entities::packages::Package;
use crate::services::vulnerabilities::StorageProvider;
use crate::Error;
use platform::persistence::mongodb::{Service, Store};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

/// Provides [Vulnerability] related data management capabilities.
#[derive(Debug)]
pub struct VulnerabilityService {
    store: Arc<Store>,
    pub(crate) storage: Option<Box<dyn StorageProvider>>,
}

impl Service<Vulnerability> for VulnerabilityService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl VulnerabilityService {
    /// Factory method to create new instances of type.
    pub fn new(store: Arc<Store>, storage: Option<Box<dyn StorageProvider>>) -> Self {
        Self { store, storage }
    }

    /// Queries the data store for vulnerabilities related to a Purl.
    pub async fn find_by_purl(&self, purl: &str) -> Result<Vec<Vulnerability>, Error> {
        let result = self.query(HashMap::from([("purl", purl)])).await?;
        Ok(result)
    }

    /// Stores the set of [Vulnerability] instances for a [Package] using the configured
    /// [StorageProvider].
    pub async fn store_by_purl(&self, package: &Package) -> Result<Option<String>, Error> {
        if package.vulnerabilities.is_empty() {
            return Ok(None);
        }

        let purl = match &package.purl {
            None => {
                return Err(Error::Vulnerability("package_purl_none".to_string()));
            }
            Some(purl) => purl.as_str(),
        };

        let storage = match &self.storage {
            None => {
                return Err(Error::Config("storage_provider_none".to_string()));
            }
            Some(storage) => storage,
        };

        match storage
            .write(purl, package.vulnerabilities.as_slice(), &package.xrefs)
            .await
        {
            Ok(file_path) => Ok(Some(file_path)),
            Err(e) => Err(Error::Enrichment(format!(
                "vulnerability::store_by_purl::write::{}",
                e
            ))),
        }
    }

    /// Transaction logic for upserting a detected [Vulnerability] using the Package URL as the
    /// unique identifier.
    pub async fn upsert_by_purl(&self, new: &mut Vulnerability) -> Result<(), Error> {
        if let true = new.purl.is_empty() {
            return Err(Error::Entity("vulnerability_purl_empty".to_string()));
        };

        // Is Purl in the DB already?
        let existing: Vec<Vulnerability> = self
            .query(HashMap::from([("purl", new.purl.as_str())]))
            .await?;

        // If more than one exists, this is a data consistency error.
        if existing.len() > 1 {
            return Err(Error::Entity(format!(
                "duplicate_vulnerability::{}",
                new.purl
            )));
        }

        // If none exists, insert and return.
        let existing = match existing.first() {
            None => {
                self.insert(new).await?;
                return Ok(());
            }
            Some(existing) => existing,
        };

        // If one exists, continue.

        // Replace previously saved instance with newly parsed instance by setting the new
        // instance id to existing instance id.
        new.id = existing.id.clone();

        // Copy existing task_refs.
        for task_ref in &existing.task_refs {
            new.task_refs(task_ref);
        }

        // Update db.
        self.update(new)
            .await
            .map_err(|e| Error::Entity(format!("upsert_vulnerability_by_purl::update::{}", e)))
    }
}
