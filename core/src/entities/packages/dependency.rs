use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::entities::cyclonedx::Component;
use crate::entities::packages::PackageCdx;
use crate::entities::sboms::{CdxFormat, SbomSource, Spec};
use crate::entities::xrefs::{Xref, XrefKind};
use crate::Error;

/// A dependency identified for a Package.
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dependency {
    /// The unique identifier for the Package.
    pub id: String,

    /// The system that generated the SBOM that the [Dependency] was extracted from.
    pub source: SbomSource,

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
    pub xrefs: Option<HashMap<XrefKind, Xref>>,
}

impl Dependency {
    pub(crate) fn from_component(
        component: &Component,
        source: SbomSource,
        package_ref: String,
        package_manager: Option<String>,
        xref_kind: XrefKind,
        xrefs: Option<Xref>,
    ) -> Dependency {
        let cdx = PackageCdx::from_component(component, package_manager);

        let xrefs = match xrefs {
            None => None,
            Some(xrefs) => Some(HashMap::from([(xref_kind, xrefs)])),
        };

        Dependency {
            id: "".to_string(),
            source,
            package_ref,
            spec: Some(Spec::Cdx(CdxFormat::Json)),
            cdx: Some(cdx),
            xrefs,
        }
    }
}
