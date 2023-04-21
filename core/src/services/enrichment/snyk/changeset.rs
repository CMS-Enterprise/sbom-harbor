use crate::entities::cyclonedx::Bom;
use crate::entities::enrichment::{Scan, ScanRef};
use crate::entities::packages::{
    ComponentKind, Dependency, Finding, Package, PackageCdx, Purl, Unsupported,
};
use crate::entities::sboms::{CdxFormat, Sbom, SbomProviderKind, Source, Spec};
use crate::entities::xrefs::{Xref, XrefKind, Xrefs};
use crate::services::packages::PackageProvider;
use crate::services::snyk::adapters::{Organization, Project};
use crate::services::snyk::{SnykRef, SnykService, API_VERSION, SNYK_DISCRIMINATOR};
use crate::Error;
use platform::mongodb::{Context, MongoDocument};
use std::collections::HashMap;
use std::default::Default;
use std::ops::Deref;
use tracing::log::debug;

// This whole section is a bridge towards an actual UnitOfWork implementation.
/// Maintains the set of pending changes. Leverages the raw Package URL as the unique key for all
/// HashMaps.
pub(crate) struct ScanSbomsChangeSet<'a> {
    /// The [Scan] instance used to relate all changes in the changeset.
    pub scan: &'a mut Scan,

    /// The set of SBOMs tracked by the sync.
    pub sboms: HashMap<String, Sbom>,

    /// The set of Purls tracked by the sync.
    pub purls: HashMap<String, Purl>,

    /// The set of Packages tracked by the sync.
    pub packages: HashMap<String, Package>,

    /// The set of Dependencies tracked by the sync.
    pub dependencies: HashMap<String, Dependency>,

    /// The set of Packages that have been identified as unsupported by the sync.
    pub unsupported: HashMap<String, Unsupported>,
}

struct ScanSbomsChange {
    /// The Package proposed for inclusion in the ChangeSet.
    pub package: Package,

    /// Pre-validated Purl for the primary package.
    pub package_purl: Purl,

    /// The set of Purls proposed for inclusion in the ChangeSet.
    pub purls: HashMap<String, Purl>,

    /// The set of Dependencies proposed for inclusion in the ChangeSet.
    pub dependencies: HashMap<String, Dependency>,

    /// The set of Unsupported proposed for inclusion in the ChangeSet.
    pub unsupported: HashMap<String, Unsupported>,
}

impl ScanSbomsChangeSet<'_> {
    /// Factory method to create new instance of type.
    pub fn new(scan: &'_ mut Scan) -> ScanSbomsChangeSet<'_> {
        ScanSbomsChangeSet {
            scan,
            packages: HashMap::new(),
            dependencies: HashMap::new(),
            purls: HashMap::new(),
            unsupported: HashMap::new(),
            sboms: HashMap::new(),
        }
    }
    pub(in crate::services::enrichment::snyk) fn ref_errs(
        &mut self,
        target_id: String,
        err: String,
    ) {
        self.scan.ref_errs(target_id, err);
    }

    /// Track an SBOM and all it's parts.
    pub(in crate::services::enrichment::snyk) fn track(
        &mut self,
        sbom: &mut Sbom,
        package_manager: String,
        snyk_ref: SnykRef,
    ) -> Result<(), Error> {
        match sbom.scan_refs(&self.scan, None) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::Enrichment(format!("changeset::track::{}", e)));
            }
        }

        // These should be safe by this point, but adding expect to detect upstream errors.
        let bom = sbom.bom.clone().expect("change_set::sbom::bom_none");
        let purl = sbom.purl.clone().expect("change_set::sbom::purl_none");
        let xref_kind = XrefKind::External(SNYK_DISCRIMINATOR.to_string());
        let xrefs = HashMap::from(snyk_ref);

        let mut change = ScanSbomsChange {
            package: Package::default(),
            package_purl: Purl::default(),
            purls: HashMap::new(),
            dependencies: HashMap::new(),
            unsupported: HashMap::new(),
        };

        change.package = self.resolve_package(&package_manager, &bom, &xref_kind, &xrefs)?;
        change.package_purl = self.resolve_package_purl(&bom, &xref_kind, &xrefs)?;
        (change.purls, change.dependencies) =
            self.resolve_dependencies(&bom, &purl, &package_manager, &xref_kind, xrefs)?;

        // Once all changes have been validated, add them to the ChangeSet.
        // TODO: Stop all these clones.
        self.sboms.insert(purl.clone(), sbom.clone());
        self.package(purl.clone(), change.package)?;
        self.purl(change.package_purl)?;

        for (_, val) in change.purls.into_iter() {
            self.purl(val)?;
        }

        for (key, val) in change.dependencies.into_iter() {
            self.dependencies(key, val)?;
        }

        Ok(())
    }

    fn resolve_package(
        &self,
        package_manager: &String,
        bom: &Bom,
        xref_kind: &XrefKind,
        xrefs: &Xref,
    ) -> Result<Package, Error> {
        Package::from_bom(
            &bom,
            SbomProviderKind::Snyk {
                api_version: crate::services::snyk::API_VERSION.to_string(),
            },
            Some(Spec::Cdx(CdxFormat::Json)),
            Some(package_manager.clone()),
            xref_kind.clone(),
            Some(xrefs.clone()),
        )
    }

    // Extract the Purl for the primary Package from the Bom.
    fn resolve_package_purl(
        &self,
        bom: &Bom,
        xref_kind: &XrefKind,
        xrefs: &Xref,
    ) -> Result<Purl, Error> {
        let component = match bom.component() {
            None => {
                let msg = "bom_component_none";
                debug!("{}", msg);
                return Err(Error::Snyk(msg.to_string()));
            }
            Some(component) => component,
        };

        let purl = match Purl::from_component(
            &component,
            ComponentKind::Package,
            &self.scan,
            xref_kind.clone(),
            Some(xrefs.clone()),
        ) {
            Ok(purl) => purl,
            Err(e) => {
                return Err(Error::Entity(e.to_string()));
            }
        };

        Ok(purl)
    }

    fn resolve_dependencies(
        &mut self,
        bom: &Bom,
        package_ref: &String,
        package_manager: &String,
        xref_kind: &XrefKind,
        xrefs: Xref,
    ) -> Result<(HashMap<String, Purl>, HashMap<String, Dependency>), Error> {
        let mut purls = HashMap::new();
        let mut dependencies = HashMap::new();

        let components = match &bom.components {
            None => {
                return Ok((purls, dependencies));
            }
            Some(components) => components,
        };

        for component in components {
            let purl = Purl::from_component(
                &component,
                ComponentKind::Dependency,
                &self.scan,
                xref_kind.clone(),
                Some(xrefs.clone()),
            )?;

            let dependency_purl = purl.purl.clone();
            self.purl(purl)?;

            dependencies.insert(
                dependency_purl,
                Dependency::from_component(
                    &component,
                    SbomProviderKind::Snyk {
                        api_version: API_VERSION.to_string(),
                    },
                    package_ref.clone(),
                    Some(package_manager.clone()),
                    xref_kind.clone(),
                    Some(xrefs.clone()),
                ),
            );
        }

        Ok((purls, dependencies))
    }

    /// Track a Package.
    pub fn package(&mut self, purl: String, package: Package) -> Result<(), Error> {
        match self.packages.get_mut(purl.as_str()) {
            None => {
                self.packages.insert(purl, package.to_owned());
            }
            Some(existing) => {
                existing.xrefs(package.xrefs);
            }
        };

        Ok(())
    }

    /// Track a Dependency.
    pub(crate) fn dependencies(
        &mut self,
        purl: String,
        dependency: Dependency,
    ) -> Result<(), Error> {
        match self.dependencies.get_mut(purl.as_str()) {
            None => {
                self.dependencies.insert(purl, dependency.to_owned());
            }
            Some(existing) => {
                existing.xrefs(dependency.xrefs);
            }
        };

        Ok(())
    }

    /// Track a Purl.
    fn purl(&mut self, purl: Purl) -> Result<(), Error> {
        match self.purls.get_mut(purl.purl.as_str()) {
            None => {
                self.purls.insert(purl.purl.clone(), purl.to_owned());
            }
            Some(existing) => {
                existing.xrefs(purl.xrefs);
            }
        };

        Ok(())
    }

    /// Track an unsupported Package.
    pub(crate) fn unsupported(&mut self, unsupported: Unsupported) -> Result<(), Error> {
        // Validate the incoming unsupported and return error for metrics.
        let external_id = match unsupported.external_id.clone() {
            None => {
                return Err(Error::Entity("missing_unsupported_external_id".to_string()));
            }
            Some(external_id) => external_id,
        };

        match self.unsupported.get_mut(external_id.as_str()) {
            None => {
                self.unsupported.insert(external_id, unsupported.to_owned());
            }
            Some(existing) => {
                existing.xrefs(unsupported.xrefs);
            }
        };

        Ok(())
    }
}

pub(crate) struct ScanFindingsChangeSet<'a> {
    /// The [Scan] instance used to relate all changes in the changeset.
    pub scan: &'a mut Scan,
}

impl ScanFindingsChangeSet<'_> {
    pub(in crate::services::enrichment::snyk) fn new(scan: &mut Scan) -> ScanFindingsChangeSet {
        ScanFindingsChangeSet { scan }
    }

    /// Track a Findings Scan.
    pub(in crate::services::enrichment::snyk) fn track(
        &mut self,
        purl: &mut Purl,
    ) -> Result<ScanRef, Error> {
        match purl.scan_refs(self.scan, None) {
            Ok(scan_ref) => Ok(scan_ref),
            Err(e) => {
                let msg = format!("changeset::error::purl::scan_refs::{}", e);
                return Err(Error::Enrichment(msg));
            }
        }
    }

    pub(in crate::services::enrichment::snyk) fn error(&mut self, purl: &mut Purl, error: String) {
        self.scan.ref_errs(purl.purl.clone(), error.clone());

        match purl.scan_ref_err(self.scan, Some(error)) {
            Ok(_) => {}
            Err(e) => {
                debug!("changeset::error::purl::scan_refs::{}", e);
            }
        }
    }
}
