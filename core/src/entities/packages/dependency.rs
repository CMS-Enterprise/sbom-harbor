use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use crate::entities::cyclonedx::Component;
use crate::entities::packages::package::CdxComponent;
use crate::entities::packages::xrefs::SnykXRef;
use crate::entities::packages::{Cdx, SnykXRef};
use crate::entities::sboms::{CdxFormat, Spec};

/// A dependency identified for a Package.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dependency {
    /// The unique identifier for the Package.
    pub id: String,

    /// A unique identifier for the package that this dependency was found in. The format of this
    /// value will vary by Spec. For CycloneDx, this will be the purl of the parent [Package].
    /// This is not the unique id from the data store, as that is not guaranteed to be available
    /// at the time of creation.
    pub package_ref: String,

    /// The spec type of the SBOM from which the Package was created.
    #[skip_serializing_none]
    pub spec: Option<Spec>,

    /// Encapsulates CycloneDx specific attributes.
    #[skip_serializing_none]
    pub cdx: Option<Cdx>,

    /// A map of Snyk API types that are associated with the Dependency.
    #[skip_serializing_none]
    pub(crate) snyk_refs: Option<Vec<SnykXRef>>,
}

impl Dependency {
    pub(crate) fn from_snyk(
        component: &Component,
        package_ref: String,
        package_manager: Option<String>,
        snyk_ref: SnykXRef,
    ) -> Dependency {
        Dependency {
            id: "".to_string(),
            package_ref,
            spec: Some(Spec::Cdx(CdxFormat::Json)),
            cdx: Some(Cdx::from_component(component, package_manager)),
            snyk_refs: Some(vec![snyk_ref]),
        }
    }

    pub fn snyk_refs(&mut self, snyk_ref: SnykXRef) {
        if self.has_snyk_ref(&snyk_ref) {
            return;
        }

        self.snyk_refs.push(snyk_ref);
    }

    fn has_snyk_ref(&self, snyk_ref: &SnykXRef) -> bool {
        match &self.snyk_refs {
            None => false,
            Some(refs) => refs.iter().any(|r| r.eq(snyk_ref)),
        }
    }
}
