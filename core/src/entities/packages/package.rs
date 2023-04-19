use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::entities::cyclonedx::component::ComponentType;
use crate::entities::cyclonedx::{Bom, Component};
use crate::entities::packages::PackageCdx;
use crate::entities::sboms::{SbomProviderKind, Spec};
use crate::entities::xrefs::{Xref, XrefKind};
use crate::Error;

/// A [Package] is a item for which an SBOM can be generated. It serves as an aggregate root for all
/// version of an SBOM, and as a way of cross-referencing SBOMs across disparate systems.
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Package {
    /// The unique identifier for the Package.
    pub id: String,

    /// The provider that generated the SBOM that the [Package] was extracted from.
    pub provider: SbomProviderKind,

    /// The spec type of the SBOM from which the Package was created.
    pub spec: Option<Spec>,

    /// Encapsulates CycloneDx specific attributes.
    pub cdx: Option<PackageCdx>,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Option<HashMap<XrefKind, Xref>>,
}

impl Default for Package {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            provider: SbomProviderKind::GitHub,
            spec: None,
            cdx: None,
            xrefs: None,
        }
    }
}

impl Package {
    pub fn from_bom(
        bom: &Bom,
        source: SbomProviderKind,
        spec: Option<Spec>,
        package_manager: Option<String>,
        xref_kind: XrefKind,
        xrefs: Option<Xref>,
    ) -> Result<Package, Error> {
        let cdx = Some(PackageCdx::from_bom(bom, package_manager)?);
        let xrefs = match xrefs {
            None => None,
            Some(xrefs) => Some(HashMap::from([(xref_kind, xrefs)])),
        };

        Ok(Self {
            id: "".to_string(),
            provider: source,
            spec,
            cdx,
            xrefs,
        })
    }
}
