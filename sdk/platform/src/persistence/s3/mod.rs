use crate::config::sdk_config_from_env;
use crate::Error;
use aws_sdk_s3::error::PutObjectError;
use aws_sdk_s3::types::{ByteStream, DisplayErrorContext};
use aws_sdk_s3::Client;
use aws_types::SdkConfig;
use regex::Regex;
use std::collections::HashMap;
use tracing::instrument;

/// Format a string so that it can be used as an object key in S3.
pub fn make_s3_key_safe(purl: &str) -> Result<String, Error> {
    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
    let result = re.replace_all(purl, "-");
    let mut result = result.as_ref();
    result = result.trim_end_matches('-');

    let mut result = result.trim_start_matches('-').to_string();

    while result.contains("--") {
        result = result.replace("--", "-");
    }

    Ok(result)
}

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
                    let safe_s3_key_name = make_s3_key_safe(k)?;

                    result.insert(safe_s3_key_name, v.to_string());
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

    /// Lists objects in a bucket
    pub async fn list(&self, bucket_name: String) -> Result<Vec<String>, Error> {
        let client = Client::new(&self.config);

        match client
            .list_objects_v2()
            .set_bucket(Some(bucket_name))
            .send()
            .await
        {
            Ok(r) => {
                let mut objects = vec![];
                for obj in r.contents().unwrap_or_default() {
                    objects.push(obj.key().unwrap().to_string())
                }
                Ok(objects)
            }
            Err(e) => {
                let msg = e.into_service_error();
                let msg = msg.message().unwrap();
                Err(Error::S3(msg.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use serde::{Deserialize, Serialize};

    #[test]
    pub fn can_format_purls_for_s3() -> Result<(), Error> {
        let test_cases: Vec<PurlTestCase> =
            serde_json::from_str(TEST_PURLS).map_err(|e| Error::Serde(e.to_string()))?;

        for test_case in test_cases.iter() {
            let result = make_s3_key_safe(test_case.purl.as_str())?;

            println!("{}", result);

            assert!(!result.starts_with('-'));
            assert!(!result.ends_with('-'));
            assert!(!result.contains("--"));
            assert!(!result.contains("@"));
            assert!(!result.contains('/'));
            assert!(!result.contains("//"));
            assert!(!result.contains("."));
            assert!(!result.contains(","));
            assert!(!result.contains("+"));
            assert!(!result.contains("^"));
            assert!(!result.contains("#"));
            assert!(!result.contains("$"));
            assert!(!result.contains("!"));
            assert!(!result.contains("%"));
            assert!(!result.contains("*"));
            assert!(!result.contains("("));
            assert!(!result.contains(")"));
            assert!(!result.contains("{"));
            assert!(!result.contains("}"));
        }

        Ok(())
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct PurlTestCase {
        purl: String,
    }

    const TEST_PURLS: &str = r#"[{
        "purl": "pkg:npm/foo-api@1.0.0"
        },
        {
        "purl": "pkg:npm/foo-daily-redshift-foo@1.0.0"
        },
        {
        "purl": "pkg:npm/foo@0.0.0"
        },
        {
        "purl": "pkg:npm/foo-bar-baz@1.0.0"
        },
        {
        "purl": "pkg:npm/foo@1.0.0"
        },
        {
        "purl": "pkg:npm/%40sfoo/kubernetes-monitor"
        },
        {
        "purl": "pkg:npm/foo-static-site@1.0.0"
        },
        {
        "purl": "pkg:npm/foo-adminv2@0.1.0"
        },
        {
        "purl": "pkg:npm/foo-ui@0.1.0"
        },
        {
        "purl": "pkg:npm/foo-saf@1.0.0"
        },
        {
        "purl": "pkg:npm/vue-project@0.1.0"
        },
        {
        "purl": "pkg:maven/bar_baz/foo_138_master//aws/apache-jmeter-5.5/lib"
        },
        {
        "purl": "pkg:maven/foo_jmeter/bar_138_master//aws/apache-jmeter-5.5/lib/ext"
        },
        {
        "purl": "pkg:maven/foo_jmeter/bar//root/.bzt/bar-baz/5.4.3/lib"
        },
        {
        "purl": "pkg:maven/foo_jmeter/taurus//root/.bzt/selenium-baz"
        },
        {
        "purl": "pkg:maven/bar_jmeter/baz//root/.bzt/foo-bar/5.4.3/lib/ext"
        },
        {
        "purl": "pkg:maven/foo/bar//home/baz-user/wf/standalone/blick"
        },        
        {
        "purl": "pkg:maven/foo/bar//baz/newrelic"
        },
        {
        "purl": "pkg:maven/foo/bar//baz/foo-bar/wf/modules/system/layers/base/org/wildfly/transaction/client/main"
        },
        {
        "purl": "pkg:maven/com.hhs.cms.foo/bar-baz.it.blick@0.0.1-SNAPSHOT"
        },
        {
        "purl": "pkg:maven/gov.hhs.cms.any/foo-bar-baz.ui.blick@1.1.20"
        },
        {
        "purl": "pkg:maven/gov.hhs.cms.any/foo-bar-baz.ui.blick@1.1.20"
        },
        {
        "purl": "pkg:maven/gov.hhs.cms.any/foo-bar-baz.it.blick@1.1.20"
        },
        {
        "purl": "pkg:nuget/project"
        },
        {
        "purl": "pkg:npm/single-module-nodejs@1.0.0"
        },
        {
        "purl": "pkg:npm/demo-demo@0.1.0"
        },
        {
        "purl": "pkg:npm/%40foo/7i-baz-lib@0.0.161"
        },
        {
        "purl": "pkg:npm/7i-baz-lib@0.0.0"
        },
        {
        "purl": "pkg:npm/foo-api@0.0.126"
        },
        {
        "purl": "pkg:npm/7i-api-lookup-lib@0.0.0"
        },
        {
        "purl": "pkg:npm/foo-lookup-api@0.0.99"
        },
        {
        "purl": "pkg:npm/dksw-sf-rrr-foobar@0.1.0"
        },
        {
        "purl": "pkg:npm/cdk@0.1.0"
        },
        {
        "purl": "pkg:npm/foo-bar-data-runner@0.1.0"
        },
        {
        "purl": "pkg:npm/foo-bar-data-runner"
        },
        {
        "purl": "pkg:npm/foo@5.2.0"
        },
        {
        "purl": "pkg:npm/%40foo-bar/bits-layout@BITS_VERSION"
        },
        {
        "purl": "pkg:maven/org.foo.bar/bar-air-jordan@1.0.69-SNAPSHOT"
        },
        {
        "purl": "pkg:npm/rrg@1.0.0"
        },
        {
        "purl": "pkg:maven/org.foo.bar/FHIR-RGB-UI@8.1.26-SNAPSHOT"
        },
        {
        "purl": "pkg:maven/org.foo.bar/FHIR-RGB-UI@5.1.9"
        },
        {
        "purl": "pkg:composer/drupal/recommended-project@0.0.0"
        },
        {
        "purl": "pkg:composer/drupal/recommended-project@0.0.0"
        },
        {
        "purl": "pkg:maven/prod/foo-to-bar-orchestrator/latest//home/baz/analyzer/.developer/splunk/lib"
        },
        {
        "purl": "pkg:npm/bar-infrastructure@0.0.1"
        },
        {
        "purl": "pkg:npm/qpp-ui@0.0.1"
        },
        {
        "purl": "pkg:npm/qpp-ui@0.0.1"
        },
        {
        "purl": "pkg:npm/qpp-style@9.28.4"
        },
        {
        "purl": "pkg:npm/foo-files@1.0.1-ds.2"
        },
        {
        "purl": "pkg:npm/foo-bar-baz-docker@1.0.0"
        },
        {
        "purl": "pkg:maven/foo/bar-tool/prod/latest//root/.baz/repository/org/springframework/spring-context/4.0.9.RELEASE"
        },
        {
        "purl": "pkg:maven/foo/bar-baz/prod/latest//usr/src/run/newrelic"
        },
        {
        "purl": "pkg:maven/foo/bar-baz/prod/latest//root/.m2/repository/org/springframework/boot/spring-boot-loader-tools/2.2.2.RELEASE"
        }]"#;
}
