mod service;
pub use service::*;

use crate::Error;

use async_trait::async_trait;

/// Service that is capable of creating and storing one or more SBOMs.
#[async_trait]
pub trait SbomProvider {
    /// Sync an external SBOM source with Harbor.
    async fn sync(&self) -> Result<(), Error>;
    // TODO
    // async fn sync_one<T>(&self, opts: T) -> Result<(), Error>;
}
