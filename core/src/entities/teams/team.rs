use chrono::{DateTime, Utc};
use core::default::Default;
use uuid::Uuid;

use crate::models::teams::{Codebase, Member, Project, Team, Token};
use crate::Error;

///  A Team is a named entity that can contain 3 child types:
/// - [Project]
/// - [Member]
/// - [Token]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Team {
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
    /// Constructor function for creating new [Team] instances.
    pub fn new(name: String) -> Self {
        Self {
            id: "".to_string(),
            name,
            members: Default::default(),
            projects: Default::default(),
            tokens: Default::default(),
        }
    }

    /// Add a member to the members Vector.
    pub fn members(&mut self, member: Member) -> &Self {
        self.members.push(member);
        self
    }

    /// Add a project to the projects Vector.
    pub fn projects(&mut self, project: Project) -> &Self {
        self.projects.push(project);
        self
    }

    /// Add a token to the tokens Vector.
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
