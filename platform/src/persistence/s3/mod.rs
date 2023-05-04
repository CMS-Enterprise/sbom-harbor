use crate::config::sdk_config_from_env;
use aws_sdk_s3::error::PutObjectError;
use aws_sdk_s3::types::{ByteStream, DisplayErrorContext};
use aws_sdk_s3::Client;
use aws_types::SdkConfig;
use std::collections::HashMap;
use tracing::instrument;

use crate::hyper::format_header_name;
use crate::Error;

/// Provides a coarse-grained abstraction over S3 that conforms to the conventions of this crate.
#[derive(Debug)]
pub struct Store {
    config: SdkConfig,
}

/// Custom S3Error type.
#[derive(Debug)]
#[allow(dead_code)]
pub struct S3Error {
    message: String,
    code: String,
    request_id: String,
}

impl From<PutObjectError> for S3Error {
    fn from(error: PutObjectError) -> Self {
        Self {
            message: match error.message() {
                None => "not_set".to_string(),
                Some(m) => m.to_string(),
            },
            code: match error.code() {
                None => "not_set".to_string(),
                Some(c) => c.to_string(),
            },
            request_id: match error.request_id() {
                None => "not_set".to_string(),
                Some(r) => r.to_string(),
            },
        }
    }
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
        let metadata = match metadata {
            Some(incoming) => {
                let mut result = HashMap::<String, String>::new();

                for (k, v) in incoming.iter() {
                    result.insert(format_header_name(k), v.to_string());
                }

                Some(result)
            }
            None => None,
        };

        let client = Client::new(&self.config);
        let body = ByteStream::from(body);

        // TODO: Come back to checksum handling.
        match client
            .put_object()
            .bucket(bucket_name.clone())
            .key(key.clone())
            .body(body)
            .set_metadata(metadata)
            //.set_checksum_sha256(checksum_256.clone())
            .send()
            .await
        {
            Ok(_result) => Ok(()),
            Err(e) => {
                let msg = DisplayErrorContext(&e);
                println!("{}", msg);
                Err(Error::S3(format!("{:#?}", msg)))
            }
        }

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
