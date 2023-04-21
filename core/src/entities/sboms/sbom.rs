use serde::{Deserialize, Serialize, Serializer};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use tracing::log::debug;

use crate::entities::cyclonedx::Bom;
use crate::entities::enrichment::{Scan, ScanRef};
use crate::entities::packages::{Dependency, Package, PackageCdx, Purl};
use crate::entities::sboms::SbomProviderKind;
use crate::entities::xrefs::{Xref, XrefKind};
use crate::services::snyk::{SnykRef, SNYK_DISCRIMINATOR};
use crate::Error;

/// An SBOM is a snapshot inventory of the components that make up a piece of software at a moment in time.
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sbom {
    /// The unique identifier for the Sbom.
    pub id: String,

    /// The Package URL for the package that the Sbom was generated from.
    pub purl: Option<String>,

    /// The instance of the [Sbom]. Forward-only incrementing counter.
    pub instance: u32,

    /// The spec the [Sbom] conforms to.
    pub spec: Spec,

    /// The entity that that was the source of the [Sbom].
    pub source: Source,

    /// The system or tool that generated the [Sbom] if known.
    pub provider: Option<SbomProviderKind>,

    /// The unix timestamp for when the [Sbom] was created.
    pub timestamp: u64,

    /// The checksum of the file.
    pub checksum_sha256: String,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Option<HashMap<XrefKind, Xref>>,

    /// Reference to each [Scan] that was performed against this [Sbom].
    pub scan_refs: Vec<ScanRef>,

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
        scan: &Scan,
        xrefs: Option<HashMap<XrefKind, Xref>>,
    ) -> Result<Sbom, Error> {
        let bom: Bom = match Bom::parse(raw, CdxFormat::Json) {
            Ok(bom) => bom,
            Err(e) => {
                let msg = format!("from_raw_cdx::bom::parse::{}", e);
                debug!("{}", msg);
                return Err(Error::Entity(msg.to_string()));
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
            Source::Harbor(provider) => Some(provider),
            Source::Vendor(_) => None,
        };

        let mut sbom = Sbom {
            id: "".to_string(),
            purl: Some(purl),
            instance: 1,
            spec: Spec::Cdx(format),
            source,
            provider,
            timestamp: platform::time::timestamp()?,
            checksum_sha256: "".to_string(),
            scan_refs: vec![],
            xrefs,
            bom: Some(bom),
            package: Default::default(),
            dependencies: vec![],
            dependents: vec![],
        };

        sbom.scan_refs(scan, None)?;

        Ok(sbom)
    }

    /// Accessor method to encapsulate logic for accessing the purl field with consistent errors
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

    // TODO: This could/should be a macro.
    pub fn scan_refs(&mut self, mut scan: &Scan, err: Option<String>) -> Result<(), Error> {
        if scan.id.is_empty() {
            return Err(Error::Entity("scan_id_required".to_string()));
        }

        let mut scan_ref = ScanRef {
            id: "".to_string(),
            scan_id: scan.id.clone(),
            purl: self.purl.clone(),
            iteration: 1,
            err,
        };

        scan_ref.iteration = match self.scan_refs.iter().max_by_key(|s| s.iteration) {
            Some(s) => s.iteration + 1,
            _ => 1,
        };

        self.scan_refs.push(scan_ref);

        Ok(())
    }
}

/// A Spec is the SBOM specification to which the SBOM conforms.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
            Spec::Spdx(format) => write!(f, "cdx::{}", format),
        }
    }
}

/// CdxFormat is the document encoding format for the CycloneDx [Spec].
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
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

/// The entity that provided or generated the Sbom.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
