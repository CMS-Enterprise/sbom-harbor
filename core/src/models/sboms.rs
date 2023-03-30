use serde::{Deserialize, Serialize};
use platform::mongodb::{MongoDocument, mongo_doc};

/// A [Package] is a item for which an SBOM can be generated. It serves as an aggregate root for all
/// version of an SBOM, and as a way of cross-referencing SBOMs across disparate systems.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Package {
    /// The unique identifier for the Package.
    pub id: String,
    /// The name of the package.
    pub name: String,
    /// The package URL of the package.
    pub purl: Option<String>,
    /// Cross-references from the package to model type or external systems.
    pub xref: PackageXRef,
    /// Log of all SBOMs uploaded for the package.
    pub sboms: Vec<Sbom>,
}

/// An SBOM is a snapshot inventory of the components that make up a piece of software at a moment in time.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sbom {
    /// The unique identifier for the Sbom.
    pub id: String,
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

// TODO: This should really be a HashMap<&str, HashMap<&str, &str> to allow dynamism.
// TODO: I'm leaving these as strong types during the modeling phase to make it easier to collaborate.
/// PackageXRef contains the metadata used to cross-reference an SBOM to another system or subsystem.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PackageXRef {
    /// FISMA cross-references.
    pub fisma: Option<FismaXRef>,
    /// Codebase cross-references.
    pub codebase: Option<CodebaseXRef>,
    /// Product cross-references.
    pub product: Option<ProductXRef>,
    /// Snyk cross-references.
    pub snyk: Option<SnykXRef>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FismaXRef {
    pub fisma_id: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodebaseXRef {
    pub team_id: Option<String>,
    pub project_id: Option<String>,
    pub codebase_id: Option<String>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductXRef {
    pub vendor_id: Option<String>,
    pub product_id: Option<String>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnykXRef {
    pub org_id: Option<String>,
    pub team_id: Option<String>,
    pub project_id: Option<String>,
}
