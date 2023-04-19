use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use tracing::debug;

use crate::entities::cyclonedx::Component;
use crate::entities::sboms::Finding;
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

        Ok(Self {
            id: "".to_string(),
            purl,
            name: component.name.clone(),
            version: component.version.clone(),
            component_kind,
            findings: None,
            xrefs,
        })
    }

    pub fn finding(&mut self, finding: Finding) {
        let mut existing = match self.findings.clone() {
            None => {
                self.findings = Some(vec![finding]);
                return;
            }
            Some(existing) => existing,
        };

        let exists = existing.iter().any(|f| f.eq(&finding));
        if !exists {
            existing.push(finding);
            self.findings = Some(existing);
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
