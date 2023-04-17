use crate::entities::cyclonedx::Component;
use crate::entities::packages::PackageCdx;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::entities::sboms::{CdxFormat, SbomSource, Spec};
use crate::entities::xrefs::{Xref, XrefKind};
use crate::Error;

/// An [Unsupported] is a item for which an SBOM cannot be generated using the current enrichment
/// provider.
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Unsupported {
    /// The unique identifier for the Package.
    pub id: String,

    ///
    pub package_manager: Option<String>,

    /// The system that generated the SBOM that the [Unsupported] was identified in.
    pub source: SbomSource,

    /// The spec type of the SBOM from which the Package was created.
    pub spec: Option<Spec>,

    /// Encapsulates CycloneDx specific attributes.
    pub cdx: Option<PackageCdx>,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Option<HashMap<XrefKind, Xref>>,
}

impl Unsupported {
    pub(crate) fn from_component(
        component: &Component,
        source: SbomSource,
        package_manager: Option<String>,
        xref_kind: XrefKind,
        xrefs: Option<Xref>,
    ) -> Unsupported {
        let cdx = Some(PackageCdx::from_component(
            component,
            package_manager.clone(),
        ));
        let xrefs = match xrefs {
            None => None,
            Some(xrefs) => Some(HashMap::from([(xref_kind, xrefs)])),
        };

        Unsupported {
            id: "".to_string(),
            package_manager,
            source,
            spec: Some(Spec::Cdx(CdxFormat::Json)),
            cdx,
            xrefs,
        }
    }
}
