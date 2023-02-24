use serde::{Deserialize, Serialize};
use crate::models;

/// A Codebase is a named entity that contains information about
/// a code base such as the language the software is developed in
/// and the tooling use to build the code.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Codebase {
    /// The unique identifier for the Codebase.
    #[serde(rename = "_id")]
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
    pub fn new(name: String,
               language: Option<String>,
               build_tool: Option<String>,
               clone_url: Option<String>) -> Self {

        Self {
            id: "".to_string(),
            name,
            language: language.unwrap_or("".to_string()),
            build_tool: build_tool.unwrap_or("".to_string()),
            clone_url: clone_url.unwrap_or("".to_string()),
        }
    }
}

impl From<models::Codebase> for Codebase {
    fn from(value: models::Codebase) -> Self {
        Self{
            id: value.id,
            name: value.name,
            language: value.language,
            build_tool: value.build_tool,
            clone_url: value.clone_url,
        }
    }
}
