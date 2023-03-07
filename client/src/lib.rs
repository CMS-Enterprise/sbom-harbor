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
/// use harbor::lib::Client;
/// pub async fn get_client(cloud_front_domain: String) -> Result<Client> {
///     let username = env::var("ADMIN_USERNAME").unwrap_or(String::from(""));
///     let password = env::var("ADMIN_PASSWORD").unwrap_or(String::from(""));
///     Client::new(cloud_front_domain, username, password).await
/// }
/// ```
pub mod client;
