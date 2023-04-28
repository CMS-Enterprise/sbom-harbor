use crate::config::sdk_config_from_env;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::Client;
use aws_types::SdkConfig;
use std::collections::HashMap;
use tracing::instrument;

use crate::Error;

#[derive(Debug)]
pub struct Store {
    config: SdkConfig,
}

impl Store {
    pub fn new(config: SdkConfig) -> Self {
        Self { config }
    }

    pub async fn new_from_env() -> Result<Self, Error> {
        let config = sdk_config_from_env()
            .await
            .map_err(|e| Error::Config(e.to_string()))?;
        Ok(Self { config })
    }

    /// Inserts an object to S3. If checksum is passed, it must be base64 encoded. Returns the
    /// version id of the object.
    #[instrument]
    pub async fn insert(
        &self,
        bucket_name: String,
        key: String,
        checksum_256: Option<String>,
        body: Vec<u8>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<String, Error> {
        let client = Client::new(&self.config);
        let body = Some(ByteStream::from(body));

        let result = client
            .put_object()
            .set_key(Some(key.clone()))
            .set_body(body)
            .set_metadata(metadata)
            .set_bucket(Some(bucket_name))
            .set_checksum_sha256(checksum_256.clone())
            .send()
            .await
            .map_err(|err| Error::S3(err.to_string()))?;

        match checksum_256 {
            None => {}
            Some(checksum_256) => match result.checksum_sha256() {
                None => {
                    return Err(Error::S3("checksum_failure".to_string()));
                }
                Some(checksum) => {
                    if !checksum_256.as_str().eq(checksum) {
                        return Err(Error::S3("checksum_mismatch".to_string()));
                    }
                }
            },
        }

        match result.version_id() {
            None => {
                return Err(Error::S3("version_id_none".to_string()));
            }
            Some(version_id) => Ok(version_id.to_string()),
        }
    }
}