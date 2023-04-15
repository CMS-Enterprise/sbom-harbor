pub mod service;

use crate::Error;

/// Service that is capable of creating and storing one or more Packages and Package related types.
#[async_trait]
pub trait PackageProvider {
    /// Sync an external Package source with Harbor.
    async fn sync(&self) -> Result<(), Error>;

    // async fn sync_package<T>(&self, opts: T) -> Result<(), Error>;
}
