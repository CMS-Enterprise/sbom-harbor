use crate::commands::ingest::filesystem::FileSystemArgs;
use crate::commands::ingest::github::GitHubArgs;
use crate::commands::ingest::snyk::SnykArgs;
use crate::Error;
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use std::str::FromStr;

mod filesystem;
mod github;
mod snyk;

/// The CommandFactory function for the `ingest` command.
pub async fn execute(args: &IngestArgs) -> Result<(), Error> {
    match args.provider {
        IngestionProviderKind::FileSystem => filesystem::execute(args).await,
        IngestionProviderKind::GitHub => github::execute(args).await,
        IngestionProviderKind::Snyk => snyk::execute(args).await,
    }
}

/// Enumerates which SBOM ingestion provider to execute.
#[derive(Clone, Debug)]
pub(crate) enum IngestionProviderKind {
    /// Generate a single SBOM from the filesystem using the default Syft provider.
    FileSystem,
    /// Generate one or more SBOMs from GitHub. Generation strategy depends on configuration options.
    GitHub,
    /// Generate one or more SBOMs from Snyk. Generation strategy depends on configuration options.
    Snyk,
}

impl ValueEnum for IngestionProviderKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::FileSystem, Self::GitHub, Self::Snyk]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            IngestionProviderKind::FileSystem => PossibleValue::new("filesystem")
                .help("Ingests one or more SBOMs from the local filesystem."),
            IngestionProviderKind::GitHub => PossibleValue::new("github").help(
                "Generates and ingests SBOMs by analyzing a GitHub organization and its \
                repositories.",
            ),
            IngestionProviderKind::Snyk => PossibleValue::new("snyk").help(
                "Generates and ingests SBOMs using the Snyk API. Requires a Snyk Account \
                and API token.",
            ),
        })
    }
}

impl FromStr for IngestionProviderKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.to_lowercase();
        let value = value.as_str();
        match value {
            "filesystem" | "f" => Ok(IngestionProviderKind::FileSystem),
            "github" | "gh" | "g" => Ok(IngestionProviderKind::GitHub),
            "snyk" | "s" => Ok(IngestionProviderKind::Snyk),
            _ => Err(()),
        }
    }
}

/// Specifies the CLI args for the `ingest` command.
#[derive(Debug, Parser)]
pub struct IngestArgs {
    /// Specifies with SBOM Provider to invoke.
    #[arg(short, long)]
    provider: IngestionProviderKind,

    /// Specifies to run the command against the local debug environment.
    #[arg(long)]
    debug: bool,

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

#[cfg(test)]
mod tests {
    use super::filesystem;
    use super::snyk;
    use crate::commands::ingest::filesystem::FileSystemArgs;
    use crate::commands::ingest::snyk::SnykArgs;
    use crate::commands::ingest::{IngestArgs, IngestionProviderKind};
    use crate::Error;

    #[async_std::test]
    #[ignore = "debug"]
    async fn debug_snyk() -> Result<(), Error> {
        match snyk::execute(&IngestArgs {
            provider: IngestionProviderKind::Snyk,
            debug: true,
            filesystem_args: None,
            github_args: None,
            snyk_args: Some(SnykArgs {
                org_id: None,
                project_id: None,
            }),
        })
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = e.to_string();
                Err(Error::Ingest(msg))
            }
        }
    }

    #[async_std::test]
    #[ignore = "debug"]
    async fn debug_filesystem() -> Result<(), Error> {
        let manifest_dir = platform::testing::workspace_dir()?;

        match filesystem::execute(&IngestArgs {
            provider: IngestionProviderKind::FileSystem,
            debug: true,
            filesystem_args: Some(FileSystemArgs {
                path: manifest_dir,
                package_name: "harbor".to_string(),
                package_version: None,
                file: None,
                source: None,
                enrich: false,
            }),
            github_args: None,
            snyk_args: None,
        })
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = e.to_string();
                Err(Error::Ingest(msg))
            }
        }
    }
}
