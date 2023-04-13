use std::fmt::{Display, Formatter};

use crate::entities::cyclonedx::Component;
use crate::entities::packages::package::CdxComponent;
use crate::entities::packages::xrefs::SnykXRef;
use crate::entities::packages::{Cdx, SnykXRef};
use crate::entities::sboms::{CdxFormat, Spec};
use serde::{Deserialize, Serialize};

/// A dependency identified for a Package.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dependency {
    /// The unique identifier for the Package.
    pub id: String,

    /// The spec type of the SBOM from which the Package was created.
    #[skip_serializing_none]
    pub spec: Option<Spec>,

    /// Encapsulates CycloneDx attributes.
    #[skip_serializing_none]
    pub cdx: Option<Cdx>,

    /// A map of Snyk API types that are associated with the Dependency.
    #[skip_serializing_none]
    pub(crate) snyk_ref: Option<SnykXRef>,
}

impl Dependency {
    fn from_snyk(
        component: Component,
        purl: Option<String>,
        package_manager: Option<String>,
        snyk_ref: Option<SnykXRef>,
    ) -> Dependency {
        Dependency {
            id: "".to_string(),
            spec: Some(Spec::Cdx(CdxFormat::Json)),
            cdx: Some(Cdx::from_bom(component)),
            snyk_ref,
        }
    }
}
