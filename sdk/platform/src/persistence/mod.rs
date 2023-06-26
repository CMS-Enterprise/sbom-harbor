/// Provides a a micro-object-data-mapping framework for common CRUD based operations against a
/// MongoDB or DocumentDB data store.
pub mod mongodb;

/// Provides AWS S3 specific persistence capabilities.
pub mod s3;

/// Provides csv persistence capabilities.
pub mod csv;

#[cfg(test)]
mod tests {
    use crate::config::from_env;
    use crate::persistence::s3::Store;
    use crate::Error;
    use std::collections::HashMap;

    // TODO: Figure out how to spin up localstack to test.
    #[async_std::test]
    #[ignore = "TODO: localstack"]
    async fn can_store_file_to_s3() -> Result<(), Error> {
        let store = Store::new_from_env().await?;
        let body = "can-store-file-to-s3".to_string();
        let bucket_name = match from_env("PLATFORM_FILE_STORE") {
            None => return Err(Error::Config("Platform file store not set".to_string())),
            Some(bucket_name) => bucket_name,
        };

        store
            .put(
                bucket_name.clone(),
                "can-store-file-to-s3".to_string(),
                None,
                body.clone().as_bytes().to_vec(),
                Some(HashMap::from([(
                    "shiny".to_string(),
                    "metadata".to_string(),
                )])),
            )
            .await?;

        store
            .delete(bucket_name, "can-store-file-to-s3".to_string())
            .await?;

        Ok(())
    }
}
