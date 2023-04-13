mod service;
pub use service::*;

use crate::Error;

/// Service that is capable of creating and storing one or more SBOMs.
#[async_trait]
pub trait SbomProvider {
    /// Sync an external SBOM source with the Harbor Registry.
    async fn sync(&self) -> Result<(), Error>;
    // TODO
    // async fn sync_one<T>(&self, opts: T) -> Result<(), Error>;
}
