use crate::commands::ingest::IngestArgs;
use crate::common;
use crate::common::enrichments;
use crate::common::ingestion;
use crate::Error;
use clap::Parser;

/// Args for generating an SBOM from the filesystem.
#[derive(Clone, Debug, Parser)]
pub struct FileSystemArgs {
    /// Path to a source code repository. The CLI will attempt to generate an SBOM for it using the
    /// Syft CLI. It will then process the SBOM and store the raw file using the configured
    /// storage provider.
    #[arg(long)]
    path: String,

    /// The name of the repository being processed.
    #[arg(long)]
    source_name: String,

    /// The version of the repository being processed. If not set, the CLI will use a shortened
    /// version of the current commit hash.
    #[arg(long)]
    source_version: Option<String>,

    /// Path to a pre-processed SBOM file. The CLI will process the SBOM and store the raw file
    /// using the configured storage provider. Requires the `source` flag also be specified.
    #[arg(long)]
    file: Option<String>,

    /// Source of the pre-processed SBOM file. Ignored if the `path` flag is specified.
    #[arg(long)]
    source: Option<String>,

    // TODO Add dir flag to support ingesting a set of pre-generated SBOMs.
    /// Indicates whether to enrich the SBOM once it has been ingested. Defaults to false if
    /// omitted.
    #[arg(short, long)]
    enrich: bool,
}

impl FileSystemArgs {
    fn to_opts(&self, debug: bool) -> Result<ingestion::RepositoryOpts, Error> {
        Ok(ingestion::RepositoryOpts {
            path: self.path.clone(),
            source_name: self.source_name.clone(),
            source_version: self.source_version.clone(),
            debug,
        })
    }
}

pub(crate) async fn execute(args: &IngestArgs) -> Result<(), Error> {
    // TODO: this debug passing is a recognized code smell. We need to get the StorageProviders
    // consolidated and then we can handle everything in one place in the CommandContext.
    let debug = args.debug;
    let args = match &args.filesystem_args {
        None => {
            return Err(Error::InvalidArg("filesystem args required".to_string()));
        }
        Some(args) => args,
    };

    let opts = match args.to_opts(debug) {
        Ok(args) => args,
        Err(e) => {
            return Err(Error::Ingest(format!(
                "error parsing filesystem args: {}",
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
