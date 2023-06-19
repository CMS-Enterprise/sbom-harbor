use crate::commands::ingest::IngestArgs;
use crate::commands::CliContext;
use crate::Error;
use clap::Parser;
use harbcore::entities::sboms::{Sbom, SbomProviderKind};
use harbcore::entities::tasks::{Task, TaskKind, TaskStatus};
use harbcore::entities::xrefs::{Xref, XrefKind};
use harbcore::services::packages::PackageService;
use harbcore::services::sboms::{
    syft, FileSystemStorageProvider, S3StorageProvider, SbomService, StorageProvider,
};
use std::path::PathBuf;
use std::process::Command;

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

/// Required arguments to ingest a repository by path.
#[derive(Clone, Debug)]
pub struct PathArgs {
    /// Path to a source code repository. The CLI will attempt to generate an SBOM for it using the
    /// Syft CLI. It will then process the SBOM and store the raw file using the configured
    /// storage provider.
    path: String,

    /// The name of the repository being processed.
    source_name: String,

    /// The version of the repository being processed. If not set, the CLI will use a shortened
    /// version of the current commit hash.
    source_version: String,
}

impl PathArgs {
    fn new(parent: &FileSystemArgs) -> Result<PathArgs, Error> {
        // Get commit hash if version is not set. Will fail if the path is not a git repository or if
        // git is not installed on the host.
        let version = match &parent.source_version {
            None => commit_hash(&parent.path)?,
            Some(source_version) => {
                if source_version.is_empty() {
                    commit_hash(&parent.path)?
                } else {
                    source_version.clone()
                }
            }
        };

        Ok(PathArgs {
            path: parent.path.clone(),
            source_name: parent.source_name.clone(),
            source_version: version,
        })
    }
}

pub(crate) async fn execute(args: &IngestArgs) -> Result<(), Error> {
    // TODO: this debug passing is a recognized code smell. We need to get the StorageProviders
    // consolidated and then we can handle everything in one place.
    let debug = args.debug;

    let args = match &args.filesystem_args {
        None => {
            return Err(Error::InvalidArg("filesystem args required".to_string()));
        }
        Some(args) => args,
    };

    let enrich = args.enrich;

    let args = match PathArgs::new(args) {
        Ok(args) => args,
        Err(e) => {
            return Err(Error::Ingest(format!(
                "error parsing filesystem args: {}",
                e
            )));
        }
    };

    let sbom = ingest_by_path(&args, debug).await?;

    if enrich {
        println!("==> todo: enrich support");
    }

    println!("==> success: SBOM ingested {}", sbom.id);
    Ok(())
}

async fn ingest_by_path(args: &PathArgs, debug: bool) -> Result<Sbom, Error> {
    let path_buf = PathBuf::from(&args.path);
    if !path_buf.is_dir() {
        return Err(Error::Ingest("path must be a directory".to_string()));
    }

    // Generate an SBOM from the repository.
    let raw = match syft(
        args.path.as_str(),
        args.source_name.as_str(),
        args.source_version.as_str(),
    ) {
        Ok(raw) => raw,
        Err(e) => {
            return Err(Error::Ingest(format!("error in syft provider: {}", e)));
        }
    };

    // If args are all valid and an SBOM can be created, instantiate the services and ingest it.
    let cx = CliContext::new(debug).await?;
    let storage: Box<dyn StorageProvider> = match debug {
        false => Box::new(S3StorageProvider {}),
        true => Box::new(FileSystemStorageProvider::new(
            "/tmp/harbor-debug/sboms".to_string(),
        )),
    };

    let service = SbomService::new(
        cx.store.clone(),
        storage,
        PackageService::new(cx.store.clone()),
    );

    // TODO: This shows that Tasks are badly coupled to Sbom generation.
    let sbom = service
        .ingest(
            raw,
            None,
            SbomProviderKind::HarborSyft,
            Xref {
                kind: XrefKind::Product,
                map: Default::default(),
            },
            &Task {
                id: "00000000-0000-0000-0000-000000000000".to_string(),
                kind: TaskKind::Sbom(SbomProviderKind::HarborSyft),
                count: 1,
                timestamp: platform::time::timestamp().map_err(|e| Error::Ingest(e.to_string()))?,
                start: Default::default(),
                finish: Default::default(),
                duration_seconds: 0,
                status: TaskStatus::Started,
                err: None,
                ref_errs: None,
                err_total: 0,
            },
        )
        .await
        .map_err(|e| Error::Ingest(e.to_string()))?;

    Ok(sbom)
}

/// Get the current commit hash of the repository.
fn commit_hash(source_path: &str) -> Result<String, Error> {
    let output = match Command::new("git")
        .current_dir(source_path)
        .arg("rev-parse")
        .arg("--short")
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(Error::Ingest(format!("error executing git: {}", err)));
        }
    };

    // Handle error generated by git.
    if !&output.status.success() {
        match String::from_utf8(output.stderr) {
            Ok(stderr) => {
                Error::Ingest(format!("error retrieving commit hash: {}", &stderr));
            }
            Err(err) => {
                return Err(Error::Ingest(format!(
                    "error formatting git stderr: {}",
                    &err
                )));
            }
        };
    }

    if output.stdout.is_empty() {
        return Err(Error::Ingest("git returned empty commit hash".to_string()));
    };

    match String::from_utf8(output.stdout) {
        Ok(commit_hash) => Ok(commit_hash),
        Err(e) => Err(Error::Ingest(format!("error reading git stdout: {}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[async_std::test]
    async fn can_ingest_by_path() -> Result<(), Error> {
        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("cannot access CARGO_MANIFEST_DIR");
        let manifest_dir = manifest_dir.replace("/cli", "");

        let _sbom = ingest_by_path(
            &PathArgs {
                path: manifest_dir,
                source_name: "harbor".to_string(),
                source_version: "".to_string(),
            },
            true,
        )
        .await?;

        Ok(())
    }
}
