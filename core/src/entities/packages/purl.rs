use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::entities::cyclonedx::Component;
use crate::entities::packages::Finding;
use crate::entities::scans::{Scan, ScanRef};
use crate::entities::xrefs::Xref;
use crate::Error;

/// Purl is a derived type that facilitates analysis of a Package across the entire enterprise.
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Purl {
    /// Unique identifier for the Package URL.
    pub id: String,

    /// The package manager for the [Purl].
    pub package_manager: Option<String>,

    /// The raw Package URL.
    pub purl: String,

    /// The package name.
    pub name: String,

    /// The package version.
    pub version: Option<String>,

    /// Source of the Purl.
    pub component_kind: ComponentKind,

    /// Reference to each [Scan] that was performed against this [Purl].
    pub scan_refs: Vec<ScanRef>,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Vec<Xref>,

    /// The list of vulnerability findings associated with this Purl.
    pub findings: Option<Vec<Finding>>,
}

impl Purl {
    pub(crate) fn decode(purl: &str) -> Result<String, Error> {
        let result = platform::encoding::url_decode(purl)
            .map_err(|e| Error::Entity(format!("purl::decode::{}", e)))?;
        Ok(result.to_string())
    }

    /// Generates a path safe file name from a Package URL.
    pub(crate) fn format_file_name(purl: &str) -> String {
        purl.replace("/", "_")
    }

    pub(crate) fn from_component(
        component: &Component,
        component_kind: ComponentKind,
        scan: &Scan,
        iteration: u32,
        xref: Xref,
    ) -> Result<Self, Error> {
        let purl = match &component.purl {
            None => {
                return Err(Error::Entity("component_purl_none".to_string()));
            }
            Some(p) => p,
        };

        let scan_ref = ScanRef::new(scan, purl.clone(), iteration);

        Ok(Self {
            id: "".to_string(),
            package_manager: None,
            purl: purl.clone(),
            name: component.name.clone(),
            version: component.version.clone(),
            component_kind,
            scan_refs: vec![scan_ref],
            findings: None,
            xrefs: vec![xref],
        })
    }

    pub fn scan_refs(&mut self, mut scan: &Scan) -> Result<ScanRef, Error> {
        if scan.id.is_empty() {
            return Err(Error::Entity("scan_id_required".to_string()));
        }

        let mut scan_ref = ScanRef::new(scan, self.purl.clone(), 0);

        scan_ref.iteration = match self.scan_refs.iter().max_by_key(|s| s.iteration) {
            Some(s) => s.iteration + 1,
            _ => 1,
        };

        let result = scan_ref.clone();
        self.scan_refs.push(scan_ref);

        Ok(result)
    }

    pub fn scan_ref_err(&mut self, scan: &Scan, err: Option<String>) -> Result<(), Error> {
        return match self.scan_refs.iter_mut().find(|e| e.scan_id == scan.id) {
            None => Err(Error::Entity("scan_ref_none".to_string())),
            Some(scan_ref) => {
                scan_ref.err = err;
                Ok(())
            }
        };
    }

    pub fn findings(&mut self, findings: Option<Vec<Finding>>) {
        let findings = match findings {
            None => {
                return;
            }
            Some(findings) => findings,
        };

        if findings.is_empty() {
            return;
        }

        match self.findings.clone() {
            None => {
                self.findings = Some(findings);
            }
            Some(mut existing) => {
                existing.extend(findings);
                self.findings = Some(existing);
            }
        }
    }
}

/// Discriminator that indicates whether the Purl was extracted from a [Package] or a [Dependency].
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ComponentKind {
    /// The Purl was extracted from a Package.
    Package,
    /// The Purl was extracted from a Dependency.
    Dependency,
}

impl ToString for ComponentKind {
    fn to_string(&self) -> String {
        match self {
            ComponentKind::Package => "package".to_string(),
            ComponentKind::Dependency => "dependency".to_string(),
        }
    }
}
