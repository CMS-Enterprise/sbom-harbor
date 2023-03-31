use serde::{Serialize, Deserialize};

//NOTE: Is this the correct way to set up these structs? Feels bad having all of the structs being public

#[derive(Debug, Serialize, Deserialize)]
pub struct SnykData {
    pub orgs: Vec<Option<Org>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Org {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectJson {
    #[serde(default = "project_list_default")]
    pub projects: Vec<ProjectDetails>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDetails {
    #[serde(default = "derived_field")]
    pub org_id: String,
    #[serde(default = "derived_field")]
    pub org_name: String,
    pub id: String,
    pub name: String,
    pub origin: String,
    pub r#type: String,
    pub browseUrl: String,
    #[serde(default = "derived_field")]
    pub sbom_url: String,
}

fn project_list_default() -> Vec<ProjectDetails> {
    return Vec::new();
}
fn derived_field() -> String {
    return format!("");
}