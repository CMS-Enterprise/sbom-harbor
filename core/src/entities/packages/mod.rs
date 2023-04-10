use std::collections::HashMap;
use platform::mongodb::{MongoDocument, mongo_doc};
use serde::{Deserialize, Serialize};
use tracing::debug;
use crate::models::cyclonedx::Component;
use crate::models::cyclonedx::component::ComponentType;

mongo_doc!(Package);
mongo_doc!(Dependency);
mongo_doc!(Unsupported);
mongo_doc!(Purl);

// WATCH: This is useful for early prototyping, but we will likely outgrow this.
/// Provides intelligent access to the full set of Packages.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Registry {
    /// The set of Packages tracked by the Registry.
    pub packages: Vec<Package>,

    /// The set of Dependencies tracked by the Registry.
    pub dependencies: Vec<Dependency>,

    /// The set of Packages that have been identified but are not supported.
    pub unsupported: Vec<Unsupported>,
}

impl Registry {
    /// Factory method to create new instance of type.
    pub fn new() -> Self {
        Self{
            packages: vec![],
            dependencies: vec![],
            unsupported: vec![],
        }
    }

    /// Add a CycloneDx Package to the Registry.
    pub(crate) fn cyclonedx_packages(&mut self, package: Package) {
        if package.cdx_component.is_none() {
            debug!("attempted to add a CycloneDx Package without component");
            return;
        }

        let existing = self.packages
            .iter_mut()
            .find(|p| {
                p.cdx_component.as_ref().unwrap().purl_eq(package.cdx_component.clone())
            });

        match existing {
            None => {
                self.packages.push(package.to_owned());
            },
            Some(p) => {
                for xref in package.xref.snyk {
                    p.xref.snyk(xref.clone());
                }
            },
        };
    }

    /// Add a CycloneDx Dependency to the Registry.
    pub(crate) fn cyclonedx_dependencies(&mut self, dependency: Dependency) {
        if dependency.cdx_component.is_none() {
            debug!("attempted to add a CycloneDx Dependency without component");
            return;
        }

        let existing = self.dependencies
            .iter_mut()
            .find(|d| {
                d.cdx_component.as_ref().unwrap().purl_eq(dependency.cdx_component.clone())
            });

        match existing {
            None => {
                self.dependencies.push(dependency.to_owned());
            },
            Some(d) => {
                for xref in dependency.xref.snyk {
                    d.xref.snyk(xref.clone());
                }
            },
        };
    }

    /// Track and unsupported package.
    pub(crate) fn unsupported(&mut self, unsupported: Unsupported) {
        let existing = self.unsupported
            .iter_mut()
            .find(|u| u.name.eq(&unsupported.name));

        match existing {
            None => {
                self.unsupported.push(unsupported.to_owned());
            },
            Some(u) => {
                for xref in unsupported.xref.snyk {
                    u.xref.snyk(xref.clone());
                }
            },
        };
    }
}

/// A [Package] is a item for which an SBOM can be generated. It serves as an aggregate root for all
/// version of an SBOM, and as a way of cross-referencing SBOMs across disparate systems.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Package {
    /// The unique identifier for the Package.
    pub id: String,

    /// The package manager of the package.
    pub manager: Option<String>,

    /// Component struct for CycloneDx Packages.
    pub cdx_component: Option<CycloneDxComponent>,

    /// Cross-references from the package to model type or external systems.
    pub xref: PackageXRef,
}

impl Package {
    pub fn snyk_ref(&mut self, snyk_ref: SnykXRef) {
        if self.has_snyk_ref(&snyk_ref) {
            return;
        }

        self.xref.snyk.push(snyk_ref);
    }

    fn has_snyk_ref(&self, snyk_ref: &SnykXRef) -> bool {
        self.xref.snyk.iter()
            .any(|r|{
                r.project_id == snyk_ref.project_id
                && r.group_id == snyk_ref.group_id
                && r.org_id == snyk_ref.org_id
            })
    }
}

/// A subset of the full CycloneDx Component suitable for tracking packages within the registry.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CycloneDxComponent {
    /// The type of the package (e.g. application, library, container).
    pub component_type: ComponentType,

    /// The unique identifier of the component, service, or vulnerability within the BOM.
    #[serde(rename = "bom-ref")]
    pub bom_ref: Option<String>,

    /// The name of the package.
    pub name: String,

    /// The version specifier for the package.
    pub version: Option<String>,

    /// The package url (e.g. pkg:npm/@aws-cdk/asset-kubectl-v20@2.1.1).
    pub purl: Option<String>,
}

impl From<Component> for CycloneDxComponent {
    fn from(value: Component) -> Self {
        Self{
            component_type: value.r#type,
            bom_ref: value.bom_ref,
            name: value.name,
            version: value.version,
            purl: value.purl,
        }
    }
}

impl CycloneDxComponent {
    /// Compares the purl of two CycloneDxComponents for equality.
    pub fn purl_eq(&self, other: Option<CycloneDxComponent>) -> bool {
        // Guard against cases where equality cannot be evaluated.
        let self_purl = match &self.purl {
            None => { return false; }
            Some(purl) => purl,
        };

        let other_purl = match other {
            None => { return false; }
            Some(other) => {
                match other.purl {
                    None => { return false; }
                    Some(purl) => purl,
                }
            }
        };

        self_purl.eq(&other_purl)
    }
}

/// A dependency identified for a Package.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dependency {
    /// The unique identifier for the Package.
    pub id: String,

    // TODO: Not sure if we can assume the package manager for a dependency in all cases.
    // The package manager of the package.
    //pub manager: Option<String>,

    /// Component struct for CycloneDx Packages.
    pub cdx_component: Option<CycloneDxComponent>,

    /// Cross-references from the dependency to model type or external systems that consumes it.
    pub xref: PackageXRef,
}

/// Unsupported models code packages for which an SBOM cannot be produced (e.g. terraform projects).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Unsupported {
    /// The unique identifier for the unsupported Package.
    pub id: String,

    /// The name of the unsupported package.
    pub name: String,

    /// The package manager of the package.
    pub manager: Option<String>,

    /// Cross-references from the dependency to model type or external systems that consumes it.
    pub xref: PackageXRef,
}

impl Unsupported {
    pub fn snyk_ref(&mut self, snyk_ref: SnykXRef) {
        if self.has_snyk_ref(&snyk_ref) {
            return;
        }

        self.xref.snyk.push(snyk_ref);
    }

    fn has_snyk_ref(&self, snyk_ref: &SnykXRef) -> bool {
        self.xref.snyk.iter()
            .any(|r|{
                r.project_id == snyk_ref.project_id
                && r.group_id == snyk_ref.group_id
                && r.org_id == snyk_ref.org_id
            })
    }
}

/// Purl is a derived type that facilitates analysis of a Package across the entire enterprise.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Purl {
    /// The package URL.
    pub(crate) id: String,

    /// The package name extracted from the package URL.
    pub(crate) name: String,

    /// The package version extracted from the package URL.
    pub(crate) version: String,

    /// Source of the Purl.
    pub source: PurlSource,

    /// A map of projects that are associated with the Purl.
    pub(crate) snyk_refs: Vec<SnykXRef>,
}

impl Purl {
    pub(crate) fn parse(package_url: String) -> (String, String) {
        if !package_url.contains("@") {
            return (package_url, "".to_string());
        }

        if package_url.matches("@").count() > 1 {
            return (package_url, "unknown".to_string());
        }

        let parts: Vec<&str> = package_url.split("@").collect();
        if parts.len() != 2 {
            debug!("should not be possible")
        }

        (parts[0].to_string(), parts[1].to_string())
    }
}

/// The Source of the Purl.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PurlSource {
    /// The Purl was extracted from the Component Metadata of a Package in the Registry.
    Package,
    /// The Purl was extracted from a Dependency of a Package in the Registry.
    Dependency,
}

impl ToString for PurlSource {
    fn to_string(&self) -> String {
        match self {
            PurlSource::Package => "package".to_string(),
            PurlSource::Dependency => "dependency".to_string(),
        }
    }
}

impl Purl {
    pub(crate) fn merge_snyk_refs(&mut self, refs: Vec<SnykXRef>) {
        for r in refs {
            if self.snyk_refs.iter().any(|existing| {
                r.org_id.eq(&existing.org_id)
                && r.project_id.eq(&existing.project_id)
                && r.group_id.eq(&existing.group_id)
            }) { continue; }

            self.snyk_refs.push(r);
        }

    }
}

// TODO: This should really be a HashMap<&str, HashMap<&str, &str> to allow dynamism.
// TODO: I'm leaving these as strong types during the modeling phase to make it easier to collaborate.
/// PackageXRef contains the metadata used to cross-reference an SBOM to another system or subsystem.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PackageXRef {
    /// FISMA cross-references.
    pub fisma: Vec<FismaXRef>,
    /// Codebase cross-references.
    pub codebase: Vec<CodebaseXRef>,
    /// Product cross-references.
    pub product: Vec<ProductXRef>,
    /// Snyk cross-references.
    pub snyk: Vec<SnykXRef>,
}

impl PackageXRef {
    /// Factory method to create new instance of type.
    pub fn new() -> Self {
        Self{
            fisma: vec![],
            codebase: vec![],
            product: vec![],
            snyk: vec![],
        }
    }

    pub fn snyk(&mut self, xref: SnykXRef) {
        match self.snyk.iter().any(|x| {
            x.org_id == xref.org_id
            && x.group_id == xref.group_id
            && x.project_id == xref.project_id
        }) {
            true => {},
            false => { self.snyk.push(xref); },
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FismaXRef {
    pub fisma_id: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodebaseXRef {
    pub team_id: String,
    pub project_id: String,
    pub codebase_id: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductXRef {
    pub vendor_id: String,
    pub product_id: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnykXRef {
    pub org_id: String,
    pub org_name: String,
    pub group_id: String,
    pub group_name: String,
    pub project_id: String,
    pub project_name: String,
}
