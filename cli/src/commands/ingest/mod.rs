use crate::commands::ingest::filesystem::{FileSystemArgs, FileSystemProvider};
use crate::commands::ingest::github::{GitHubArgs, GithubProvider};
use crate::commands::ingest::snyk::SnykArgs;
use crate::commands::ingest::snyk::SnykProvider;
use crate::Error;
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use std::str::FromStr;

mod filesystem;
mod github;
mod snyk;

/// The SBOM Command handler.
pub async fn execute(args: &IngestArgs) -> Result<(), Error> {
    match args.provider {
        IngestionProviderKind::FileSystem => FileSystemProvider::execute(args).await,
        IngestionProviderKind::GitHub => GithubProvider::execute(args).await,
        IngestionProviderKind::Snyk => SnykProvider::execute(args).await,
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

#[cfg(test)]
mod tests {
    use crate::commands::enrich::snyk::SnykProvider;
    use crate::Error;
    use harbcore::config::dev_context;
    use harbcore::entities;
    use harbcore::entities::tasks::{Task, TaskKind};
    use harbcore::services::vulnerabilities::{FileSystemStorageProvider, StorageProvider};
    use harbcore::tasks::TaskProvider;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_execute_snyk() -> Result<(), Error> {
        let cx = dev_context(Some("core-test")).map_err(|e| Error::Config(e.to_string()))?;

        let storage: Box<dyn StorageProvider> = Box::new(FileSystemStorageProvider::new(
            "/tmp/harbor-debug/sboms".to_string(),
        ));

        let mut task: Task = Task::new(TaskKind::Sbom(entities::sboms::SbomProviderKind::Snyk))
            .map_err(|e| Error::Sbom(e.to_string()))?;

        let provider = SnykProvider::new_provider(cx, storage).await?;

        match provider.execute(&mut task).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = e.to_string();
                Err(Error::Sbom(msg))
            }
        }
    }
}
