use crate::common::CommandContext;
use crate::Error;
use harbcore::entities::packages::{BuildTargetRef, ProductRef};
use harbcore::entities::sboms::{Sbom, SbomProviderKind};
use harbcore::entities::tasks::Task;
use harbcore::entities::vendors::{Product, Vendor};
use harbcore::entities::xrefs::Xref;
use harbcore::services::packages::PackageService;
use harbcore::services::sboms::{
    syft, FileSystemStorageProvider, S3StorageProvider, SbomService, StorageProvider,
};
use std::path::PathBuf;
use std::process::Command;

/// Required arguments to ingest a pre-processed SBOM supplied by a Vendor.
pub struct VendorOpts {
    /// Path to an sbom file.
    pub file: String,
    /// [Vendor] that supplied the SBOM.
    pub vendor: Vendor,
    /// [Product] the SBOM represents.
    pub product: Product,
    /// The name of the software represented by the SBOM. If the SBOM does not include a valid
    /// Package URL, this will be used to generate a Package URL. If the SBOM does include a
    /// valid Package URL, it will be validated against this value.
    pub name: String,
    /// The version of the software represented by the SBOM. If the SBOM does not include a valid
    /// Package URL, this will be used to generate a Package URL. If the SBOM does include a
    /// valid Package URL, it will be validated against this value.
    pub version: String,
    /// Flag indicating if the command is running in a debug context.
    pub debug: bool,
}

// TODO: Will need to support multiple SBOMs per repo eventually.
/// Required arguments to ingest an sbom from a clone of a source code repository.
pub struct RepositoryOpts {
    /// Path to the clone of the source code repository
    pub path: String,
    /// The name of the package for which an SBOM is being ingested. This will become part of the
    /// Package URL.
    pub package_name: String,
    /// The version of the package for which an SBOM is being ingested. This will become part of
    /// the Package URL. Defaults to the current commit hash if not set.
    pub package_version: Option<String>,
    /// Flag indicating if the command is running in a debug context.
    pub debug: bool,
}

pub(crate) async fn ingest_raw(
    raw: &str,
    provider: SbomProviderKind,
    xref: Xref,
    task: Option<&Task>,
    debug: bool,
) -> Result<Sbom, Error> {
    // If args are all valid and an SBOM can be created, instantiate the services and ingest it.
    let cx = CommandContext::new(debug).await?;
    let storage: Box<dyn StorageProvider> = match debug {
        false => Box::new(S3StorageProvider {}),
        true => Box::new(FileSystemStorageProvider::new(
            "/tmp/harbor-debug/sboms".to_string(),
        )),
    };

    let service = SbomService::new(
        cx.store.clone(),
        Some(storage),
        Some(PackageService::new(cx.store.clone())),
    );

    let sbom = service.ingest(raw, None, provider, xref, task).await?;

    Ok(sbom)
}

pub(crate) async fn ingest_repository(opts: &RepositoryOpts) -> Result<(String, Sbom), Error> {
    let path_buf = PathBuf::from(&opts.path);
    if !path_buf.is_dir() {
        return Err(Error::Ingest("path must be a directory".to_string()));
    }
    // Get commit hash if version is not set. Will fail if the path is not a git repository or if
    // git is not installed on the host.
    let version = match &opts.package_version {
        None => commit_hash(&opts.path)?,
        Some(source_version) => {
            if source_version.is_empty() {
                commit_hash(&opts.path)?
            } else {
                source_version.clone()
            }
        }
    };

    // TODO: Need to resolve repositories automatically using upstream.
    let build_target_ref = BuildTargetRef {
        team_id: "".to_string(),
        repository_id: "".to_string(),
        build_target_id: "".to_string(),
    };

    // Generate an SBOM from the repository.
    let raw = match syft(
        opts.path.as_str(),
        opts.package_name.as_str(),
        version.as_str(),
    ) {
        Ok(raw) => raw,
        Err(e) => {
            return Err(Error::Ingest(format!("error in syft provider: {}", e)));
        }
    };

    let sbom = ingest_raw(
        raw.as_str(),
        SbomProviderKind::HarborSyft,
        Xref::from(build_target_ref),
        None,
        opts.debug,
    )
    .await?;

    Ok((raw, sbom))
}

pub(crate) async fn ingest_vendor_supplied(opts: &VendorOpts) -> Result<(String, Sbom), Error> {
    let path_buf = PathBuf::from(&opts.file);
    if !path_buf.is_file() {
        return Err(Error::Ingest("path must point to a file".to_string()));
    }

    let raw = std::fs::read_to_string(path_buf)
        .map_err(|e| Error::Ingest(format!("ingest_file_error::{}", e)))?;

    let product_ref = ProductRef {
        vendor_id: opts.vendor.id.clone(),
        product_id: opts.product.id.clone(),
    };

    let sbom = ingest_raw(
        raw.as_str(),
        SbomProviderKind::Vendor(opts.vendor.name.clone()),
        Xref::from(product_ref),
        None,
        opts.debug,
    )
    .await?;

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
        let manifest_dir = platform::testing::manifest_dir()?;

        let _sbom = ingest_repository(&RepositoryOpts {
            path: manifest_dir,
            package_name: "harbor".to_string(),
            package_version: None,
            debug: true,
        })
        .await?;

        Ok(())
    }
}
