use core::default::Default;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{Discriminator, Member};
use crate::entities::Project;
use crate::entities::Token;

///  A Team is a named entity that can contain 3 child types:
/// - [Project]
/// - [Member]
/// - [Token]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Team {
    // This first block of fields are required by the schema.
    // Each type will have to implement the HarborTeam trait
    // to reference these fields directly since traits
    // cannot reference instance members.
    #[serde(rename = "TeamId")]
    pub partition_key: String,
    #[serde(rename = "EntityKey")]
    pub sort_key: String,
    /// The id of the Team. Teams are the aggregate root, so they are their own parent.
    #[serde(rename = "parentId")]
    pub parent_id: String,

    /// The unique identifier for the Team.
    pub id: String,
    /// The name of the team.
    pub name: String,
    /// Members of the Team.
    #[serde(default = "Vec::new")]
    pub members: Vec<Member>,
    /// Projects that are owned by the Team.
    #[serde(default = "Vec::new")]
    pub projects: Vec<Project>,
    /// Tokens associated with the Team.
    #[serde(default = "Vec::new")]
    pub tokens: Vec<Token>,
}

impl Team {
    /// Constructor function for creating new team instances.
    pub fn new(name: String) -> Self {
        let id = Uuid::new_v4().to_string();

        Self {
            partition_key: id.clone(),
            sort_key: Discriminator::Team.to_sort_key(&id).unwrap(),
            parent_id: id.clone(),
            id,
            name,
            members: Default::default(),
            projects: Default::default(),
            tokens: Default::default(),
        }
    }

    pub fn members(&mut self, member: Member) -> &Self {
        self.members.push(member);
        self
    }

    pub fn projects(&mut self, project: Project) -> &Self {
        self.projects.push(project);
        self
    }

    pub fn tokens(&mut self, token: Token) -> &Self {
        self.tokens.push(token);
        self
    }

    /// Determines if the specified codebase id is owned by a given project.
    pub fn owns_project_and_codebase(&self, project_id: String, codebase_id: String) -> bool {
        self.projects
            .iter()
            .find(|p| p.id == *project_id)
            .and_then(|p: &Project| p.codebases.iter().find(|c| c.id == *codebase_id))
            .is_some()
    }

    #[allow(dead_code)]
    pub(crate) fn get_sbom_token(&self) -> Option<&str> {
        let sbom_token = self
            .tokens
            .iter()
            .filter(|t| t.enabled && t.expired().unwrap_or(true))
            .map(|t| &*t.token)
            .next();

        sbom_token
    }
}
