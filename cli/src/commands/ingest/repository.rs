use crate::commands::ingest::IngestArgs;
use crate::common;
use crate::common::enrichments;
use crate::common::ingestion;
use crate::Error;
use clap::Parser;

/// Args for generating an SBOM from a git repository.
#[derive(Clone, Debug, Parser)]
pub struct RepositoryArgs {
    /// Path to a git repository. The CLI will attempt to generate an SBOM for it using the
    /// Syft CLI. It will then process the SBOM and store the raw file using the configured
    /// storage provider.
    #[arg(long)]
    pub(crate) path: String,

    /// The name of the package the SBOM represents.
    #[arg(long)]
    pub(crate) name: String,

    /// The version of the package the SBOM represents. If not set, the CLI will use a shortened
    /// version of the current commit hash.
    #[arg(long)]
    pub(crate) version: Option<String>,

    /// Indicates whether to enrich the SBOM once it has been ingested. Defaults to false if
    /// omitted.
    #[arg(short, long)]
    pub(crate) enrich: bool,
}

impl RepositoryArgs {
    pub fn to_opts(&self, debug: bool) -> Result<ingestion::RepositoryOpts, Error> {
        Ok(ingestion::RepositoryOpts {
            path: self.path.clone(),
            package_name: self.name.clone(),
            package_version: self.version.clone(),
            debug,
        })
    }
}

pub(crate) async fn execute(args: &IngestArgs) -> Result<(), Error> {
    // TODO: this debug passing is a recognized code smell. We need to get the StorageProviders
    // consolidated and then we can handle everything in one place in the CommandContext.
    let debug = args.debug;
    let args = match &args.repository_args {
        None => {
            return Err(Error::InvalidArg("repository args required".to_string()));
        }
        Some(args) => args,
    };

    let opts = match args.to_opts(debug) {
        Ok(args) => args,
        Err(e) => {
            return Err(Error::Ingest(format!(
                "error parsing repository args: {}",
                e
            )));
        }
    };

    let (raw, sbom) = ingestion::ingest_repository(&opts).await?;
    if debug {
        common::pretty_print_json(raw.as_str());
    }

    println!("==> success: SBOM ingested {}", sbom.id);

    if args.enrich {
        let vulns = enrichments::grype::from_raw_sbom(raw.as_bytes())?;
        if debug {
            let json = match serde_json::to_string(&vulns) {
                Ok(json) => json,
                Err(e) => return Err(Error::Enrich(e.to_string())),
            };
            println!("==> success: SBOM scanned");
            common::pretty_print_json(json.as_str());
        }
    }
    Ok(())
}
