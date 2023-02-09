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
    #[serde(default = "Vec::new")]
    pub members: Vec<Member>,
    /// Projects that are owned by the Team.
    #[serde(default = "Vec::new")]
    pub projects: Vec<Project>,
    /// Tokens associated with the Team.
<<<<<<< HEAD:harbor-rs/core/src/models.rs
    #[serde(default = "Vec::new")]
=======
>>>>>>> 51e7f1f (feat: now working on json):harbor-rs/src/entities_old.rs
    pub tokens: Vec<Token>,
}

/// A Member is an entity representing a Harbor User who can manage projects, codebases, tokens, and SBOMs.
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
    #[serde(default = "Vec::new")]
    pub codebases: Vec<Codebase>,
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
    #[serde(rename = "cloneUrl")]
    /// The URL from which the codebase can be cloned.
    pub clone_url: String,
}

/// A Token is an entity that represents a string used to authorize sending
/// SBOMs into the system.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    /// The unique identifier for the Token.
    pub id: String,
    /// The name of the token.
    pub name: String,
    /// The secret token value.
    pub token: String,
    /// Flag indicating whether the token is enabled.
    pub enabled: bool,
    /// The string representation of the expiration date of the token.
    pub expires: String,
}
