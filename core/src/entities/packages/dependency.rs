use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::entities::cyclonedx::Component;
use crate::entities::packages::PackageCdx;
use crate::entities::sboms::{CdxFormat, SbomProviderKind, Spec};
use crate::entities::xrefs::Xref;

/// A dependency identified for a Package.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Dependency {
    /// The unique identifier for the Package.
    pub id: String,

    /// The provider that generated the SBOM that the [Dependency] was extracted from.
    pub provider: SbomProviderKind,

    /// A unique identifier for the package that this dependency was found in. The format of this
    /// value will vary by Spec. For CycloneDx, this will be the purl of the parent [Package].
    /// This is not the unique id from the data store, as that is not guaranteed to be available
    /// at the time of creation.
    pub package_ref: String,

    /// The spec type of the SBOM from which the Package was created.
    pub spec: Option<Spec>,

    /// Encapsulates CycloneDx specific attributes.
    pub cdx: Option<PackageCdx>,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Vec<Xref>,

    /// CycloneDx JSON spec model of Sbom. Hydrated at runtime.
    #[serde(skip)]
    pub(crate) component: Option<Component>,
}

impl Dependency {
    pub(crate) fn from_component(
        component: &Component,
        provider: SbomProviderKind,
        package_ref: String,
        package_manager: Option<String>,
        xref: Xref,
    ) -> Dependency {
        let cdx = PackageCdx::from_component(component, package_manager);

        Dependency {
            id: "".to_string(),
            provider,
            package_ref,
            spec: Some(Spec::Cdx(CdxFormat::Json)),
            cdx: Some(cdx),
            component: Some(component.clone()),
            xrefs: vec![xref],
        }
    }

    /// Utility method for extracting the purl from the CycloneDx Component.
    pub fn purl(&self) -> Option<String> {
        match &self.cdx {
            None => None,
            Some(cdx) => cdx.purl.clone(),
        }
    }
}
