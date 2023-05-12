use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{Display, Formatter};
use tracing::log::debug;

use crate::entities::cyclonedx::{Bom, Component};
use crate::entities::packages::{Dependency, Package};
use crate::entities::sboms::SbomProviderKind;
use crate::entities::tasks::TaskRef;
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
    pub spec: Spec,

    /// The actor that that was the source of the [Sbom].
    pub source: Source,

    /// The system or tool that generated the [Sbom] if known.
    pub provider: Option<SbomProviderKind>,

    /// The unix timestamp for when the [Sbom] was created.
    pub timestamp: u64,

    /// The checksum of the file.
    pub checksum_sha256: String,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Vec<Xref>,

    /// Reference to each [Task] that was performed against this [Sbom] instance.
    pub task_refs: Vec<TaskRef>,

    /// Denormalized list of dependency refs.
    pub dependency_refs: Option<Vec<String>>,

    /// CycloneDx JSON spec model of Sbom. Hydrated at runtime.
    #[serde(skip)]
    pub(crate) bom: Option<Bom>,

    /// [Package] instance that represents this Sbom. Hydrated at runtime.
    #[serde(skip)]
    pub package: Option<Package>,

    /// [Packages] this Sbom depends on. Hydrated at runtime.
    #[serde(skip)]
    pub dependencies: Vec<Dependency>,

    /// [Packages] that depend on this Sbom. Hydrated at runtime.
    #[serde(skip)]
    pub dependents: Vec<Dependency>,
}

impl Sbom {
    /// Factory method to create new instance of type from a CycloneDx model.
    pub fn from_raw_cdx(
        raw: &str,
        format: CdxFormat,
        source: Source,
        package_manager: &Option<String>,
        xref: Xref,
    ) -> Result<Sbom, Error> {
        let bom: Bom = match Bom::parse(raw, CdxFormat::Json) {
            Ok(bom) => bom,
            Err(e) => {
                let msg = format!("from_raw_cdx::bom::parse::{}", e);
                debug!("{}", msg);
                return Err(Error::Entity(msg));
            }
        };

        let purl = match bom.purl() {
            None => {
                return Err(Error::Entity("from_raw_cdx::purl_none".to_string()));
            }
            Some(purl) => {
                if purl.is_empty() {
                    return Err(Error::Entity("from_raw_cdx::purl_empty".to_string()));
                }
                purl
            }
        };

        let provider = match source.clone() {
            Source::Harbor(provider) => provider,
            Source::Vendor(name) => SbomProviderKind::Vendor(name),
        };

        let package = Package::from_bom(&bom, package_manager.clone(), xref.clone())?;
        let package_ref = purl.clone();

        let mut dependency_refs = vec![];
        let mut dependencies = vec![];

        let components = match &bom.components {
            None => vec![],
            Some(components) => components.to_vec(),
        };

        components
            .iter()
            .for_each(|component| match &component.purl {
                None => {}
                Some(purl) => {
                    dependency_refs.push(purl.clone());
                    dependencies.push(Dependency::from_component(
                        component,
                        provider.clone(),
                        package_ref.clone(),
                        package_manager.clone(),
                        xref.clone(),
                    ));
                }
            });

        let dependency_refs = match dependency_refs.is_empty() {
            true => Some(dependency_refs),
            false => None,
        };

        let sbom = Sbom {
            id: "".to_string(),
            purl: Some(purl),
            instance: 1,
            spec: Spec::Cdx(format),
            source,
            package_manager: package_manager.clone(),
            provider: Some(provider),
            timestamp: platform::time::timestamp()?,
            checksum_sha256: "".to_string(),
            task_refs: vec![],
            xrefs: vec![xref],
            bom: Some(bom),
            package: Some(package),
            dependencies,
            dependents: vec![],
            dependency_refs,
        };

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

    /// Utility function to get the current iteration by [TaskRef].
    pub fn iteration(&self) -> u32 {
        match self.task_refs.iter().max_by_key(|s| s.iteration) {
            Some(s) => s.iteration,
            _ => 1,
        }
    }

    /// Utility function to get the next iteration by [TaskRef].
    pub fn next_iteration(&self) -> u32 {
        match self.task_refs.iter().max_by_key(|s| s.iteration) {
            Some(s) => s.iteration + 1,
            _ => 1,
        }
    }

    /// Add a [TaskRef] to the [Sbom].
    pub fn task_refs(&mut self, task_ref: &mut TaskRef) {
        task_ref.iteration = self.next_iteration();
        self.task_refs.push(task_ref.to_owned());
    }
}

/// A Spec is the SBOM specification to which the SBOM conforms.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Spec {
    /// SBOM is a CycloneDx document.
    Cdx(CdxFormat),

    /// SBOM is an Spdx document.
    Spdx(SpdxFormat),
}

impl Display for Spec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Spec::Cdx(format) => write!(f, "cdx::{}", format),
            Spec::Spdx(format) => write!(f, "spdx::{}", format),
        }
    }
}

/// CdxFormat is the document encoding format for the CycloneDx [Spec].
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

/// SpdxFormat is the document encoding format for the Spdx [Spec].
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
pub enum Source {
    /// SBOM produced by Harbor using the specified internal provider (e.g. GitHub, Snyk)
    Harbor(SbomProviderKind),
    /// SBOM provided by the vendor.
    Vendor(String),
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Harbor(provider) => write!(f, "harbor::{}", provider),
            Source::Vendor(name) => write!(f, "vendor::{}", name.to_lowercase()),
        }
    }
}
