use crate::entities::cyclonedx::component::ComponentType;
use crate::entities::cyclonedx::{Bom, Component};
use crate::entities::packages::xrefs::SnykXRef;
use crate::entities::packages::SnykXRef;
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

    /// The denormalized raw Package URL from the CycloneDxComponent.
    #[skip_serializing_none]
    pub purl: Option<String>,

    /// The package manager of the package.
    #[skip_serializing_none]
    pub package_manager: Option<String>,

    /// Component for CycloneDx Packages.
    #[skip_serializing_none]
    pub cdx_component: Option<CdxComponent>,

    /// A map of Snyk API types that are associated with the Package.
    #[skip_serializing_none]
    pub(crate) snyk_ref: Option<SnykXRef>,
}

impl Package {
    pub fn from_bom(
        bom: &Bom,
        spec: Option<Spec>,
        package_manager: Option<String>,
        snyk_ref: Option<SnykXRef>,
    ) -> Package {
        Self {
            id: "".to_string(),
            purl: bom_purl(bom),
            spec,
            package_manager,
            cdx_component: CdxComponent::from_bom(bom),
            snyk_ref,
        }
    }

    pub fn snyk_refs(&mut self, snyk_ref: SnykXRef) {
        if self.has_snyk_ref(&snyk_ref) {
            return;
        }

        self.snyk_refs.push(snyk_ref);
    }

    fn has_snyk_ref(&self, snyk_ref: &SnykXRef) -> bool {
        self.snyk_refs
            .iter()
            .any(|r| r.is_some() && r.unwrap().eq(snyk_ref))
    }
}

/// A subset of the full CycloneDx Component suitable for tracking packages within the registry.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cdx {
    /// The denormalized raw Package URL from the CdxComponent.
    #[skip_serializing_none]
    pub purl: Option<String>,

    /// The name of the package.
    pub name: String,

    /// The package manager of the package.
    #[skip_serializing_none]
    pub package_manager: Option<String>,

    /// The type of the package (e.g. application, library, container).
    pub component_type: ComponentType,

    /// The unique identifier of the component, service, or vulnerability within the BOM.
    #[skip_serializing_none]
    pub bom_ref: Option<String>,

    /// The version specifier for the package.
    #[skip_serializing_none]
    pub version: Option<String>,

    /// The denormalized raw Package URLs from the SBOM Components list.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub dependencies: Vec<String>,
}

#[serde(rename_all = "camelCase")]
impl Cdx {
    /// Creates a Cdx summary from a full CycloneDx Bom.
    pub(crate) fn from_bom(bom: &Bom, package_manager: Option<String>) -> Result<Cdx, Error> {
        let component = match bom.component() {
            None => {
                return Err(Error::Entity("no_bom_component".to_string()));
            }
            Some(c) => c,
        };

        let mut dependencies = vec![];

        match &bom.components {
            None => {}
            Some(c) => c
                .iter()
                .for_each(|component: Component| match component.purl {
                    None => {}
                    Some(purl) => dependencies.push(purl),
                }),
        }

        Ok(Self {
            component_type: component.r#type,
            bom_ref: component.bom_ref,
            name: component.name,
            version: component.version,
            purl: component.purl,
            package_manager,
            dependencies,
        })
    }

    /// Compares the purl of two CycloneDxComponents for equality.
    pub fn purl_eq(&self, other: Option<CdxComponent>) -> bool {
        // Guard against cases where equality cannot be evaluated.
        let self_purl = match &self.purl {
            None => {
                return false;
            }
            Some(purl) => purl,
        };

        let other_purl = match other {
            None => {
                return false;
            }
            Some(other) => match other.purl {
                None => {
                    return false;
                }
                Some(purl) => purl,
            },
        };

        self_purl.eq(&other_purl)
    }
}
