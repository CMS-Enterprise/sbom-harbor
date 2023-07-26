use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{Display, Formatter};
use tracing::log::debug;

use crate::entities::cyclonedx::{Bom, Component};
use crate::entities::packages::Package;
use crate::entities::sboms::SbomProviderKind;
use crate::entities::tasks::{Task, TaskRef};
use crate::entities::xrefs::Xref;
use crate::Error;

/// An SBOM is a snapshot inventory of the components that make up a piece of software at a moment in time.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Sbom {
    /// The unique identifier for the Sbom.
    pub id: String,

    /// The package manager for the [Sbom].
    pub package_manager: Option<String>,

    /// The Package URL for the package that the Sbom was generated from.
    pub purl: Option<String>,

    /// The instance of the [Sbom]. Forward-only incrementing counter.
    pub instance: u32,

    /// The spec the [Sbom] conforms to.
    pub kind: SpecKind,

    /// The system or tool that generated the [Sbom] if known. Should satisfy the Author
    /// element as specified by the NTIA SBOM Minimum Elements.
    pub author: Author,

    /// Denormalized from Author. Allows partitioning SBOMs by the tool that was used to generate
    /// them.
    pub provider: Option<SbomProviderKind>,

    /// The name of an entity that creates, defines, and identifies the component that this [Sbom]
    /// pertains to. Part of the NTIA SBOM Minimum Elements.
    pub supplier_name: Option<String>,

    /// Designation assigned to a unit of software defined by the original supplier as specified by
    /// the NTIA SBOM Minimum Elements.
    pub component_name: Option<String>,

    /// Identifier used by the supplier to specify a change in software from a previously identified
    /// version as specified by the NTIA SBOM Minimum Elements.
    pub version: Option<String>,

    /// Identifiers that are used to identify a component, or serve as a look-up key for relevant
    /// databases as specified by the NTIA SBOM Minimum Elements. (e.g. CPE).
    pub other_identifiers: Option<Vec<String>>,

    /// Denormalized list of dependency refs as specified by the NTIA SBOM Minimum Elements.
    pub dependency_refs: Option<Vec<String>>,

    /// The iso8601 timestamp. for when the [Sbom] was created and used for display.
    pub created: Option<String>,

    /// The unix timestamp for when the [Sbom] was created as specified by the NTIA SBOM Minimum Elements.
    pub timestamp: u64,

    /// The checksum of the file.
    pub checksum_sha256: String,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Vec<Xref>,

    /// Reference to each [Task] that was performed against this [Sbom] instance.
    pub task_refs: Vec<TaskRef>,

    /// CycloneDx JSON spec model of Sbom. Hydrated at runtime.
    #[serde(skip)]
    pub(crate) bom: Option<Bom>,

    /// [Package] instance that represents this Sbom. Hydrated at runtime.
    #[serde(skip)]
    pub package: Option<Package>,

    /// [Package] instances that represent dependencies defined in this Sbom. Hydrated at runtime.
    #[serde(skip)]
    pub dependencies: Option<Vec<Package>>,
}

impl Sbom {
    /// Factory method to create new instance of type from a CycloneDx model.
    pub fn from_raw_cdx(
        raw: &str,
        format: CdxFormat,
        author: Author,
        package_manager: &Option<String>,
        xref: Xref,
        task: Option<&Task>,
    ) -> Result<Sbom, Error> {
        let bom: Bom = match Bom::parse(raw, CdxFormat::Json) {
            Ok(bom) => bom,
            Err(e) => {
                let msg = format!("from_raw_cdx::bom::parse::{}", e);
                debug!("{}", msg);
                return Err(Error::Entity(msg));
            }
        };

        let component_name = bom.component_name();
        let supplier_name = bom.supplier_name();
        let version = bom.component_version();

        let purl = match bom.purl() {
            None => {
                // TODO: Integrate a default name & version/commit hash from GitHub provider.
                match bom.try_build_purl_from_metadata(component_name.clone(), version.clone()) {
                    None => {
                        return Err(Error::Entity("from_raw_cdx::purl_none".to_string()));
                    }
                    Some(p) => p,
                }
            }
            Some(p) => p,
        };

        if purl.is_empty() {
            return Err(Error::Entity("from_raw_cdx::purl_empty".to_string()));
        }

        let provider = match author.clone() {
            Author::Harbor(provider) => provider,
            Author::Vendor(name) => SbomProviderKind::Vendor(name),
        };

        let package = Package::from_bom(&bom, package_manager.clone(), xref.clone(), task)?;

        let other_identifiers = bom.cpe().map(|cpe| vec![cpe]);

        let dependency_refs = package.dependency_refs.clone();

        let created = match platform::time::iso8601_timestamp() {
            Ok(created) => Some(created),
            Err(e) => {
                println!("timestamp creation failed with {}", e);
                None
            }
        };

        let mut sbom = Sbom {
            id: "".to_string(),
            purl: Some(purl.clone()),
            instance: 1,
            kind: SpecKind::Cdx(format),
            author,
            component_name,
            version,
            supplier_name,
            package_manager: package_manager.clone(),
            provider: Some(provider),
            created,
            timestamp: platform::time::timestamp()?,
            checksum_sha256: "".to_string(),
            task_refs: vec![],
            xrefs: vec![xref],
            bom: Some(bom),
            package: Some(package),
            dependency_refs,
            other_identifiers,
            dependencies: None,
        };

        match task {
            None => {}
            Some(task) => {
                sbom.join_task(purl, task)?;
            }
        }

        Ok(sbom)
    }

    /// Accessor function to encapsulate logic for accessing the purl field with consistent errors
    /// that can be reported as metrics.
    pub fn purl(&self) -> Result<String, Error> {
        let purl = match &self.purl {
            None => {
                return Err(Error::Entity("sbom_purl_none".to_string()));
            }
            Some(purl) => purl,
        };

        if purl.is_empty() {
            return Err(Error::Entity("sbom_purl_empty".to_string()));
        }

        Ok(purl.clone())
    }

    /// Accessor function to simplify accessing the Sbom as as CycloneDx Component.
    pub fn component(&self) -> Option<Component> {
        match &self.bom {
            None => None,
            Some(bom) => bom.component(),
        }
    }

    /// Add a [TaskRef] to the [Sbom].
    pub fn task_refs(&mut self, task_ref: &TaskRef) {
        self.task_refs.push(task_ref.to_owned());
    }

    /// Sets up a reference between the [Sbom] and the [Task].
    pub fn join_task(&mut self, target_id: String, task: &Task) -> Result<TaskRef, Error> {
        if task.id.is_empty() {
            return Err(Error::Entity("task_id_required".to_string()));
        }

        let task_ref = TaskRef::new(task, target_id);

        let result = task_ref.clone();
        self.task_refs.push(task_ref);

        Ok(result)
    }
}

/// The SBOM specification kind of the [Sbom].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SpecKind {
    /// SBOM is a CycloneDx document.
    Cdx(CdxFormat),

    /// SBOM is an Spdx document.
    Spdx(SpdxFormat),
}

impl Display for SpecKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecKind::Cdx(format) => write!(f, "cdx::{}", format),
            SpecKind::Spdx(format) => write!(f, "spdx::{}", format),
        }
    }
}

/// CdxFormat is the document encoding format for the CycloneDx [SpecKind].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CdxFormat {
    /// Document is a JSON document.
    Json,
    /// Document is an XML document.
    Xml,
}

impl Display for CdxFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CdxFormat::Json => write!(f, "json"),
            CdxFormat::Xml => write!(f, "xml"),
        }
    }
}

/// SpdxFormat is the document encoding format for the Spdx [SpecKind].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SpdxFormat {
    /// Document is a JSON document.
    Json,
    /// Document is an RDF document. Not currently supported. Should throw validation error.
    Rdf,
    /// Document is a TagValue document. Not currently supported. Should throw validation error.
    TagValue,
    /// Document is a spreadsheet document. Not currently supported. Should throw validation error.
    Spreadsheet,
    /// Document is a YAML document. Not currently supported. Should throw validation error.
    Yaml,
}

impl Display for SpdxFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SpdxFormat::Json => write!(f, "json"),
            SpdxFormat::Rdf => write!(f, "rdf"),
            SpdxFormat::TagValue => write!(f, "tag-value"),
            SpdxFormat::Spreadsheet => write!(f, "spreadsheet"),
            SpdxFormat::Yaml => write!(f, "yaml"),
        }
    }
}

/// The actor that provided or generated the Sbom.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Author {
    /// SBOM produced by Harbor using the specified internal provider (e.g. GitHub, Snyk)
    Harbor(SbomProviderKind),
    /// SBOM provided by the vendor.
    Vendor(String),
}

impl Display for Author {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Author::Harbor(provider) => write!(f, "harbor::{}", provider),
            Author::Vendor(name) => write!(f, "vendor::{}", name.to_lowercase()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::xrefs::XrefKind;
    use crate::testing::sbom_raw;
    use crate::Error;
    use std::collections::HashMap;

    #[test]
    fn can_parse_sbom_test_fixture() -> Result<(), Error> {
        let raw = sbom_raw()?;

        let sbom = Sbom::from_raw_cdx(
            raw.as_str(),
            CdxFormat::Json,
            Author::Vendor("can_parse_sbom_test_fixture".to_string()),
            &None,
            Xref {
                kind: XrefKind::Product,
                map: HashMap::default(),
            },
            None,
        )?;

        assert!(sbom.dependency_refs.is_some());
        match &sbom.dependency_refs {
            None => {}
            Some(dependencies) => {
                assert!(dependencies.len() > 1);
            }
        }

        Ok(())
    }
}
