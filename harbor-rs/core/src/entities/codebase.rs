use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entities::{Discriminator, Project};

/// A Codebase is a named entity that contains information about
/// a code base such as the language the software is developed in
/// and the tooling use to build the code.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Codebase {
    /// The DynamoDB partition key for the item entry.
    #[serde(rename = "TeamId")]
    pub partition_key: String,
    #[serde(rename = "EntityKey")]
    /// The DynamoDB sort key for the item entry.
    pub sort_key: String,
    /// The id of the Project that owns the Codebase.
    #[serde(rename = "parentId")]
    pub parent_id: String,

    /// The unique identifier for the Codebase.
    pub id: String,
    /// The name of the codebase, usually the repository name.
    pub name: String,
    /// The primary programming language of the source code.
    pub language: String,
    #[serde(rename = "buildTool")]
    /// The build tool used by the codebase.
    pub build_tool: String,
    #[serde(rename = "cloneUrl")]
    /// The URL from which the codebase can be cloned.
    pub clone_url: String,
}

impl Codebase {
    pub fn new(parent: &Project,
               name: String,
               language: Option<String>,
               build_tool: Option<String>,
               clone_url: Option<String>) -> Self {
        let id = Uuid::new_v4().to_string();

        Self {
            partition_key: parent.partition_key.clone(),
            sort_key: Discriminator::Codebase.to_sort_key(&id).unwrap(),
            parent_id: parent.id.clone(),
            id,
            name,
            language: language.unwrap_or("".to_string()),
            build_tool: build_tool.unwrap_or("".to_string()),
            clone_url: clone_url.unwrap_or("".to_string()),
        }
    }
}
