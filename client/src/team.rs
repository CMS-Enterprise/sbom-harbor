use core::default::Default;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::errors::Error;
use crate::models::{
    Team,
    Project,
    Codebase,
    Member,
    Token,
};

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
    /// Extracts the token from model state
    ///
    pub fn get_sbom_token(&self) -> Option<&str> {
        let sbom_token = self
            .tokens
            .iter()
            .filter(|t| t.enabled && t.expired().unwrap_or(true))
            .map(|t| &*t.token)
            .next();

        sbom_token
    }
}

impl Project {

    /// Constructor function for creating new [Project] instances.
    pub fn new(name: String, fisma: Option<String>) -> Self {
        Self {
            id: "".to_string(),
            name,
            fisma: fisma.unwrap_or("".to_string()),
            codebases: vec![],
        }
    }

    /// Add a [Codebase] to the codebases Vector.
    pub fn codebases(&mut self, codebase: Codebase) -> &Self {
        self.codebases.push(codebase);
        self
    }
}

impl Token {

    /// Constructor function for creating new [Token] instances.
    pub fn new(name: String, expires: String, enabled: Option<bool>) -> Self {
        Self {
            id: "".to_string(),
            name,
            token: Uuid::new_v4().to_string(),
            enabled: enabled.unwrap_or(false),
            expires,
        }
    }

    /// Determines whether a token is expired.
    #[allow(dead_code)]
    pub fn expired(&self) -> Result<bool, Error> {
        if self.expires.is_empty() {
            return Ok(false);
        }

        match DateTime::parse_from_rfc3339(&self.expires) {
            Ok(expiry) => Ok(Utc::now() >= expiry),
            Err(e) => Err(
                Error::InvalidFormat(
                    format!("error parsing token expires: {}", e)
                )
            ),
        }
    }
}
