use serde::{Deserialize, Serialize};
use platform::mongodb::{MongoDocument, mongo_doc};

/// An SBOM is a snapshot inventory of the components that make up a piece of software at a moment in time.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sbom {
    /// The unique identifier for the Sbom.
    pub id: String,
    /// The Package URL for the package that the Sbom targets.
    pub purl: String,
    // TODO: Review with team. This is naive. Version varies widely based on a lot of factors.
    // This might be useful to us as humans, but might also be a really confusing name since
    // version has meaning that is context sensitive.
    /// The version of the Sbom. Forward-only incrementing counter.
    pub version: u32,
    /// The spec the Sbom conforms to.
    pub spec: Spec,
    /// The system or tool that produced the Sbom.
    pub source: Source,
    /// The unix timestamp for when the Sbom was received by Harbor.
    pub timestamp: String,
}

/// A Spec is the SBOM specification to which the SBOM conforms.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Spec {
    /// SBOM is a CycloneDx document.
    CycloneDx(CycloneDxFormat),
    /// SBOM is an Spdx document.
    Spdx(SpdxFormat),
}

/// CycloneDxFormat is the document encoding format for the CycloneDx [Spec].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CycloneDxFormat {
    /// Document is a JSON document.
    Json,
    /// Document is an XML document.
    Xml,
}

/// SpdxFormat is the document encoding format for the Spdx [Spec].
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

/// The system or tool that produced the Sbom.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Source {
    /// SBOM produced by Harbor using the specified internal provider (e.g. GitHub, Snyk)
    Harbor(String),
    /// SBOM provided by the vendor.
    Vendor,
}
