mod adapters;
mod changeset;
mod client;
mod findings;
mod sboms;
mod service;

pub use service::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::mongo_context;
    use crate::services::sboms::SbomProvider;
    use crate::Error;

    fn test_service() -> Result<SnykService, Error> {
        let token = std::env::var("SNYK_API_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let cx = mongo_context(Some("core-test"))?;

        let service = SnykService::new(token, cx);
        Ok(service)
    }

    #[async_std::test]
    #[ignore = "manual run only"]
    async fn can_sync_sboms() -> Result<(), Error> {
        let service = test_service()?;
        let service = service as SbomProvider;

        service.sync().await?;

        Ok(())
    }

    #[async_std::test]
    async fn can_sync_purls() -> Result<(), Error> {
        let service = test_service()?;

        service
            .register_purls()
            .await
            .map_err(|e| Error::Snyk(e.to_string()))?;

        //service.registry_issues(purls).await?;

        Ok(())
    }

    #[async_std::test]
    async fn can_sync_issues() -> Result<(), Error> {
        let service = test_service()?;

        service
            .register_issues()
            .await
            .map_err(|e| Error::Snyk(e.to_string()))?;

        Ok(())
    }

    #[async_std::test]
    async fn can_register_sboms() -> Result<(), Error> {
        let service = test_service()?;

        service
            .register_sboms()
            .await
            .map_err(|e| Error::Snyk(e.to_string()))?;

        Ok(())
    }
}
