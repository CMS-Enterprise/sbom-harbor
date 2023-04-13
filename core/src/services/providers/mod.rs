use async_trait::async_trait;
mod snyk;

/// This module is the provider responsible for extracting
/// SBOMs from GitHub Repositories
///
pub mod github;

/// Common Trait for SBOM Providers
///
#[async_trait]
pub trait SbomProvider<T, E> {

    /// Common method to start the process
    ///
    async fn provide_sboms(&self) -> Result<T, E>;
}