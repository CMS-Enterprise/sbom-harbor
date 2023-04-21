use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use tracing::debug;

use crate::entities::cyclonedx::Component;
use crate::entities::enrichment::{Scan, ScanRef};
use crate::entities::packages::Finding;
use crate::entities::xrefs::{Xref, XrefKind};
use crate::Error;

/// Purl is a derived type that facilitates analysis of a Package across the entire enterprise.
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Purl {
    /// Unique identifier for the Package URL.
    pub id: String,

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

    /// The list of vulnerability findings associated with this Purl.
    pub findings: Option<Vec<Finding>>,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Option<HashMap<XrefKind, Xref>>,
}

impl Default for Purl {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            purl: "".to_string(),
            name: "".to_string(),
            version: None,
            component_kind: ComponentKind::Package,
            scan_refs: vec![],
            findings: None,
            xrefs: None,
        }
    }
}

impl Purl {
    pub(crate) fn decode(purl: &str) -> Result<String, Error> {
        let result = platform::encoding::url_decode(purl)
            .map_err(|e| Error::Entity(format!("purl::decode::{}", e)))?;
        Ok(result.to_string())
    }

    pub(crate) fn from_component(
        component: &Component,
        component_kind: ComponentKind,
        scan: &Scan,
        xref_kind: XrefKind,
        xrefs: Option<Xref>,
    ) -> Result<Self, Error> {
        let purl = match &component.purl {
            None => {
                return Err(Error::Entity("component_purl_none".to_string()));
            }
            Some(p) => p,
        };

        let purl = match Self::decode(purl.as_str()) {
            Ok(p) => p,
            Err(e) => {
                return Err(Error::Entity(format!("purl::from_component::{}", e)));
            }
        };

        let xrefs = match xrefs {
            None => None,
            Some(xrefs) => Some(HashMap::from([(xref_kind, xrefs)])),
        };

        let scan_ref = ScanRef::new(scan, Some(purl.clone()));

        Ok(Self {
            id: "".to_string(),
            purl,
            name: component.name.clone(),
            version: component.version.clone(),
            component_kind,
            scan_refs: vec![scan_ref],
            findings: None,
            xrefs,
        })
    }

    pub fn scan_refs(&mut self, mut scan: &Scan, err: Option<String>) -> Result<(), Error> {
        if scan.id.is_empty() {
            return Err(Error::Entity("scan_id_required".to_string()));
        }

        let mut scan_ref = ScanRef {
            id: "".to_string(),
            scan_id: scan.id.clone(),
            purl: Some(self.purl.clone()),
            iteration: 1,
            err,
        };

        scan_ref.iteration = match self.scan_refs.iter().max_by_key(|s| s.iteration) {
            Some(s) => s.iteration + 1,
            _ => 1,
        };

        self.scan_refs.push(scan_ref);

        Ok(())
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
