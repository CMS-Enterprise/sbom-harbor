/// Provides crate access;
pub(crate) mod service;
pub(crate) use service::*;

/// Provides module access.
pub(in crate::services::snyk) mod client;
