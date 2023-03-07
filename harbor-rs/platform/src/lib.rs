#![warn(missing_docs)]
//! The Platform crate encapsulates all functionality related to underlying platform services (e.g. databases,
//! AWS Managed services). It also includes a generic authorization module based on AWS IAM.

/// The `auth` module provides a reusable RBAC model inspired by the AWS IAM model. It was initially
/// developed to solve multi-tenant database access, but as a general purpose RBAC model, it should be
/// usable in a variety of scenarios.
pub mod auth;

/// The `cognito` module provides high level abstractions over the AWS Cognito SDK.
pub mod cognito;

/// The `errors` module provides common error types for the library.
pub mod errors;

/// The `hyper` module provides a lightweight HTTP client facade based on the `hyper` SDK.
pub mod hyper;

/// The `mongodb` module provides a `Service` and `Store` abstraction over common CRUD based operations
/// against a MongoDB or DocumentDB back end.
pub mod mongodb;

/// Implementation of the `thiserror` enum.
pub use errors::Error;
