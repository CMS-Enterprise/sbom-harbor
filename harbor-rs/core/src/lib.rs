extern crate core;

/// The Models module contains the API types defined by the OpenAPI specification for this API.
pub mod models;

/// The config module contains functions that are used to retrieve runtime configuration.
pub mod config;

/// The Entities module contains representations of the state that is managed
/// by the Harbor HTTP API as well as associated domain logic relevant to managing
/// entity relationships.
///
/// Entities are the things that the Service Layer manages. Some entities, on the server side,
/// may be persisted as entries in a database and therefore contain additional fields related to
/// database structural concerns. Some entities are materialized at runtime for the only for the
/// purpose of executing business logic, and are never serialized or persisted.
///
/// An Entity can be a standalone thing that represents and manages only itself, or it
/// can be an Aggregate Root. An Aggregate Root is an Entity that represents and manages itself,
/// as well as any subordinate entities that cannot exist without the root. For example, Projects
/// cannot exist outside a Team context because without a Team, a Project cannot be queried or
/// used, so it has no meaning. In this case, Team is the Aggregate Root of Project. The domain
/// model requires you to create and manage Projects through its associated Team Aggregate Root.
///
/// Entities can be both an Aggregate Root and a subordinate entity. For example, while Projects
/// are subordinate to Teams, they are the Aggregate Root of Codebases. A Codebase has no meaning
/// without a Project, so likewise the domain model requires you to create or manage Codebases
/// through its associated Project Aggregate Root.
///
/// Example
/// ```rust
/// use chrono::{DateTime, Utc};
/// use serde::{Deserialize, Serialize};
/// use harbor_core::Error;
///
///#[derive(Clone, Debug, Deserialize, Serialize)]
/// pub struct Token {
///     pub id: String,
///     pub name: String,
///     pub token: String,
///     pub enabled: bool,
///     pub expires: String,
/// }
///
/// impl Token {
///     pub fn expired(&self) -> Result<bool, Error> {
///         if self.expires.is_empty() {
///             return Ok(false);
///         }
///
///         match DateTime::parse_from_rfc3339(&self.expires) {
///             Ok(expiry) => Ok(Utc::now() >= expiry),
///             Err(err) => Err(Error::Runtime(format!("error parsing token expires: {}", err.to_string()))),
///         }
///     }
/// }
/// ```
pub mod entities;

/// The Services module is responsible for
pub mod services;



pub mod errors;
pub use errors::Error;

pub mod auth;

mod migrations;
