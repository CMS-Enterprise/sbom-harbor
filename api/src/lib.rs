#![warn(missing_docs)]
//! The [API] crate contains all REST and RPC endpoints for the Harbor application. It handles
//! JWT verification and authorization, and the serialization of types. Handlers in this crate
//! handle only HTTP protocol concerns. They should contain no business logic, and instead invoke
//! [Services] from the [Harbcore] crate.

/// Authorization related functionality for JWT-based claims handling.
pub mod auth;

/// Request handler implementations.
pub mod controllers;

/// Error module for the crate.
mod errors;

/// Error type for the crate.
pub use errors::Error;
