use crate::entities::cyclonedx::component::ComponentType;
use crate::entities::cyclonedx::{Bom, Component};
use crate::entities::packages::Purl;
use crate::Error;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::string::FromUtf8Error;
use tracing::log::debug;
use urlencoding::decode;

/// A subset of the full CycloneDx Component suitable for tracking packages and dependencies within
/// the registry.
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

impl Cdx {
    /// Creates a Cdx summary from a full CycloneDx Bom. Typically used during [Package] creation.
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
                    Some(purl) => {
                        match Purl::decode(purl.as_str()) {
                            Ok(purl) => dependencies.push(purl),
                            Err(e) => {
                                debug!("cdx::from_bom::purl:decode::{}::{}", purl, e);
                            }
                        };
                    }
                }),
        }

        let purl = Purl::decode(purl)?;

        Ok(Self {
            component_type: component.r#type,
            bom_ref: component.bom_ref,
            name: component.name,
            version: component.version,
            purl: Some(purl),
            package_manager,
            dependencies,
        })
    }

    /// Creates a Cdx summary from a CycloneDx Component. Typically used during [Dependency]
    /// creation.
    pub fn from_component(component: &Component, package_manager: Option<String>) -> Cdx {
        Self {
            component_type: component.r#type,
            bom_ref: component.bom_ref.clone(),
            name: component.name.clone(),
            version: component.version.clone(),
            purl: component.purl.clone(),
            package_manager,
            dependencies: vec![],
        }
    }

    /// Compares the purl of two Cdx instances for equality.
    pub fn purl_eq(&self, other: Option<Cdx>) -> bool {
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

    // TODO: Create a way to hydrate original source Bom/Component from S3.
}
