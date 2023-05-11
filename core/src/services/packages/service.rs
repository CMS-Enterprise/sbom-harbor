use crate::entities::packages::{Dependency, Package, Purl, Unsupported};
use crate::entities::xrefs::Xrefs;
use crate::Error;
use platform::mongodb::{Service, Store};
use std::collections::HashMap;
use std::sync::Arc;

/// Service that is capable of creating, storing, and managing relationships between one or more
/// types from the [package] module.
#[derive(Debug)]
pub struct PackageService {
    store: Arc<Store>,
}

impl Service<Package> for PackageService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl Service<Dependency> for PackageService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl Service<Purl> for PackageService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl Service<Unsupported> for PackageService {
    fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}

impl PackageService {
    /// Factory method for new instance of type.
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }

    /// Transaction logic for upserting a detected [Package].
    pub(crate) async fn upsert_package_by_purl(&self, new: &mut Package) -> Result<(), Error> {
        let purl = match &new.purl {
            None => {
                return Err(Error::Entity("package_purl_none".to_string()));
            }
            Some(purl) => purl,
        };

        // Is Package in the DB already for this Package?
        let existing: Vec<Package> = self.query(HashMap::from([("purl", purl.as_str())])).await?;

        // If more than one exists, this is a data consistency error.
        if existing.len() > 1 {
            return Err(Error::Entity(format!("duplicate_package_purl::{}", purl)));
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

        // Copy existing xrefs
        for xref in &existing.xrefs {
            new.xrefs(xref);
        }

        // Update db.
        self.update(new)
            .await
            .map_err(|e| Error::Entity(format!("upsert_package_by_purl::update::{}", e)))
    }

    /// Transaction logic for upserting a detected [Dependency].
    pub(crate) async fn upsert_dependency_by_purl(
        &self,
        new: &mut Dependency,
    ) -> Result<(), Error> {
        let purl = match new.purl() {
            None => {
                return Err(Error::Entity("dependency_purl_none".to_string()));
            }
            Some(purl) => purl,
        };

        // Is Dependency in the DB already for this PackageRef and Purl?
        let existing: Vec<Dependency> = self
            .query(HashMap::from([
                ("purl", purl.as_str()),
                ("packageRef", new.package_ref.as_str()),
            ]))
            .await?;

        // If more than one exists, this is a data consistency error.
        if existing.len() > 1 {
            return Err(Error::Entity(format!(
                "duplicate_dependency_purl::{}",
                purl
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

        // Copy existing xrefs
        for xref in &existing.xrefs {
            new.xrefs(xref);
        }

        // Update db.
        self.update(new)
            .await
            .map_err(|e| Error::Entity(format!("dependency_save::{}", e)))
    }

    /// Transaction logic for upserting a detected [Purl].
    pub(crate) async fn upsert_purl(&self, new: &mut Purl) -> Result<(), Error> {
        if let true = new.purl.is_empty() {
            return Err(Error::Entity("dependency_purl_empty".to_string()));
        };

        // Is Purl in the DB already?
        let existing: Vec<Purl> = self
            .query(HashMap::from([("purl", new.purl.as_str())]))
            .await?;

        // If more than one exists, this is a data consistency error.
        if existing.len() > 1 {
            return Err(Error::Entity(format!("duplicate_purl::{}", new.purl)));
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

        // Merge the vulnerabilities if there existing vulnerabilities.
        match &existing.vulnerabilities {
            None => {}
            Some(findings) => {
                new.vulnerabilities(findings);
            }
        }

        // // Copy existing scan_refs.
        for scan_ref in &existing.scan_refs {
            new.scan_refs(scan_ref);
        }

        // Copy existing xrefs.
        for xref in &existing.xrefs {
            new.xrefs(xref);
        }

        // Update db.
        self.update(new)
            .await
            .map_err(|e| Error::Entity(format!("purl_update::{}", e)))
    }

    /// Transaction logic for upserting a detected [Unsupported].
    pub(crate) async fn upsert_unsupported_by_external_id(
        &self,
        new: &mut Unsupported,
    ) -> Result<(), Error> {
        // Is Unsupported in the DB already?
        let existing: Vec<Purl> = self
            .query(HashMap::from([("external_id", new.external_id.as_str())]))
            .await?;

        // If more than one exists, this is a data consistency error.
        if existing.len() > 1 {
            return Err(Error::Entity(format!(
                "duplicate_unsupported::{}",
                new.external_id
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

        // Copy existing xrefs
        for xref in &existing.xrefs {
            new.xrefs(xref);
        }

        // Update db.
        self.update(new)
            .await
            .map_err(|e| Error::Entity(format!("unsupported_update::{}", e)))
    }
}
