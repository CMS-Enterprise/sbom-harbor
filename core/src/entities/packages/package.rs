use serde::{Deserialize, Serialize};

use crate::entities::cyclonedx::component::ComponentType;
use crate::entities::cyclonedx::{Bom, Component};
use crate::entities::packages::xrefs::SnykXRef;
use crate::entities::packages::{Cdx, SnykXRef};
use crate::entities::sboms::Spec;
use crate::models::sbom::Spec;
use crate::services::cyclonedx::models::component::ComponentType;
use crate::services::cyclonedx::models::{Bom, Component};
use crate::services::cyclonedx::{bom_component, bom_purl};
use crate::Error;

/// A [Package] is a item for which an SBOM can be generated. It serves as an aggregate root for all
/// version of an SBOM, and as a way of cross-referencing SBOMs across disparate systems.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Package {
    /// The unique identifier for the Package.
    pub id: String,

    /// The spec type of the SBOM from which the Package was created.
    #[skip_serializing_none]
    pub spec: Option<Spec>,

    /// Encapsulates CycloneDx specific attributes.
    #[skip_serializing_none]
    pub cdx: Option<Cdx>,

    /// A map of Snyk API types that are associated with the Package.
    #[skip_serializing_none]
    pub(crate) snyk_refs: Option<Vec<SnykXRef>>,
}

impl Package {
    pub fn from_bom(
        bom: &Bom,
        spec: Option<Spec>,
        package_manager: Option<String>,
        snyk_ref: Option<SnykXRef>,
    ) -> Result<Package, Error> {
        Ok(Self {
            id: "".to_string(),
            spec,
            cdx: Some(Cdx::from_bom(bom, package_manager)?),
            snyk_refs: match snyk_ref {
                None => None,
                Some(snyk_ref) => Some(vec![snyk_ref]),
            },
        })
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
