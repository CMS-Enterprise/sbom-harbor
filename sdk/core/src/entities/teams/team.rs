use crate::entities::teams::{Repository, Token};
use crate::entities::vendors::Product;
use crate::entities::xrefs::Xref;
use platform::auth::User;
use serde::{Deserialize, Serialize};

///  A Team is a named entity that can contain 2 child types:
/// - [Repository]
/// - [Token]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    /// The unique identifier for the Team.
    pub id: String,
    /// The name of the team.
    pub name: String,
    /// Projects that are owned by the Team.
    #[serde(default = "Vec::new")]
    pub repositories: Vec<Repository>,
    /// Tokens associated with the Team.
    #[serde(default = "Vec::new")]
    pub tokens: Vec<Token>,
    /// A map of cross-references to internal and external systems.
    pub xrefs: Vec<Xref>,
    /// [User] instances that represent users that are members of the Team. Hydrated at
    /// runtime from auth configuration.
    #[serde(skip)]
    pub members: Option<Vec<User>>,
    /// [Product] instances that represent products that are managed by members of the Team.
    /// Hydrated at runtime from auth configuration.
    #[serde(skip)]
    pub products: Option<Vec<Product>>,
}

impl Team {
    /// Constructor function for creating new [Team] instances.
    pub fn new(name: String) -> Self {
        Self {
            id: "".to_string(),
            name,
            repositories: Default::default(),
            tokens: Default::default(),
            xrefs: vec![],
            members: None,
            products: None,
        }
    }

    /// Add a repository to the repositories Vector.
    pub fn repositories(&mut self, project: Repository) -> &Self {
        self.repositories.push(project);
        self
    }

    /// Add a token to the tokens Vector.
    pub fn tokens(&mut self, token: Token) -> &Self {
        self.tokens.push(token);
        self
    }

    /// Determines if the specified repository is owned by a team instance.
    pub fn owns_repository(&self, repository_id: String) -> bool {
        self.repositories.iter().any(|p| p.id == repository_id)
    }
}
