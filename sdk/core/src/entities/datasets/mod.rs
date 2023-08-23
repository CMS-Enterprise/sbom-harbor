use platform::mongo_doc;
use platform::persistence::mongodb::MongoDocument;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Common Platform Enumeration (CPE) is a structured naming scheme for information
/// technology systems, software, and packages. Based upon the generic syntax for
/// Uniform Resource Identifiers (URI), CPE includes a formal name format, a method
/// for checking names against a system, and a description format for binding text
/// and tests to a name.
pub struct Cpe {
    /// Place to store the original string to return later
    original_cpe_string: String,
    /// "cpe" string literal
    pub cpe: String,
    /// The version of the CPE definition. The latest CPE definition version is 2.3.
    pub cpe_version: String,
    /// May have 1 of 3 values:
    /// 1. a for Applications
    /// 2. h for Hardware
    /// 3. o for Operating Systems
    pub part: String,
    /// Values for this attribute SHOULD describe or identify the person or organization
    /// that manufactured or created the product. Values for this attribute SHOULD be
    /// selected from an attribute-specific valid-values list, which MAY be defined by
    /// other specifications that utilize this specification. Any character string meeting
    /// the requirements for WFNs (cf. 5.3.2) MAY be specified as the value of the attribute
    pub vendor: String,
    /// The name of the system/package/component. product and vendor are sometimes identical.
    /// It can not contain spaces, slashes, or most special characters. An underscore should
    /// be used in place of whitespace characters.
    pub product: String,
    /// The version of the system/package/component.
    pub version: String,
    /// This is used for update or service pack information. Sometimes referred to as
    /// "point releases" or minor versions. The technical difference between version and update
    /// will be different for certain vendors and products. Common examples include beta, update4,
    /// SP1, and ga (for General Availability), but it is most often left blank.
    pub update: String,
    /// A further granularity describing the build of the system/package/component, beyond version.
    pub edition: String,
    /// A valid language tag as defined by IETF RFC 4646 entitled "Tags for Identifying Languages".
    /// Examples include: en-us for US English, and zh-tw for Taiwanese Mandarin.
    pub language: String,
    /// The Software Edition Component is used to define specific target software architectures
    /// that need to be named
    pub sw_edition: String,
    /// The operating system the software was built to run on
    pub target_sw: String,
    /// The hardware the software was built to run on
    pub target_hw: String,
    /// Ancillary information
    pub other: String,
}

impl Cpe {
    /// Conventional Constructor
    pub fn new(cpe: String) -> Self {
        let cpe_parts = cpe.split(':').collect::<Vec<&str>>();
        Self {
            original_cpe_string: cpe.clone(),
            cpe: cpe_parts[0].to_string(),
            cpe_version: cpe_parts[1].to_string(),
            part: cpe_parts[2].to_string(),
            vendor: cpe_parts[3].to_string(),
            product: cpe_parts[4].to_string(),
            version: cpe_parts[5].to_string(),
            update: cpe_parts[6].to_string(),
            edition: cpe_parts[7].to_string(),
            language: cpe_parts[8].to_string(),
            sw_edition: cpe_parts[9].to_string(),
            target_sw: cpe_parts[10].to_string(),
            target_hw: cpe_parts[11].to_string(),
            other: cpe_parts[12].to_string(),
        }
    }

    /// Function to return the original string
    pub fn get_string(&self) -> String {
        self.original_cpe_string.clone()
    }
}

/// Struct used to create an entry in the data set collection.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Purl2Cpes {
    /// Storing the purl as the id because the purls must be unique
    pub id: String,
    /// All the associated cpes to the given purl
    pub cpes: Vec<String>,
}
mongo_doc!(Purl2Cpes);

/// Inner structure to hold an object with an id and the purl so we can
/// update mongo later using the regular MongoDB persistence API
#[derive(Deserialize, Debug, Clone)]
pub struct PurlPlusId {
    pub(crate) id: String,
    pub(crate) purl: String,
}

impl Purl2Cpes {
    /// Conventional constructor
    pub fn new(purl: String, cpes: Vec<String>) -> Self {
        Self { id: purl, cpes }
    }
}

/// Tiny struct to hold a deserialized vector of purls
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PurlsContainer {
    /// A vector of Purls as defined by
    pub purls: Vec<String>,
}

/// Tiny struct to hold a deserialized vector of cpes
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CpesContainer {
    /// a vector of Cpes that are associated to a given Purl
    pub cpes: Vec<String>,
}

/// Enum to specify the Construction kind.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ConstructionProviderKind {
    /// Data set kind for building a Purl -> Cpe.
    Purl2Cpe,
}

impl Display for ConstructionProviderKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstructionProviderKind::Purl2Cpe => write!(f, "purl-2-cpe"),
        }
    }
}
