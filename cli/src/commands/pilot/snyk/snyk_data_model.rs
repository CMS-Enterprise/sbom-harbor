use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;

//NOTE: Is this the correct way to set up these structs? Feels bad having all of the structs being public

#[derive(Debug, Serialize, Deserialize)]
pub struct SnykData {
    pub orgs: Vec<Option<Org>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Org {
    pub id: Option<String>,
    pub name: Option<String>,
    pub projects: Option<ProjectJson>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectJson {
    #[serde(default = "project_list_default")]
    pub projects: Vec<ProjectDetails>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDetails {
    pub id: String,
    pub name: String,
    pub origin: String,
    pub r#type: String,
    #[serde(default = "sbom_default")]
    pub sbom_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sbom {
    #[serde(flatten)]
    pub inner: HashMap<String, Value>,
}

impl Org {
    pub fn add_project(&mut self, projects: Option<ProjectJson>) {
        self.projects = projects;
    }
}

fn project_list_default() -> Vec<ProjectDetails> {
    return Vec::new();
}
fn sbom_default() -> String {
    return format!("");
}