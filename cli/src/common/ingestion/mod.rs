use crate::common::CommandContext;
use crate::Error;
use harbcore::entities::sboms::{Sbom, SbomProviderKind};
use harbcore::entities::tasks::{Task, TaskKind, TaskStatus};
use harbcore::entities::xrefs::{Xref, XrefKind};
use harbcore::services::packages::PackageService;
use harbcore::services::sboms::{
    syft, FileSystemStorageProvider, S3StorageProvider, SbomService, StorageProvider,
};
use std::path::PathBuf;
use std::process::Command;

// TODO: Will need to support multiple SBOMs per repo eventually.
/// Required arguments to ingest an sbom from a clone of a source code repository.
pub struct RepositoryOpts {
    /// Path to the clone of the source code repository
    pub path: String,
    /// The name of the software being ingested. This will become part of the Package URL.
    pub source_name: String,
    /// The version of the software being ingested. This will become part of the Package URL.
    /// Defaults to the current commit hash if not set.
    pub source_version: Option<String>,
    /// Flag indicating if the command is running in a debug context.
    pub debug: bool,
}

pub(crate) async fn ingest_repository(opts: &RepositoryOpts) -> Result<(String, Sbom), Error> {
    let path_buf = PathBuf::from(&opts.path);
    if !path_buf.is_dir() {
        return Err(Error::Ingest("path must be a directory".to_string()));
    }
    // Get commit hash if version is not set. Will fail if the path is not a git repository or if
    // git is not installed on the host.
    let version = match &opts.source_version {
        None => commit_hash(&opts.path)?,
        Some(source_version) => {
            if source_version.is_empty() {
                commit_hash(&opts.path)?
            } else {
                source_version.clone()
            }
        }
    };

    // Generate an SBOM from the repository.
    let raw = match syft(
        opts.path.as_str(),
        opts.source_name.as_str(),
        version.as_str(),
    ) {
        Ok(raw) => raw,
        Err(e) => {
            return Err(Error::Ingest(format!("error in syft provider: {}", e)));
        }
    };

    // If args are all valid and an SBOM can be created, instantiate the services and ingest it.
    let cx = CommandContext::new(opts.debug).await?;
    let storage: Box<dyn StorageProvider> = match opts.debug {
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
            raw.as_str(),
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

    Ok((raw, sbom))
}

/// Get the current commit hash of the repository.
pub(crate) fn commit_hash(source_path: &str) -> Result<String, Error> {
    let mut cmd = Command::new("git");
    let cmd = cmd
        .current_dir(source_path)
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD");

    platform::process::execute(cmd, "git").map_err(|e| Error::Ingest(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ingestion::RepositoryOpts;
    use crate::Error;

    #[async_std::test]
    async fn can_ingest_by_path() -> Result<(), Error> {
        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("cannot access CARGO_MANIFEST_DIR");
        let manifest_dir = manifest_dir.replace("/cli", "");

        let _sbom = ingest_repository(&RepositoryOpts {
            path: manifest_dir,
            source_name: "harbor".to_string(),
            source_version: None,
            debug: true,
        })
        .await?;

        Ok(())
    }
}
