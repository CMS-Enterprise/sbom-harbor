#![warn(missing_docs)]
//! The [Core] crate contains the [Models], [Services], and domain logic for Harbor.
//! Other crates such as the API or CLI are thin wrappers over the [Core] crate that handle
//! translating user input from the wire protocol or stdin to native types and then invoking
//! the corresponding [Service] type found in this crate.

/// The [Entities] module extends the [Models] and contains domain logic relevant to managing
/// entity relationships.
///
/// Entities are the things that the Service Layer manages. Some entities, on the server side,
/// may be persisted as entries in a database. Some entities are materialized at runtime only for the
/// purpose of executing business logic, and are never serialized or persisted.
///
/// An Entity can be a standalone thing that represents and manages only itself, or it
/// can be an Aggregate Root. An Aggregate Root is an Entity that represents and manages itself,
/// as well as any subordinate entities that cannot exist without the root. For example, a [Project]
/// cannot exist outside a [Team] context because without a [Team], a [Project] cannot be queried or
/// used, so it has no meaning. In this case, [Team] is the Aggregate Root of [Project]. The domain
/// model requires you to create and manage a [Project] through its associated [Team] Aggregate Root.
///
/// Entities can be both an Aggregate Root and a subordinate entity. For example, while [Projects]
/// are subordinate to [Teams], they are the Aggregate Root of [Codebases]. A [Codebase] has no meaning
/// without a [Project], so likewise the domain model requires you to create or manage a [Codebase]
/// through its [Project] Aggregate Root.
///
/// Example
/// ```rust
/// use chrono::{DateTime, Utc};
/// use serde::{Deserialize, Serialize};
/// use harbcore::Error;
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

/// The [Services] module is responsible for exposing domain logic for the Harbor runtime.
pub mod services;

/// Errors exposed by this crate.
pub mod errors;
/// The Error type for this crate.
pub use errors::Error;

/// Authorization logic and types for this crate.
pub mod auth;

/// DB Migrations for the Harbor database.
mod migrations;

/// Runtime configuration helpers.
pub mod config;


/// Third-Party client APIs
pub mod clients;