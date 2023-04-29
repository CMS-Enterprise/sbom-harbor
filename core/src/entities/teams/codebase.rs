use serde::{Deserialize, Serialize};

/// A Codebase is a named entity that contains information about
/// a code base such as the language the software is developed in
/// and the tooling use to build the code.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Codebase {
    /// The unique identifier for the codebase.
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
