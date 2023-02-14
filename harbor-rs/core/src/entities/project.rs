use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{Codebase, Discriminator, Team};

/// A Project is a named entity that can contain 1 child type:
/// - [Codebase]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Project {
    /// The DynamoDB partition key for the item entry.
    #[serde(rename = "TeamId")]
    pub partition_key: String,
    #[serde(rename = "EntityKey")]
    /// The DynamoDB sort key for the item entry.
    pub sort_key: String,
    /// The id of the Team that owns the Project.
    #[serde(rename = "parentId")]
    pub parent_id: String,

    /// The unique identifier for the Project.
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
    pub fn new(parent: &Team, name: String, fisma: Option<String>) -> Self {
        let id = Uuid::new_v4().to_string();
        Self {
            partition_key: parent.partition_key.clone(),
            sort_key: Discriminator::Project.to_sort_key(&id).unwrap(),
            parent_id: parent.id.clone(),
            id,
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
