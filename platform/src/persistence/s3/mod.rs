use crate::config::sdk_config_from_env;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::Client;
use aws_types::SdkConfig;
use std::collections::HashMap;
use tracing::instrument;

use crate::Error;

/// Provides a coarse-grained abstraction over S3 that conforms to the conventions of this crate.
#[derive(Debug)]
pub struct Store {
    config: SdkConfig,
}

impl Store {
    /// Factory method for creating new instance of type.
    pub fn new(config: SdkConfig) -> Self {
        Self { config }
    }

    /// Factory method for creating new instance of type. SDK Configuration is retrieved from the
    /// environment.
    pub async fn new_from_env() -> Result<Self, Error> {
        let config = sdk_config_from_env()
            .await
            .map_err(|e| Error::Config(e.to_string()))?;
        Ok(Self { config })
    }

    /// Inserts an object to S3. If checksum is passed, it must be base64 encoded. Returns the
    /// version id of the object.
    #[instrument]
    pub async fn put(
        &self,
        bucket_name: String,
        key: String,
        _checksum_256: Option<String>,
        body: Vec<u8>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<(), Error> {
        let client = Client::new(&self.config);
        // let body = Some(ByteStream::from(body));
        let body = ByteStream::from(body);

        // TODO: Come back to checksum handling.
        return match client
            .put_object()
            .key(key.clone())
            .body(body)
            .set_metadata(metadata)
            .bucket(bucket_name.clone())
            //.set_checksum_sha256(checksum_256.clone())
            .send()
            .await
        {
            Ok(_result) => Ok(()),
            Err(e) => {
                println!("s3_error::{}::{}", bucket_name, e);
                let raw = e.to_string();
                let msg = e.into_service_error();
                let msg = match msg.message() {
                    None => format!("service_error_none::{}", raw),
                    Some(msg) => msg.to_string(),
                };
                println!("{}", msg);
                Err(Error::S3(msg))
            }
        };

        // match checksum_256 {
        //     None => {}
        //     Some(checksum_256) => match result.checksum_sha256() {
        //         None => {
        //             return Err(Error::S3("checksum_failure".to_string()));
        //         }
        //         Some(checksum) => {
        //             if !checksum_256.as_str().eq(checksum) {
        //                 return Err(Error::S3("checksum_mismatch".to_string()));
        //             }
        //         }
        //     },
        // }

        // Ok(())
    }

    /// Inserts an object to S3. If checksum is passed, it must be base64 encoded. Returns the
    /// version id of the object.
    #[instrument]
    pub async fn delete(&self, bucket_name: String, key: String) -> Result<(), Error> {
        let client = Client::new(&self.config);

        match client
            .delete_object()
            .set_key(Some(key.clone()))
            .set_bucket(Some(bucket_name))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = e.into_service_error();
                let msg = msg.message().unwrap();
                println!("{}", msg);
                Err(Error::S3(msg.to_string()))
            }
        }
    }
}
