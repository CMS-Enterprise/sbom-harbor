use std::io::Write;
use std::str::FromStr;

use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use harbcore::entities::packages::FindingProviderKind;
use harbcore::services::findings::snyk::FindingScanProvider;
use harbcore::services::findings::{
    FileSystemStorageProvider as FindingStorageProvider, FileSystemStorageProvider,
    FindingProvider, FindingService,
};
use harbcore::services::packages::PackageService;
use harbcore::services::sboms::{FileSystemStorageProvider as SbomStorageProvider, SbomService};
use harbcore::services::scans::ScanProvider;
use harbcore::services::snyk::SnykService;

use crate::Error;

/// Specifies the CLI args for the Enrich command.
#[derive(Debug, Parser)]
pub struct EnrichArgs {
    /// Specifies with Enrichment Provider to invoke.
    #[arg(short, long)]
    pub provider: EnrichmentProviderKind,

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
        EnrichmentProviderKind::Snyk => SnykProvider::execute(&args.snyk_args).await,
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
    fn new_service() -> Result<FindingScanProvider, Error> {
        let token = harbcore::config::snyk_token().map_err(|e| Error::Config(e.to_string()))?;

        let cx = harbcore::config::mongo_context(Some("core-test"))
            .map_err(|e| Error::Config(e.to_string()))?;

        let service = FindingScanProvider::new(
            cx.clone(),
            SnykService::new(token, cx.clone()),
            PackageService::new(cx.clone()),
            FindingService::new(
                cx.clone(),
                Box::new(FileSystemStorageProvider::new(
                    "/tmp/harbor/findings".to_string(),
                )),
            ),
        );

        Ok(service)
    }

    /// Concrete implementation of the command handler. Responsible for
    /// dispatching command to the correct logic handler based on args passed.
    async fn execute(args: &Option<SnykArgs>) -> Result<(), Error> {
        match args {
            None => {
                SnykProvider::enrich().await?;
            }
            Some(a) => {
                SnykProvider::enrich().await?;
            }
        }

        Ok(())
    }

    /// If no args are passed, the CLI will scan and sync the entire registry.
    async fn enrich() -> Result<(), Error> {
        let service = Self::new_service()?;
        let mut scan = match service.init_scan(FindingProviderKind::Snyk, None).await {
            Ok(scan) => scan,
            Err(e) => {
                let msg = format!("can_scan_from_local::{}", e);
                println!("{}", msg);
                return Err(Error::Enrich(msg));
            }
        };

        service
            .scan(&mut scan)
            .await
            .map_err(|e| Error::Enrich(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[async_std::test]
    async fn can_enrich() -> Result<(), Error> {
        match SnykProvider::enrich().await {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = format!("{}", e);
                println!("{}", msg);
                return Err(Error::Enrich(msg));
            }
        }
    }

    #[async_std::test]
    async fn can_sync_project() -> Result<(), Error> {
        // let org_id = from_env("SNYK_TEST_ORG");
        // let project_id = from_env("SNYK_TEST_PROJECT");
        //
        // let args = SnykArgs{
        //     org_id,
        //     project_id,
        // };
        //
        // let (bom, findings) = SnykProvider::sync_project(&args).await?;
        // assert!(bom.is_some());
        //
        // let sbom = bom.unwrap();
        // let findings = findings.unwrap();
        //
        // let sbom = serde_json::to_string(&sbom)
        //     .map_err(|e| Error::Enrich(format!("error serializing sbom = {}", e)))?;
        //
        // let findings = serde_json::to_string(&findings)
        //     .map_err(|e| Error::Enrich(format!("error serializing findings = {}", e)))?;
        //
        // let debug_dir = from_env("DEBUG_DIR").unwrap();
        // let mut file = std::fs::File::create(format!("{}/sbom.json", debug_dir))
        //     .map_err(|e| Error::Enrich(e.to_string()))?;
        //
        // file.write_all(sbom.as_ref()).map_err(|e| Error::Enrich(e.to_string()))?;
        //
        // let mut file = std::fs::File::create(format!("{}/findings.json", debug_dir))
        //     .map_err(|e| Error::Enrich(e.to_string()))?;
        //
        // file.write_all(findings.as_ref()).map_err(|e| Error::Enrich(e.to_string()))?;

        Ok(())
    }
}
