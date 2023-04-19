use serde::{Deserialize, Serialize, Serializer};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use tracing::log::debug;

use crate::entities::cyclonedx::Bom;
use crate::entities::packages::{PackageCdx, Purl};
use crate::entities::sboms::{SbomProviderKind, Scan};
use crate::entities::xrefs::{Xref, XrefKind};
use crate::services::snyk::SnykRef;
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

    /// The results of each vulnerability [Scan].
    pub scans: Vec<Scan>,

    // TODO: Define a new interface for types that can only have one Xref per kind.
    /// A map of cross-references to internal and external systems.
    pub xrefs: Option<HashMap<XrefKind, Xref>>,

    /// In-memory struct representation of SBOM based on CycloneDx JSON spec.
    #[serde(skip_serializing)]
    pub(crate) bom: Option<Bom>,
}

impl Sbom {
    /// Factory method to create new instance of type from a CycloneDx model.
    pub fn from_raw_cdx(
        raw: &str,
        format: CdxFormat,
        source: Source,
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

        let purl = match Purl::decode(purl.as_str()) {
            Ok(purl) => purl,
            Err(e) => {
                return Err(Error::Entity(format!("purl::decode::{}", e)));
            }
        };

        let provider = match source.clone() {
            Source::Harbor(provider) => Some(provider),
            Source::Vendor(_) => None,
        };

        let sbom = Sbom {
            id: "".to_string(),
            purl: Some(purl),
            instance: 1,
            spec: Spec::Cdx(format),
            source,
            provider,
            timestamp: platform::time::timestamp()?,
            checksum_sha256: "".to_string(),
            scans: vec![],
            xrefs,
            bom: Some(bom),
        };

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

    pub fn scans(&mut self, mut scan: Scan) {
        scan.iteration = match self
            .scans
            .iter()
            .filter(|scan| scan.provider == provider)
            .max_by_key(|scan| scan.iteration)
        {
            None => 1,
            Some(scan) => scan.iteration + 1,
        };

        self.scans.push(scan);
    }

    pub fn snyk_ref(&self) -> Result<HashMap<String, String>, Error> {
        let xrefs = match &self.xrefs {
            None => {
                return Err(Error::Entity("sbom_xrefs_none".to_string()));
            }
            Some(snyk_refs) => snyk_refs,
        };

        let snyk_ref = match xrefs.get(&XrefKind::External(SNYK_DISCRIMINATOR.to_string())) {
            None => {
                return Err(Error::Entity("sbom_xrefs_snyk_none".to_string()));
            }
            Some(xref) => xref.clone(),
        };

        Ok(snyk_ref)
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
