use crate::entities::packages::{Dependency, Package, Purl, SourceKind, Unsupported};
use crate::entities::sboms::{CdxFormat, Sbom, Spec};
use crate::services::packages::PackageProvider;
use crate::services::snyk::adapters::{Organization, Project};
use crate::services::snyk::service::SUPPORTED_SBOM_PROJECT_TYPES;
use crate::services::snyk::SnykService;
use crate::Error;
use platform::mongodb::{Context, MongoDocument};
use std::collections::HashMap;
use tracing::log::debug;

// This whole section is a bridge towards an actual UnitOfWork implementation.
/// Maintains the set of pending changes.
struct ChangeSet {
    /// The set of Packages tracked by the sync.
    pub packages: Vec<Package>,

    /// The set of Dependencies tracked by the sync.
    pub dependencies: Vec<Dependency>,

    /// The set of Purls tracked by the sync.
    pub purls: Vec<Purl>,

    /// The set of Packages that have been identified as unsupported by the sync.
    pub unsupported: Vec<Unsupported>,

    /// The set of SBOMs tracked by the sync.
    pub sboms: Vec<Sbom>,
}

impl ChangeSet {
    /// Factory method to create new instance of type.
    pub fn new() -> Self {
        Self {
            packages: vec![],
            dependencies: vec![],
            purls: vec![],
            unsupported: vec![],
            sboms: vec![],
        }
    }

    /// Add a Package.
    pub fn package(&mut self, package: Package) -> Result<(), Error> {
        // Validate the incoming package and return error for metrics.
        if package.cdx.is_none() {
            return Err(Error::Entity("missing_package_component".to_string()));
        }

        let existing = self
            .packages
            .iter_mut()
            .find(|p| p.cdx.as_ref().unwrap().purl_eq(package.cdx.clone()));

        match existing {
            None => {
                self.packages.push(package.to_owned());
            }
            Some(existing) => match package.snyk_refs {
                None => {}
                Some(snyk_refs) => snyk_refs
                    .iter()
                    .for_each(|snyk_ref| existing.snyk_refs(snyk_ref.clone())),
            },
        };

        Ok(())
    }

    /// Add a CycloneDx Dependency.
    pub(crate) fn dependencies(&mut self, dependency: Dependency) -> Result<(), Error> {
        if dependency.cdx.is_none() {
            return Err(Error::Entity("missing_dependency_component".to_string()));
        }

        let existing = match self
            .dependencies
            .iter_mut()
            .find(|d| d.cdx.as_ref().unwrap().purl_eq(dependency.cdx.clone()))
        {
            None => {
                self.dependencies.push(dependency.to_owned());
            }
            Some(existing) => match depenency.snyk_refs {
                None => {}
                Some(snyk_refs) => snyk_refs
                    .iter()
                    .for_each(|snyk_ref| existing.snyk_refs(snyk_ref.clone())),
            },
        };

        Ok(())
    }

    /// Add a Purl.
    fn purl(&mut self, purl: Purl) {}

    /// Add an unsupported Package.
    pub(crate) fn unsupported(&mut self, unsupported: Unsupported) {
        let existing = self
            .unsupported
            .iter_mut()
            .find(|u| u.name.eq(&unsupported.name));

        match existing {
            None => {
                self.unsupported.push(unsupported.to_owned());
            }
            Some(u) => {
                for xref in unsupported.xref.snyk {
                    u.xref.snyk(xref.clone());
                }
            }
        };
    }

    /// Add an SBOM.
    fn sbom(&mut self, sbom: Sbom) {}
}
