use crate::entities::cyclonedx::{Bom, Component};
use crate::entities::enrichments::Vulnerability;
use crate::entities::packages::PackageCdx;
use crate::entities::tasks::{Task, TaskRef};
use crate::entities::xrefs::Xref;
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{Display, Formatter};

/// A [Package] is a item for which an SBOM can be generated. It serves as an aggregate root for all
/// version of an SBOM, and as a way of cross-referencing SBOMs across disparate systems.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Package {
    /// The unique identifier for the Package.
    pub id: String,

    /// Discriminator indicating if the Package defines a [Package] being monitored or a dependency
    /// of a [Package] being monitored.
    pub kind: PackageKind,

    /// The package manager for the [Package].
    pub package_manager: Option<String>,

    /// Optional Package URL if known.
    pub purl: Option<String>,

    /// The package version.
    pub version: Option<String>,

    /// Optional Common Platform Enumeration (CPE) identifier if known.
    pub cpe: Option<String>,

    /// Encapsulates CycloneDx specific attributes.
    pub cdx: Option<PackageCdx>,

    /// Characterizing the relationship that an upstream component X is included in software Y.
    pub dependency_refs: Option<Vec<String>>,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Vec<Xref>,

    /// Reference to each [Task] that was performed against this [Package].
    pub task_refs: Vec<TaskRef>,

    /// Dependencies represented as packages. Hydrated at runtime.
    #[serde(skip)]
    pub(crate) dependencies: Vec<Package>,

    /// Vulnerabilities associated with this [Package]. Hydrated at runtime.
    #[serde(skip)]
    pub(crate) vulnerabilities: Vec<Vulnerability>,
}

impl Package {
    /// Factory method for creating a [Package] entity by analyzing a CycloneDx [Bom] instance.
    pub fn from_bom(
        bom: &Bom,
        package_manager: Option<String>,
        xref: Xref,
        task: Option<&Task>,
    ) -> Result<Package, Error> {
        let cdx = PackageCdx::from_bom(bom, package_manager.clone())?;
        let version = cdx.version.clone();
        let purl = match &cdx.purl {
            None => {
                return Err(Error::Entity(format!("package_purl_none::{}", cdx.name)));
            }
            Some(purl) => purl.clone(),
        };

        let cdx = Some(cdx);
        let mut dependency_refs = vec![];
        let mut dependencies = vec![];

        match &bom.components {
            None => {}
            Some(components) => {
                for component in components.iter() {
                    match &component.purl {
                        None => {
                            return Err(Error::Entity(format!(
                                "dependency_purl_none::{}",
                                component.name
                            )));
                        }
                        Some(purl) => {
                            dependency_refs.push(purl.clone());
                        }
                    }

                    match Package::from_dependency(
                        component,
                        package_manager.clone(),
                        xref.clone(),
                        task,
                    ) {
                        Ok(dependency) => dependencies.push(dependency),
                        Err(e) => {
                            return Err(Error::Entity(format!(
                                "dependency_package_error::{}::{}",
                                component.name, e
                            )));
                        }
                    }
                }
            }
        };

        let mut package = Package {
            id: "".to_string(),
            kind: PackageKind::Primary,
            package_manager,
            version,
            cpe: bom.cpe(),
            purl: Some(purl.clone()),
            cdx,
            dependency_refs: Some(dependency_refs),
            xrefs: vec![xref],
            task_refs: vec![],
            dependencies,
            vulnerabilities: vec![],
        };

        match task {
            None => {}
            Some(task) => {
                package.join_task(purl, task)?;
            }
        }

        Ok(package)
    }

    /// Creates a [Package] instance from a CycloneDx dependency.
    pub fn from_dependency(
        component: &Component,
        package_manager: Option<String>,
        xref: Xref,
        task: Option<&Task>,
    ) -> Result<Package, Error> {
        let cdx = PackageCdx::from_dependency(component, package_manager.clone());
        let version = cdx.version.clone();

        let purl = match &cdx.purl {
            None => {
                return Err(Error::Entity(format!("dependency_purl_none::{}", cdx.name)));
            }
            Some(purl) => purl.clone(),
        };

        let cdx = Some(cdx);

        let mut dependency = Package {
            id: "".to_string(),
            kind: PackageKind::Dependency,
            package_manager,
            version,
            cpe: component.cpe.clone(),
            purl: Some(purl.clone()),
            cdx,
            dependency_refs: Some(vec![]),
            xrefs: vec![xref],
            task_refs: vec![],
            dependencies: vec![],
            vulnerabilities: vec![],
        };

        match task {
            None => {}
            Some(task) => {
                dependency.join_task(purl, task)?;
            }
        }

        Ok(dependency)
    }

    /// Sets up a reference between the [Package] and the [Task]. Callers specify the target_id
    /// being used by the task. This may vary by process from the unique ID to the purl or
    /// potentially the CPE or some other identifier.
    pub fn join_task(&mut self, target_id: String, task: &Task) -> Result<TaskRef, Error> {
        if task.id.is_empty() {
            return Err(Error::Entity("task_id_required".to_string()));
        }

        let task_ref = TaskRef::new(task, target_id);

        let result = task_ref.clone();
        self.task_refs.push(task_ref);

        Ok(result)
    }

    /// Add a [TaskRef] to the [Purl].
    pub fn task_refs(&mut self, task_ref: &TaskRef) {
        if !self.task_refs.iter().any(|s| s.task_id == task_ref.task_id) {
            self.task_refs.push(task_ref.clone());
        }
    }

    /// Appends Vulnerabilities to the Purl.
    pub fn vulnerabilities(&mut self, new: &Vulnerability) {
        if self.vulnerabilities.is_empty() {
            self.vulnerabilities = vec![new.clone()];
            return;
        }

        let mut current = self.vulnerabilities.clone();

        match self
            .vulnerabilities
            .iter()
            .any(|existing| existing.cve == new.cve && existing.provider == new.provider)
        {
            true => {}
            false => {
                current.push(new.clone());
            }
        }

        self.vulnerabilities = current;
    }
}

/// Discriminator that indicates whether the Purl was extracted from a [Package] or a [Dependency].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PackageKind {
    /// Indicates a [Package] is being directly monitored.
    Primary,
    /// Indicates a [Package] is a dependency of a [Package] being directly monitored.
    Dependency,
}

impl Display for PackageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageKind::Primary => write!(f, "primary"),
            PackageKind::Dependency => write!(f, "dependency"),
        }
    }
}
