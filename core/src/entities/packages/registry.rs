use crate::entities::packages::{Dependency, Package, Unsupported};
use crate::Error;

// WATCH: This is useful for early prototyping, but we will likely outgrow this.
/// Provides intelligent access to the full set of Packages.
#[serde(rename_all = "camelCase")]
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
        Self {
            packages: vec![],
            dependencies: vec![],
            unsupported: vec![],
        }
    }

    /// Add a CycloneDx Package to the Registry.
    pub(crate) fn cyclonedx_package(&mut self, package: Package) -> Result<(), Error> {
        // Validate the incoming package and return error for metrics.
        if package.cdx_component.is_none() {
            return Err(Error::Entity("no_package_component".to_string()));
        }

        let existing = self.packages.iter_mut().find(|p| {
            p.cdx_component
                .as_ref()
                .unwrap()
                .purl_eq(package.cdx_component.clone())
        });

        match existing {
            None => {
                self.packages.push(package.to_owned());
            }
            Some(p) => {
                for xref in package.snyk_refs {
                    p.snyk_refs(xref.clone());
                }
            }
        };

        Ok(())
    }

    /// Add a CycloneDx Dependency to the Registry.
    pub(crate) fn cyclonedx_dependencies(&mut self, dependency: Dependency) -> Result<(), Error> {
        if dependency.cdx_component.is_none() {
            return Err(Error::Entity("no_dependency_component".to_string()));
        }

        let existing = self.dependencies.iter_mut().find(|d| {
            d.cdx_component
                .as_ref()
                .unwrap()
                .purl_eq(dependency.cdx_component.clone())
        });

        match existing {
            None => {
                self.dependencies.push(dependency.to_owned());
            }
            Some(d) => {
                for xref in dependency.snyk_refs {
                    d.xref.snyk(xref.clone());
                }
            }
        };

        Ok(())
    }

    /// Track and unsupported package.
    pub(crate) fn unsupported(&mut self, unsupported: Unsupported) {
        let existing = self
            .unsupported
            .iter_mut()
            .find(|u| u.name.eq(&unsupported.name));

        match existing {
            None => {
                self.unsupported.push(unsupported.to_owned());
            }
            Some(u) => {
                for xref in unsupported.xref.snyk {
                    u.xref.snyk(xref.clone());
                }
            }
        };
    }
}
