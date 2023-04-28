use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::entities::cyclonedx::component::ComponentType;
use crate::entities::cyclonedx::{Bom, Component};
use crate::entities::packages::PackageCdx;
use crate::entities::sboms::{Sbom, SbomProviderKind, Spec};
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

    /// The package manager for the [Package].
    pub manager: Option<String>,

    /// Optional denormalized Package URL if the Package is associated with a Purl.
    pub purl: Option<String>,

    /// Encapsulates CycloneDx specific attributes.
    pub cdx: Option<PackageCdx>,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Vec<Xref>,
}

impl Package {
    pub fn from_bom(
        bom: &Bom,
        package_manager: Option<String>,
        xref: Xref,
    ) -> Result<Package, Error> {
        let cdx = PackageCdx::from_bom(bom, package_manager.clone())?;
        let purl = cdx.purl.clone();
        let cdx = Some(cdx);

        Ok(Self {
            id: "".to_string(),
            manager: package_manager,
            purl,
            cdx,
            xrefs: vec![xref],
        })
    }
}
