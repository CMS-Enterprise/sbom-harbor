use crate::entities::cyclonedx::Bom;
use crate::entities::packages::PackageCdx;
use crate::entities::xrefs::Xref;
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A [Package] is a item for which an SBOM can be generated. It serves as an aggregate root for all
/// version of an SBOM, and as a way of cross-referencing SBOMs across disparate systems.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
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
    /// Factory method for creating a [Package] entity by analyzing a CycloneDx [Bom] instance.
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
