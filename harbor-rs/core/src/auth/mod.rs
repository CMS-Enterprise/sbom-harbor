mod auth;

use std::fmt::{Display, Formatter};
pub use auth::*;

pub mod migrations;

pub enum ResourceKind {
    Any,
    Team,
    Project,
    Codebase,
    Token,
}

impl Display for ResourceKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceKind::Any => write!(f, "*"),
            ResourceKind::Team => write!(f, "team"),
            ResourceKind::Project => write!(f, "project"),
            ResourceKind::Codebase => write!(f, "codebase"),
            ResourceKind::Token => write!(f, "token"),
        }
    }
}
