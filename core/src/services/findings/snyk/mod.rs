mod changeset;
mod provider;

#[cfg(test)]
mod tests {
    use crate::config::{mongo_context, snyk_token};
    use crate::entities::packages::{Finding, FindingProviderKind, Purl};
    use crate::entities::sboms::{Sbom, SbomProviderKind};
    use crate::entities::scans::{Scan, ScanKind, ScanRef};
    use crate::entities::xrefs::{XrefKind, Xrefs};
    use crate::services::findings::snyk::changeset::ChangeSet;
    use crate::services::findings::snyk::provider::FindingScanProvider;
    use crate::services::findings::{
        FileSystemStorageProvider, FindingProvider, FindingService, StorageProvider,
    };
    use crate::services::packages::PackageService;
    use crate::services::sboms::{
        FileSystemStorageProvider as SbomFileSystemStorageProvider, SbomService,
    };
    use crate::services::snyk::adapters::Project;
    use crate::services::snyk::{ProjectStatus, API_VERSION};
    use crate::services::snyk::{SnykService, SNYK_DISCRIMINATOR};
    use crate::Error;
    use platform::mongodb::{Context, Service};
    use std::collections::hash_map::Iter;
    use std::collections::HashMap;
    use std::fmt::{Debug, Formatter};
    use tracing::log::debug;

    #[async_std::test]
    async fn can_scan_from_local() -> Result<(), Error> {
        let service = test_service()?;
        let mut scan = match service.init_scan(FindingProviderKind::Snyk, None).await {
            Ok(scan) => scan,
            Err(e) => {
                let msg = format!("can_store_findings_change_set_from_local::{}", e);
                debug!("{}", msg);
                return Err(Error::Enrichment(msg));
            }
        };

        service.scan(&mut scan).await
    }

    #[async_std::test]
    async fn can_store_findings_change_set_from_local() -> Result<(), Error> {
        let service = test_service()?;
        let mut scan = match service.init_scan(FindingProviderKind::Snyk, None).await {
            Ok(scan) => scan,
            Err(e) => {
                let msg = format!("can_store_findings_change_set_from_local::{}", e);
                debug!("{}", msg);
                return Err(Error::Enrichment(msg));
            }
        };

        let mut change_set = ChangeSet::new(&mut scan);
        let snyk_kind = XrefKind::External(SNYK_DISCRIMINATOR.to_string());
        let mut with_findings = 0;

        let purls = service.packages.list().await?;
        let mut purls_with_findings = HashMap::<String, u8>::new();

        for mut purl in purls.into_iter() {
            match service.scan_purl(&mut change_set, &mut purl).await {
                Ok(_) => {}
                Err(e) => {
                    debug!("{}", e);
                }
            }
        }

        Ok(())
    }

    fn debug_service() -> Result<DebugService, Error> {
        let token = snyk_token()?;
        let cx = mongo_context(Some("core-test"))?;

        let service = DebugService::new(token, cx);
        Ok(service)
    }

    fn test_service() -> Result<FindingScanProvider, Error> {
        let token = snyk_token()?;
        let cx = mongo_context(Some("core-test"))?;
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

    /// Used for manual local debugging.
    #[derive(Debug)]
    struct DebugService {
        cx: Context,
        token: String,
    }

    impl Service<Scan> for DebugService {
        fn cx(&self) -> &Context {
            &self.cx
        }
    }

    impl Service<Purl> for DebugService {
        fn cx(&self) -> &Context {
            &self.cx
        }
    }

    impl DebugService {
        fn new(token: String, cx: Context) -> DebugService {
            DebugService { cx, token }
        }

        // Use this to load purls that have already been stored. Speeds up testing dramatically.
        async fn purls(&self) -> Result<Vec<Purl>, Error> {
            self.list().await.map_err(|e| Error::Runtime(e.to_string()))
        }
    }
}
