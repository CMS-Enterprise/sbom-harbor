use crate::entities::teams::{Repository, Token};
use crate::entities::vendors::Product;
use crate::Error;
use platform::auth::User;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

///  A Team is a named entity that can contain 2 child types:
/// - [Repository]
/// - [Token]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Team {
    /// The unique identifier for the Team.
    pub id: String,

    /// The name of the team.
    pub name: String,

    /// Tokens associated with the Team.
    pub tokens: Option<HashMap<String, Token>>,

    /// [User] instances that represent users that are members of the Team.
    pub members: Option<HashMap<String, User>>,

    /// [Product] instances that are managed by members of the Team.
    pub products: Option<HashMap<String, Product>>,

    /// [Repository] instances that are owned by the Team.
    pub repositories: Option<HashMap<String, Repository>>,
}

impl Team {
    /// Constructor function for creating new [Team] instances.
    pub fn new(name: String, members: Option<HashMap<String, User>>) -> Result<Team, Error> {
        if name.is_empty() {
            return Err(Error::Entity("name required".to_string()));
        }

        if name.len() < 2 {
            return Err(Error::Entity(
                "name must be at least 2 characters in length".to_string(),
            ));
        }

        Ok(Team {
            id: "".to_string(),
            name,
            repositories: None,
            tokens: None,
            members,
            products: None,
        })
    }

    /// Add a repository to the repositories Vector.
    pub fn repositories(&mut self, _project: Repository) -> &Self {
        // TODO:
        self
    }

    /// Add a token to the tokens Vector.
    pub fn tokens(&mut self, _token: Token) -> &Self {
        // TODO:
        self
    }

    /// Determines if the specified repository is owned by a team instance.
    pub fn owns_repository(&self, repository_id: String) -> bool {
        match &self.repositories {
            None => false,
            Some(repositories) => repositories.contains_key(repository_id.as_str()),
        }
    }
}

/// Validatable insert type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct TeamInsert {
    /// The name of the team.
    pub name: Option<String>,

    /// [User] instances that represent users that are members of the Team.
    pub members: Option<HashMap<String, User>>,
}

impl TeamInsert {
    /// Validates insert type and converts to entity.
    #[allow(dead_code)]
    pub fn to_entity(&self) -> Result<Team, Error> {
        let name = match &self.name {
            None => {
                return Err(Error::Entity("name required".to_string()));
            }
            Some(name) => name.clone(),
        };

        Team::new(name, self.members.clone())
    }
}
