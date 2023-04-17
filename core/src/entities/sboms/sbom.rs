use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use tracing::log::debug;

use crate::entities::cyclonedx::Bom;
use crate::entities::packages::Purl;
use crate::entities::xrefs::{Xref, XrefKind};
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

    /// The instance of the Sbom. Forward-only incrementing counter.
    pub instance: u32,

    /// The spec the Sbom conforms to.
    pub spec: Spec,

    /// The system or tool that produced the Sbom.
    pub source: Source,

    /// The unix timestamp for when the Sbom was received by Harbor.
    pub timestamp: u64,

    /// The checksum of the file when persisted to disk.
    pub checksum_sha256: Option<String>,

    /// A map of cross-references to internal and external systems.
    pub xrefs: Option<HashMap<XrefKind, Xref>>,

    /// In-memory struct representation of SBOM based on CycloneDx JSON spec.
    // #[serde(skip_serializing)]
    pub(crate) bom: Option<Bom>,
}

impl Sbom {
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

        let sbom = Sbom {
            id: "".to_string(),
            purl: Some(purl),
            instance: 1,
            spec: Spec::Cdx(format),
            source,
            timestamp: platform::time::timestamp()?,
            checksum_sha256: None,
            xrefs,
            bom: Some(bom),
        };

        Ok(sbom)
    }

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

/// The system or tool that produced the Sbom.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Source {
    /// SBOM produced by Harbor using the specified internal provider (e.g. GitHub, Snyk)
    Harbor(String),
    /// SBOM provided by the vendor.
    Vendor(String),
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Harbor(name) => write!(f, "harbor::{}", name.to_lowercase()),
            Source::Vendor(name) => write!(f, "vendor::{}", name.to_lowercase()),
        }
    }
}
