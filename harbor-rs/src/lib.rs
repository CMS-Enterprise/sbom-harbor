#![warn(missing_docs)]
//! Harbor SDK for Rust.
//!
//! Harbor-rs is the SDK for interacting with Harbor. Rust provides several advantages over other
//! languages including:
//! - Memory safety
//! - Strong typing
//! - Speed of execution

/// The Client module provides programmatic access to the Harbor HTTP API.
///
/// Creating a client instance requires users to authenticate with a Harbor instance using a
/// the CloudFront URL and a valid Harbor account. Once authenticated, the client will maintain
/// a reference to a JWT token that allows users to perform API operations like querying entities
/// or uploading SBOMs.
///
/// Example
/// ```rust
/// use std::env;
/// use anyhow::Result;
/// use harbor::client::Client;
/// pub async fn get_client(cloud_front_domain: String) -> Result<Client> {
///     let username = env::var("ADMIN_USERNAME").unwrap_or(String::from(""));
///     let password = env::var("ADMIN_PASSWORD").unwrap_or(String::from(""));
///     Client::new(cloud_front_domain, username, password).await
/// }
/// ```
pub mod client;

/// The Entities module contains representations of the state that is managed
/// by the Harbor HTTP API.
///
/// Entities on the server side, are persisted as entries in DynamoDB and contain
/// additional fields related to persistence requirements relevant to DynamoDB internals.
/// Their representations in this crate exclude the DynamoDB specific fields and
/// can instead be thought of as hybrid domain objects that provide both utility
/// functions, like filters and constructors, and also define the wire protocol.
///
/// Example
/// ```rust
/// use anyhow::{bail, Result};
/// use chrono::{DateTime, Utc};
/// use serde::{Deserialize, Serialize};
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
///     pub fn expired(&self) -> Result<bool> {
///         if self.expires.is_empty() {
///             return Ok(false);
///         }
///
///         match DateTime::parse_from_rfc3339(&self.expires) {
///             Ok(expiry) => Ok(Utc::now() >= expiry),
///             Err(err) => bail!(err),
///         }
///     }
/// }
/// ```
pub mod entities;

/// The HTTP module contains generic abstractions over the Hyper SDK.
///
/// Hyper is a general purpose HTTP library for Rust. As a general purpose
/// library it focuses on supporting the core use case in a un-opinionated way.
/// This tradeoff comes at the expense of developer ergonomics. There are numerous
/// libraries built on top of Hyper that aim to improve this situation, but
/// result in a much larger crate size and dependency tree. The HTTP module is
/// designed to improve developer ergonomics and reduce boilerplate without
/// introducing new dependencies.
///
/// Example
/// ```rust
/// use anyhow::Result;
/// use harbor::entities::Team;
/// use harbor::http;
/// pub async fn delete_team(token: &str, id: String) -> Result<()> {
///     let url = delete_team_url(id);
///     http::delete(url.as_str(), token, None::<Team>).await?;
///     Ok(())
/// }
/// ```
pub mod http;

/// The Importer module contains the Lambda handler implementation and supporting
/// domain logic related to syncing Harbor with existing source control systems.
pub mod importer;

/// The Pilot module contains the Lambda handler implementation and supporting
/// domain logic for the Harbor Pilot API.
pub mod pilot;
