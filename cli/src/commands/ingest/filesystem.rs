use crate::commands::ingest::IngestArgs;
use crate::Error;
use clap::Parser;

/// Args for generating an SBOM from the filesystem.
#[derive(Clone, Debug, Parser)]
pub struct FileSystemArgs {}

/// Strategy pattern implementation that handles Snyk SBOM commands.
pub struct FileSystemProvider {}

impl FileSystemProvider {
    pub(crate) async fn execute(_args: &IngestArgs) -> Result<(), Error> {
        // Construct and invoke Core Services here or if args are contextual call specialized subroutine.
        todo!()
    }
}
