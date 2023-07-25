use crate::entities::teams::BuildTarget;
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// A Repository is a named entity that can contain 1 child type:
/// - [BuildTarget]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Repository {
    /// The unique identifier for the repository.
    pub id: String,

    /// The name of the project.
    pub name: String,

    /// The unique identifier for the team that manages the repository.
    pub team_id: String,

    /// The URL from which the Repository can be cloned.
    pub clone_url: Option<String>,

    /// BuildTargets contained within the repository.
    pub build_targets: Option<HashMap<String, BuildTarget>>,
}

impl Repository {
    /// Factory method to create new instance of type.
    pub fn new(
        name: String,
        team_id: String,
        clone_url: Option<String>,
        build_targets: Option<HashMap<String, BuildTarget>>,
    ) -> Result<Repository, Error> {
        if name.is_empty() {
            return Err(Error::Entity("name required".to_string()));
        }

        if name.len() < 2 {
            return Err(Error::Entity(
                "name must be at least 2 characters in length".to_string(),
            ));
        }

        if team_id.is_empty() {
            return Err(Error::Entity("team id required".to_string()));
        }

        if team_id.len() < 2 {
            return Err(Error::Entity(
                "team id must be at least 2 characters in length".to_string(),
            ));
        }

        Ok(Repository {
            id: "".to_string(),
            name,
            team_id,
            clone_url,
            build_targets,
        })
    }

    /// Add a [BuildTarget] to the codebases Vector.
    pub fn build_targets(&mut self, _build_target: BuildTarget) -> &Self {
        // TODO:
        self
    }
}
