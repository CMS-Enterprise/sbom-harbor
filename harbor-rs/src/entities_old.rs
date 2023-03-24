use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

///  A Team is a named entity that can contain 3 child types:
/// - [Project]
/// - [Member]
/// - [Token]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Team {
    /// The unique identifier for the Team.
    pub id: String,
    /// The name of the team.
    pub name: String,
    /// Members of the Team.
    pub members: Vec<Member>,
    /// Projects that are owned by the Team.
    pub projects: Vec<Project>,
    /// Tokens associated with the Team.
    pub(crate) tokens: Vec<Token>,
}

impl Team {
    /// Constructor funtion for creating new team instances.
    pub fn new(name: String) -> Team {
        Team {
            id: String::from(""),
            name,
            members: vec![],
            projects: vec![],
            tokens: vec![],
        }
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

/// A Member is an entity representing a Harbor User who can submit SBOMs.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Member {
    /// The unique identifier for the Member.
    pub id: String,
    /// The email address for the Member.
    pub email: String,
    // TODO: Consider roles
    /// Flag indicating whether the member is a team lead.
    #[serde(rename = "isTeamLead")]
    is_team_lead: bool,
}

/// A Project is a named entity that can contain 1 child type:
/// - [Codebase]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    /// The unique identifier for the Project.
    pub id: String,
    /// The name of the project.
    pub name: String,
    /// The FISMA ID for the project.
    pub fisma: String,
    /// Codebases owned by the project.
    pub codebases: Vec<Codebase>,
}

impl Project {
    /// Constructor function for a new Project instance. Optionally,
    /// creates codebases owned by the project.
    pub fn new(name: String, codebases: Vec<String>) -> Project {
        let codebases = codebases
            .into_iter()
            .map(move |c| Codebase {
                id: String::from(""),
                name: c,
                language: "".to_string(),
                build_tool: "".to_string(),
            })
            .collect();

        Project {
            id: String::from(""),
            name,
            fisma: "".to_string(),
            codebases,
        }
    }
}

/// A Codebase is a named entity that contains information about
/// a code base such as the language the software is developed in
/// and the tooling use to build the code.
#[derive(Clone, Debug, Deserialize, Serialize)]
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
}

/// A Token is an entity that represents a string used to authorize sending
/// SBOMs into the system.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    /// The unique identifier of the token.
    pub id: String,
    /// The name of the token.
    pub name: String,
    /// The secret token value.
    pub token: String,
    /// Flag indicating whether the token is enabled.
    pub enabled: bool,
    /// The RFC 3339 formatted expiration date of the token.
    pub expires: String,
}

impl Token {
    #[allow(dead_code)]
    pub(crate) fn expired(&self) -> Result<bool> {
        if self.expires.is_empty() {
            return Ok(false);
        }

        match DateTime::parse_from_rfc3339(&self.expires) {
            Ok(expiry) => Ok(Utc::now() >= expiry),
            Err(err) => bail!(err),
        }
    }
}
