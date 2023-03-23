use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnykData {
    pub orgs: Vec<Option<Org>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Org {
    pub id: Option<String>,
    pub name: Option<String>,
    pub projects: Option<ProjectList>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectList {
    pub projects: Vec<Option<ProjectDetails>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDetails {
    pub id: Option<String>,
    pub name: Option<String>,
    pub origin: Option<String>,
    pub r#type: Option<String>,
    pub sbom_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sbom {
    #[serde(flatten)]
    pub inner: HashMap<String, Value>,
}

impl Org {
    pub fn add_project(&mut self, projects: Option<ProjectList>) {
        self.projects = projects;
    }
}