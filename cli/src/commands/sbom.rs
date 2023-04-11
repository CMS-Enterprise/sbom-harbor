use std::str::FromStr;

use clap::{Parser, ValueEnum};
use clap::builder::PossibleValue;
use harbcore::services::{SbomProvider, SnykService};
use platform::mongodb::Context;

use crate::Error;

/// The SBOM Command handler.
pub async fn execute(args: &SbomArgs) -> Result<(), Error> {
    match args.provider {
        SbomProviderKind::FileSystem => {
            FileSystemProvider::execute(&args.filesystem_args).await
        }
        SbomProviderKind::GitHub => {
            GithubProvider::execute(&args.github_args).await
        }
        SbomProviderKind::Snyk => {
            SnykProvider::execute(&args.snyk_args).await
        }
    }
}

/// Enumerates which SBOM generation strategy to employ.
#[derive(Clone, Debug)]
pub(crate) enum SbomProviderKind {
    /// Generate a single SBOM from the filesystem using the default Syft provider.
    FileSystem,
    /// Generate one or more SBOMs from GitHub. Generation strategy depends on configuration options.
    GitHub,
    /// Generate one or more SBOMs from Snyk. Generation strategy depends on configuration options.
    Snyk,
}

impl ValueEnum for SbomProviderKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::FileSystem, Self::GitHub, Self::Snyk]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            SbomProviderKind::FileSystem => {
                PossibleValue::new("filesystem").help("Use the default SBOM Provider to generate an SBOM from the local filesystem")
            }
            SbomProviderKind::GitHub => {
                PossibleValue::new("github").help("Use the GitHub SBOM Provider to generate one or more SBOMs from the GitHub API")
            }
            SbomProviderKind::Snyk => {
                PossibleValue::new("github").help("Use the Snyk SBOM Provider to generate one or more SBOMs from the Snyk API")
            }
        })
    }
}

/// Specifies the CLI args for the SBOM command.
#[derive(Debug, Parser)]
pub struct SbomArgs {
    /// Specifies with SBOM Provider to invoke.
    #[arg(short, long)]
    provider: SbomProviderKind,

    /// Flattened args for use with the file system SBOM provider.
    #[command(flatten)]
    pub filesystem_args: Option<FileSystemArgs>,

    /// Flattened args for use with the GitHub SBOM provider.
    #[command(flatten)]
    pub github_args: Option<GitHubArgs>,

    /// Flattened args for use with the Snyk SBOM provider.
    #[command(flatten)]
    pub snyk_args: Option<SnykArgs>,
}

/// Args for generating and SBOM from the filesystem.
#[derive(Clone, Debug, Parser)]
pub struct FileSystemArgs {}

/// Args for generating one ore more SBOMs from a GitHub Organization.
#[derive(Clone, Debug, Parser)]
pub struct GitHubArgs {}

/// Args for generating a single SBOM from the Snyk API.
#[derive(Clone, Debug, Parser)]
pub struct SnykArgs {
    /// The Snyk Org ID for the SBOM target.
    #[arg(short, long)]
    pub org_id: Option<String>,
    /// The Snyk Project ID for the SBOM target.
    #[arg(short, long)]
    pub project_id: Option<String>,
}

impl FromStr for SbomProviderKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.to_lowercase();
        let value = value.as_str();
        match value {
            "filesystem"| "f" => Ok(SbomProviderKind::FileSystem),
            "github"| "gh" | "g" => Ok(SbomProviderKind::GitHub),
            "snyk" | "s" => Ok(SbomProviderKind::Snyk),
            _ => Err(()),
        }
    }
}


/// Strategy pattern implementation that handles Snyk SBOM commands.
struct FileSystemProvider {}

impl FileSystemProvider {
    async fn execute(_args: &Option<FileSystemArgs>) -> Result<(), Error> {
        // Construct and invoke Core Services here or if args are contextual call specialized subroutine.
        todo!()
    }
}

/// Strategy pattern implementation that handles Snyk SBOM commands.
struct GithubProvider {}

impl GithubProvider {
    async fn execute(_args: &Option<GitHubArgs>) -> Result<(), Error> {
        // Construct and invoke Core Services here or if args are contextual call specialized subroutine.
        todo!()
    }
}

/// Strategy pattern implementation that handles Snyk SBOM commands.
struct SnykProvider {}

impl SnykProvider {
    /// Factory method to create new instance of type.
    fn new_service(cx: Context) -> Result<SnykService, Error> {
        let token = harbcore::config::snyk_token()
            .map_err(|e| Error::Config(e.to_string()))?;

       Ok(SnykService::new(token, cx))
    }

    async fn execute(args: &Option<SnykArgs>) -> Result<(), Error> {
        match args {
            None => Err(Error::Sbom("snyk args required".to_string())),
            Some(args) => {
                let cx = harbcore::config::mongo_context(None)
                    .map_err(|e| Error::Config(e.to_string()))?;

                match args {
                    None => {
                        let service = Self::new_service(cx)?;
                        service.sync().await?;
                    },
                    Some(args) => {
                        todo!() // Handle single project target use case.
                    }
                }
            }
        }
    }
}

mod tests {
    use harbcore::config::{Environ, environment};
    use platform::config::from_env;
    use super::*;
    use crate::Error;

    #[async_std::test]
    async fn can_sync() -> Result<(), Error> {
        // Figure out how to test arg parsing.
        todo!()
    }
}