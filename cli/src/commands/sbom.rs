use std::str::FromStr;

use clap::{Parser, ValueEnum};
use clap::builder::PossibleValue;
use harbcore::config::{get_cf_domain, get_cms_team_id, get_cms_team_token, get_v1_password, get_v1_username};

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

/// Args for generating one or more SBOMs from the Snyk API.
#[derive(Clone, Debug, Parser)]
pub struct SnykArgs {}

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
        let team_id = get_cms_team_id();
        let v1_token = get_cms_team_token();
        let cf_domain = get_cf_domain();
        let v1_username = get_v1_username();
        let v1_password = get_v1_password();
    }
}

/// Strategy pattern implementation that handles Snyk SBOM commands.
struct SnykProvider {}

impl SnykProvider {
    async fn execute(_args: &Option<SnykArgs>) -> Result<(), Error> {
        // Construct and invoke Core Services here or if args are contextual call specialized subroutine.
        todo!()
    }
}