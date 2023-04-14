use serde::{Serialize, Deserialize};

//NOTE: Is this the correct way to set up these structs? Feels bad having all of the structs being public

#[derive(Debug, Serialize, Deserialize)]
pub (crate) struct SnykData {
    pub orgs: Vec<Org>,
}

/// This struct represents a subset of the json retrieved from Snyk when requesting a list of Orgs
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Org {
    /// Org ID
    pub id: Option<String>,
    /// Org Name
    pub name: Option<String>,
}

/// This struct represents a subset of the json retrieved from Snyk when requesting a list of projects for an Org
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectJson {
    /// The list of projects and their details for an Org
    #[serde(default = "project_list_default")]
    pub projects: Vec<ProjectDetails>,
    /// Org associated with the list of Projects
    pub org: Org,
}

/// This struct represents a subset of the json retrieved from Snyk that describes details of a particular project
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDetails {
    /// Project ID
    pub id: String,
    /// Project Name
    pub name: String,
    /// Project Origin (Source control location)
    pub origin: String,
    /// Project Type (Project's build tools or language)
    pub r#type: String,
    /// Url to the project's source control page
    #[serde(default = "browse_url_default")]
    pub browse_url: String,
}
fn browse_url_default() -> String {
    return format!("");
}
fn project_list_default() -> Vec<ProjectDetails> {
    return Vec::new();
}