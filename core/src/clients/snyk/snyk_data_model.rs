use serde::{Serialize, Deserialize};

//NOTE: Is this the correct way to set up these structs? Feels bad having all of the structs being public

#[derive(Debug, Serialize, Deserialize)]
pub (crate) struct SnykData {
    pub orgs: Vec<Org>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Org {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectJson {
    #[serde(default = "project_list_default")]
    pub projects: Vec<ProjectDetails>,
    pub org: Org,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDetails {
    pub id: String,
    pub name: String,
    pub origin: String,
    pub r#type: String,
    pub browseUrl: String,
}

fn project_list_default() -> Vec<ProjectDetails> {
    return Vec::new();
}