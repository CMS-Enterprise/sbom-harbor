use std::str::FromStr;
use std::sync::Arc;

use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use harbcore::entities::packages::FindingProviderKind;
use harbcore::entities::scans::{Scan, ScanKind};
use harbcore::services::findings::snyk::FindingScanProvider;
use harbcore::services::findings::{
    FileSystemStorageProvider, FindingService, S3StorageProvider, StorageProvider,
};
use harbcore::services::packages::PackageService;
use harbcore::services::scans::ScanProvider;
use harbcore::services::snyk::SnykService;
use platform::mongodb::{Context, Store};

use crate::Error;

/// Specifies the CLI args for the Enrich command.
#[derive(Debug, Parser)]
pub struct EnrichArgs {
    /// Specifies with Enrichment Provider to invoke.
    #[arg(short, long)]
    pub provider: EnrichmentProviderKind,

    /// Specifies to run the command against the local debug environment.
    #[arg(long)]
    debug: bool,

    /// Flattened args for use with the Snyk enrichment provider.
    #[command(flatten)]
    pub snyk_args: Option<SnykArgs>,

    /// Flattened args for use with the Dependency Track enrichment provider.
    #[command(flatten)]
    pub dt_args: Option<DependencyTrackArgs>,
}

/// The Enrich Command handler.
pub async fn execute(args: &EnrichArgs) -> Result<(), Error> {
    match args.provider {
        EnrichmentProviderKind::DependencyTrack => {
            todo!()
        }
        EnrichmentProviderKind::Snyk => SnykProvider::execute(args).await,
    }
}

/// Enumerates the supported enrichment providers.
#[derive(Clone, Debug)]
pub enum EnrichmentProviderKind {
    /// Use the Dependency Track enrichment provider.
    DependencyTrack,
    /// Use the Snyk enrichment provider.
    Snyk,
}

impl ValueEnum for EnrichmentProviderKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::DependencyTrack, Self::Snyk]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::DependencyTrack => {
                PossibleValue::new("dependency-track").help("Run Dependency Track enrichment")
            }
            Self::Snyk => PossibleValue::new("snyk").help("Run Snyk enrichment"),
        })
    }
}

impl FromStr for EnrichmentProviderKind {
    type Err = ();

    fn from_str(s: &str) -> Result<EnrichmentProviderKind, Self::Err> {
        let value = s.to_lowercase();
        let value = value.as_str();
        match value {
            "dependencytrack" | "dependency-track" | "d" | "dt" => {
                Ok(EnrichmentProviderKind::DependencyTrack)
            }
            "snyk" | "s" => Ok(EnrichmentProviderKind::Snyk),
            _ => Err(()),
        }
    }
}

/// Args for use with the Dependency Track enrichment provider.
#[derive(Clone, Debug, Parser)]
pub struct DependencyTrackArgs {}

/// Args for use with the Snyk enrichment provider.
#[derive(Clone, Debug, Parser)]
pub struct SnykArgs {
    /// The Snyk Org ID for the enrichment target.
    #[arg(short, long)]
    pub org_id: Option<String>,
    /// The Snyk Project ID for the enrichment target.
    #[arg(long)]
    pub project_id: Option<String>,
}

/// Strategy pattern implementation that handles Snyk Enrich commands.
struct SnykProvider {}

impl SnykProvider {
    /// Factory method to create new instance of type.
    async fn new_provider(
        cx: Context,
        storage: Box<dyn StorageProvider>,
    ) -> Result<FindingScanProvider, Error> {
        let token = harbcore::config::snyk_token().map_err(|e| Error::Config(e.to_string()))?;
        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Sbom(e.to_string()))?,
        );

        let provider = FindingScanProvider::new(
            store.clone(),
            SnykService::new(token),
            PackageService::new(store.clone()),
            FindingService::new(store, storage),
        )
        .map_err(|e| Error::Enrich(e.to_string()))?;

        Ok(provider)
    }

    /// Concrete implementation of the command handler. Responsible for
    /// dispatching command to the correct logic handler based on args passed.
    async fn execute(args: &EnrichArgs) -> Result<(), Error> {
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
            None => {
                let mut scan: Scan = Scan::new(ScanKind::Finding(FindingProviderKind::Snyk))
                    .map_err(|e| Error::Enrich(e.to_string()))?;

                let provider = SnykProvider::new_provider(cx, storage)
                    .await
                    .map_err(|e| Error::Enrich(e.to_string()))?;

                provider
                    .execute(&mut scan)
                    .await
                    .map_err(|e| Error::Sbom(e.to_string()))
            }
            Some(_a) => Err(Error::Sbom(
                "individual project scans not yet implemented".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use harbcore::config::dev_context;
    use harbcore::services::findings::{FileSystemStorageProvider, StorageProvider};

    #[async_std::test]
    #[ignore = "debug manual only"]
    async fn can_execute() -> Result<(), Error> {
        let cx = dev_context(Some("core-test")).map_err(|e| Error::Config(e.to_string()))?;
        let storage: Box<dyn StorageProvider> = Box::new(FileSystemStorageProvider::new(
            "/tmp/harbor-debug/findings".to_string(),
        ));

        let mut scan: Scan = Scan::new(ScanKind::Finding(FindingProviderKind::Snyk))
            .map_err(|e| Error::Enrich(e.to_string()))?;

        let provider = SnykProvider::new_provider(cx, storage).await?;

        match provider.execute(&mut scan).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = e.to_string();
                Err(Error::Sbom(msg))
            }
        }
    }
}
