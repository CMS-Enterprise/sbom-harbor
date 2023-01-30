use core::default::Default;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::Error;
use crate::models::{Codebase, Member, Project, Team, Token};

impl Team {
    /// Constructor function for creating new team instances.
    pub fn new(name: String) -> Self {
        Self {
            id: "".to_string(),
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

impl Project {
    pub fn new(name: String, fisma: Option<String>) -> Self {
        Self {
            id: "".to_string(),
            name,
            fisma: fisma.unwrap_or("".to_string()),
            codebases: vec![],
        }
    }

    pub fn codebases(&mut self, codebase: Codebase) -> &Self {
        self.codebases.push(codebase);
        self
    }
}

impl Token {
    pub fn new(name: String, expires: String, enabled: Option<bool>) -> Self {
        Self {
            id: "".to_string(),
            name,
            token: Uuid::new_v4().to_string(),
            enabled: enabled.unwrap_or(false),
            expires,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn expired(&self) -> Result<bool, Error> {
        if self.expires.is_empty() {
            return Ok(false);
        }

        match DateTime::parse_from_rfc3339(&self.expires) {
            Ok(expiry) => Ok(Utc::now() >= expiry),
            Err(err) => Err(Error::Format(format!("error parsing token expires: {}", err.to_string()))),
        }
    }
}
