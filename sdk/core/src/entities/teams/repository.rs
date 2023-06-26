use crate::entities::teams::BuildTarget;
use crate::entities::xrefs::Xref;
use serde::{Deserialize, Serialize};

/// A Repository is a named entity that can contain 1 child type:
/// - [BuildTarget]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    /// The unique identifier for the repository.
    pub id: String,
    /// The name of the project.
    pub name: String,
    /// The URL from which the Repository can be cloned.
    pub clone_url: String,
    /// BuildTargets contained within the repository.
    #[serde(default = "Vec::new")]
    pub build_targets: Vec<BuildTarget>,
    /// A map of cross-references to internal and external systems.
    pub xrefs: Vec<Xref>,
}

impl Repository {
    /// Factory method to create new instance of type.
    pub fn new(name: String, clone_url: String) -> Self {
        Self {
            id: "".to_string(),
            name,
            clone_url,
            build_targets: vec![],
            xrefs: vec![],
        }
    }

    /// Add a [BuildTarget] to the codebases Vector.
    pub fn build_targets(&mut self, build_target: BuildTarget) -> &Self {
        self.build_targets.push(build_target);
        self
    }
}
