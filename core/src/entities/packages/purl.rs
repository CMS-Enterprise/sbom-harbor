use serde::{Deserialize, Serialize};
use tracing::debug;
use urlencoding::decode;

use crate::entities::cyclonedx::Component;
use crate::entities::packages::finding::Finding;
use crate::entities::packages::xrefs::SnykXRef;
use crate::entities::packages::SnykXRef;
use crate::Error;

/// Purl is a derived type that facilitates analysis of a Package across the entire enterprise.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Purl {
    /// Unique identifier for the Package URL.
    pub id: String,

    /// The raw Package URL.
    pub purl: String,

    /// The package name.
    pub name: String,

    /// The package version.
    #[skip_serializing_none]
    pub version: Option<String>,

    /// Source of the Purl.
    pub source: SourceKind,

    /// The list of vulnerability findings associated with this Purl.
    #[skip_serializing_none]
    pub findings: Option<Vec<Finding>>,

    /// A map of projects that are associated with the Purl.
    #[skip_serializing_none]
    pub snyk_refs: Option<Vec<SnykXRef>>,
}

impl Purl {
    pub(crate) fn decode(purl: &str) -> Result<String, Error> {
        let result = decode(purl)?;
        Ok(result.to_string())
    }

    pub(crate) fn from_snyk(
        component: &Component,
        source: SourceKind,
        snyk_ref: SnykXRef,
    ) -> Result<Option<Self>, Error> {
        let purl = match &component.purl {
            None => {
                return Ok(None);
            }
            Some(p) => p,
        };

        let purl = match Self::decode(purl.as_str()) {
            Ok(p) => p,
            Err(e) => {
                return Err(Error::Entity(format!("purl::from_component::{}", e)));
            }
        };

        Ok(Some(Self {
            id: "".to_string(),
            purl,
            name: component.name.clone(),
            version: component.version.clone(),
            source,
            findings: None,
            snyk_refs: Some(vec![snyk_ref]),
        }))
    }

    pub fn finding(&mut self, finding: Finding) {
        match &self.findings {
            None => self.findings = Some(vec![finding]),
            Some(existing) => {
                let exists = existing.iter().any(|f| f.eq(&finding));

                if !exists {
                    self.findings.push(finding);
                }
            }
        }
    }

    pub fn snyk_ref(&mut self, snyk_ref: SnykXRef) {
        match &self.snyk_refs {
            None => self.snyk_refs = Some(vec![snyk_ref]),
            Some(existing) => {
                let exists = existing.iter().any(|r| r.eq(&snyk_ref));

                if !exists {
                    self.snyk_refs.push(snyk_ref);
                }
            }
        }
    }
}

/// Indicates the type from which the Purl was extracted.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SourceKind {
    /// The Purl was extracted from the Component Metadata of a Package.
    Package,
    /// The Purl was extracted from a Dependency of a Package.
    Dependency,
}

impl ToString for SourceKind {
    fn to_string(&self) -> String {
        match self {
            SourceKind::Package => "package".to_string(),
            SourceKind::Dependency => "dependency".to_string(),
        }
    }
}

impl Purl {
    pub(crate) fn merge_snyk_refs(&mut self, refs: Vec<SnykXRef>) {
        let existing_refs = match &self.snyk_refs {
            None => {
                self.snyk_refs = Some(refs);
                return;
            }
            Some(r) => r,
        };

        for r in refs {
            if existing_refs.iter().any(|existing| r.eq(&existing)) {
                continue;
            }

            self.snyk_refs.push(r);
        }
    }
}
