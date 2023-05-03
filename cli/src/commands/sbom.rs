use std::str::FromStr;

use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use harbcore::entities;
use harbcore::services::packages::PackageService;
use harbcore::services::sboms::snyk::SbomScanProvider;
use harbcore::services::sboms::{
    FileSystemStorageProvider, S3StorageProvider, SbomProvider, SbomService, StorageProvider,
};
use harbcore::services::snyk::{SnykService, API_VERSION};
use platform::mongodb::Context;

use crate::Error;

/// The SBOM Command handler.
pub async fn execute(args: &SbomArgs) -> Result<(), Error> {
    match args.provider {
        SbomProviderKind::FileSystem => FileSystemProvider::execute(args).await,
        SbomProviderKind::GitHub => GithubProvider::execute(args).await,
        SbomProviderKind::Snyk => SnykProvider::execute(args).await,
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
            SbomProviderKind::FileSystem => PossibleValue::new("filesystem").help(
                "Use the default SBOM Provider to generate an SBOM from the local filesystem",
            ),
            SbomProviderKind::GitHub => PossibleValue::new("github").help(
                "Use the GitHub SBOM Provider to generate one or more SBOMs from the GitHub API",
            ),
            SbomProviderKind::Snyk => PossibleValue::new("github")
                .help("Use the Snyk SBOM Provider to generate one or more SBOMs from the Snyk API"),
        })
    }
}

/// Specifies the CLI args for the SBOM command.
#[derive(Debug, Parser)]
pub struct SbomArgs {
    /// Specifies with SBOM Provider to invoke.
    #[arg(short, long)]
    provider: SbomProviderKind,

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
            "filesystem" | "f" => Ok(SbomProviderKind::FileSystem),
            "github" | "gh" | "g" => Ok(SbomProviderKind::GitHub),
            "snyk" | "s" => Ok(SbomProviderKind::Snyk),
            _ => Err(()),
        }
    }
}

/// Strategy pattern implementation that handles Snyk SBOM commands.
struct FileSystemProvider {}

impl FileSystemProvider {
    async fn execute(_args: &SbomArgs) -> Result<(), Error> {
        // Construct and invoke Core Services here or if args are contextual call specialized subroutine.
        todo!()
    }
}

/// Strategy pattern implementation that handles Snyk SBOM commands.
struct GithubProvider {}

impl GithubProvider {
    async fn execute(_args: &SbomArgs) -> Result<(), Error> {
        // Construct and invoke Core Services here or if args are contextual call specialized subroutine.
        todo!()
    }
}

/// Strategy pattern implementation that handles Snyk SBOM commands.
struct SnykProvider {}

impl SnykProvider {
    /// Factory method to create new instance of type.
    fn new_provider(
        cx: Context,
        storage: Box<dyn StorageProvider>,
    ) -> Result<SbomScanProvider, Error> {
        let token = harbcore::config::snyk_token().map_err(|e| Error::Config(e.to_string()))?;

        let provider = SbomScanProvider::new(
            cx.clone(),
            SnykService::new(token),
            PackageService::new(cx.clone()),
            SbomService::new(cx, storage),
        );

        Ok(provider)
    }

    /// Concrete implementation of the command handler. Responsible for
    /// dispatching command to the correct logic handler based on args passed.
    async fn execute(args: &SbomArgs) -> Result<(), Error> {
        let storage: Box<dyn StorageProvider>;

        let cx = match &args.debug {
            false => {
                storage = Box::new(S3StorageProvider {});
                harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?
            }
            true => {
                storage = Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor-debug/sboms".to_string(),
                ));
                harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?
            }
        };

        match &args.snyk_args {
            None => Err(Error::Sbom("snyk args required".to_string())),
            Some(args) => match (&args.org_id, &args.project_id) {
                (None, None) => {
                    let provider = SnykProvider::new_provider(cx, storage)?;
                    provider
                        .execute(entities::sboms::SbomProviderKind::Snyk {
                            api_version: API_VERSION.to_string(),
                        })
                        .await
                        .map_err(|e| Error::Sbom(e.to_string()))
                }
                (_, _) => Err(Error::Sbom(
                    "individual project scans not yet implemented".to_string(),
                )),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use harbcore::config::dev_context;
    use harbcore::services::sboms::FileSystemStorageProvider;

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_execute() -> Result<(), Error> {
        let cx = dev_context(Some("core-test")).map_err(|e| Error::Config(e.to_string()))?;

        let storage: Box<dyn StorageProvider> = Box::new(FileSystemStorageProvider::new(
            "/tmp/harbor-debug/sboms".to_string(),
        ));

        let provider = SnykProvider::new_provider(cx, storage)?;

        match provider
            .execute(entities::sboms::SbomProviderKind::Snyk {
                api_version: API_VERSION.to_string(),
            })
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = e.to_string();
                Err(Error::Sbom(msg))
            }
        }
    }
}
