use serde::{Deserialize, Serialize};
use platform::mongodb::{MongoDocument, mongo_doc};


/// A Target is a item for which an SBOM can be generated. It serves as an aggregate root for all
/// version of an SBOM, and as a way of cross-referencing SBOMs across disparate systems.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Target {
    /// The unique identifier for the Package.
    pub id: String,
    /// The name of the package.
    pub name: String,
    /// The package URL of the package.
    pub purl: Option<String>,
    /// Cross-references from the package to model type or external systems.
    pub xref: TargetXRef,
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
    CycloneDx(CycloneDxFormat),
    Spdx(SpdxFormat),
}

/// CycloneDxFormat is the document encoding format for the CycloneDx [Spec].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CycloneDxFormat {
    Json,
    Xml,
}

/// SpdxFormat is the document encoding format for the Spdx [Spec].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SpdxFormat {
    Json,
    Rdf,
    TagValue,
    Spreadsheet,
    Yaml,
}

/// The system or tool that produced the Sbom.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Source {
    Harbor(String),
    Syft,
    Snyk,
    Vendor,
}

// TODO: This should really be a HashMap<&str, HashMap<&str, &str> to allow dynamism.
// TODO: I'm leaving these as strong types during the modeling phase to make it easier to collaborate.
/// TargetXRef contains the metadata used to cross-reference an SBOM to another system or subsystem.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TargetXRef {
    pub fisma: Option<FismaXRef>,
    pub codebase: Option<CodebaseXRef>,
    pub product: Option<ProductXRef>,
    pub snyk: Option<SnykXRef>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FismaXRef {
    pub fisma_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodebaseXRef {
    pub team_id: Option<String>,
    pub project_id: Option<String>,
    pub codebase_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductXRef {
    pub vendor_id: Option<String>,
    pub product_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnykXRef {
    pub org_id: Option<String>,
    pub project_id: Option<String>,
}
