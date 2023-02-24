use serde::{Deserialize, Serialize};
use crate::entities::Codebase;
use crate::models;

/// A Project is a named entity that can contain 1 child type:
/// - [Codebase]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Project {
    /// The unique identifier for the Project.
    #[serde(rename = "_id")]
    pub id: String,

    /// The name of the project.
    pub name: String,

    /// The FISMA ID for the project.
    pub fisma: String,

    /// Codebases owned by the project.
    #[serde(default = "Vec::new")]
    pub codebases: Vec<Codebase>,
}

impl Project {
    pub fn new(name: String, fisma: Option<String>) -> Self {
        Self {
            id: "".to_string(),
            name,
            fisma: fisma.unwrap_or("".to_string()),
            codebases: vec![],
        }
    }

    pub fn codebases(&mut self, codebase: Codebase) -> &Self {
        self.codebases.push(codebase);
        self
    }
}

impl From<models::Project> for Project {
    fn from(value: models::Project) -> Self {
        Self {
            id: value.id,
            name: value.name,
            fisma: value.fisma,
            codebases: value.codebases
                .into_iter()
                .map(|c|c.into())
                .collect(),
        }
    }
}
