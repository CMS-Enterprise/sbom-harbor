use harbcore::entities::teams::{BuildTarget, Repository};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::Error;

/// Validatable insert type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct RepositoryInsert {
    /// The name of the project.
    pub name: Option<String>,

    /// The unique identifier for the team that manages the repository.
    pub team_id: Option<String>,

    /// The URL from which the Repository can be cloned.
    pub clone_url: Option<String>,

    /// BuildTargets contained within the repository.
    pub build_targets: Option<HashMap<String, BuildTarget>>,
}

impl RepositoryInsert {
    /// Validates insert type and converts to entity.
    #[allow(dead_code)]
    pub fn to_entity(&self) -> Result<Repository, Error> {
        let name = match &self.name {
            None => {
                return Err(Error::InvalidParameters("name required".to_string()));
            }
            Some(name) => name.clone(),
        };

        let team_id = match &self.team_id {
            None => {
                return Err(Error::InvalidParameters("team id required".to_string()));
            }
            Some(team_id) => team_id.clone(),
        };

        Repository::new(
            name,
            team_id,
            self.clone_url.clone(),
            self.build_targets.clone(),
        )
        .map_err(|e| Error::InvalidParameters(e.to_string()))
    }
}
