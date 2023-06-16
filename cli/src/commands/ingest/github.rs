use crate::commands::ingest::IngestArgs;
use crate::Error;
use clap::Parser;

/// Args for generating one or more SBOMs from a GitHub Organization.
#[derive(Clone, Debug, Parser)]
pub struct GitHubArgs {}

/// Strategy pattern implementation that handles Snyk SBOM commands.
pub struct GithubProvider {}

impl GithubProvider {
    pub(crate) async fn execute(_args: &IngestArgs) -> Result<(), Error> {
        // Construct and invoke Core Services here or if args are contextual call specialized subroutine.
        todo!()
    }
}
