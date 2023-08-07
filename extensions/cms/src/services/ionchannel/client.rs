use harbcore::entities::cyclonedx::Bom;
use harbcore::entities::sboms::CdxFormat;
use platform::hyper::{ContentType, Method, StatusCode};
use platform::json;
use platform::json::sanitize_ndjson;
use platform::persistence::mongodb::MongoDocument;
use platform::testing::persistence::mongodb::DebugEntity;
use platform::{hyper, mongo_doc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::Error;

/// A purpose built Ion Channel HTTP Client.
#[derive(Debug)]
pub struct Client {
    token: String,
    org_id: String,
    inner: hyper::Client,
}

impl Client {
    /// Factory method for creating new instances of a Client.
    pub fn new(token: String, org_id: String) -> Self {
        let inner = hyper::Client::new();
        Self {
            token,
            org_id,
            inner,
        }
    }

    fn token(&self) -> String {
        format!("Bearer {}", self.token)
    }

    /// Get a list of Ion Channel SBOM entities.
    pub async fn sboms(&self) -> Result<SbomsResponse, Error> {
        let uri = format!(
            "https://api.ionchannel.io/v1/project/getSBOMs?org_id={}",
            self.org_id
        );

        let response = self
            .inner
            .get(
                uri.as_str(),
                ContentType::Json,
                &self.token(),
                None::<String>,
            )
            .await
            .map_err(|e| Error::IonChannel(e.to_string()))?;

        match response {
            None => Err(Error::IonChannel(
                "ion-channel failed to list sboms".to_string(),
            )),
            Some(r) => Ok(r),
        }
    }

    pub async fn sbom(&self, id: &str) -> Result<SbomResponse, Error> {
        let response: (StatusCode, String) = self
            .inner
            .raw(
                Method::POST,
                "https://api.ionchannel.io/v1/report/getSBOM?sbom_type=CycloneDX&include_dependencies=true&team_top_level=false",
                ContentType::Json,
                &self.token(),
                Some(SbomRequest {
                    sbom_id: id.to_string(),
                }),
            )
            .await
            .map_err(|e| Error::IonChannel(e.to_string()))?;

        if response.0 != StatusCode::OK {
            return Err(Error::IonChannel(response.0.to_string()));
        }

        let obj = json!(response.1);
        let _data = obj[0].clone();
        let raw = match obj[0]["data"].as_str() {
            None => return Err(Error::IonChannel("sbom_data_none".to_string())),
            Some(e) => e,
        };

        // let raw = sanitize_ndjson(raw).map_err(|e| Error::IonChannel(e.to_string()))?;

        let bom = Bom::parse(raw, CdxFormat::Json).map_err(|e| Error::IonChannel(e.to_string()))?;

        let response = SbomResponse { data: Some(bom) };

        Ok(response)
    }

    pub async fn metrics(
        &self,
        repo_id: Option<&str>,
        package_id: Option<&str>,
        product_id: Option<&str>,
    ) -> Result<MetricsResponse, Error> {
        let token = self.token();
        let response: Option<MetricsResponse> = self
            .inner
            .post(
                "https://api.ionchannel.io/v1/score/getMetricsForEntity",
                ContentType::Json,
                token.as_str(),
                Some(MetricsRequest {
                    repo_id: repo_id.map(|id| id.to_string()),
                    package_id: package_id.map(|id| id.to_string()),
                    product_id: product_id.map(|id| id.to_string()),
                }),
            )
            .await
            .map_err(|e| Error::IonChannel(e.to_string()))?;

        match response {
            None => Err(Error::IonChannel(
                "ion-channel failed to list metrics".to_string(),
            )),
            Some(r) => Ok(r),
        }
    }

    pub async fn debug_metrics(
        &self,
        repo_id: Option<&str>,
        package_id: Option<&str>,
        product_id: Option<&str>,
    ) -> Result<DebugEntity, Error> {
        let token = self.token();
        let (status_code, body) = self
            .inner
            .raw(
                Method::POST,
                "https://api.ionchannel.io/v1/score/getMetricsForEntity",
                ContentType::Json,
                token.as_str(),
                Some(MetricsRequest {
                    repo_id: repo_id.map(|id| id.to_string()),
                    package_id: package_id.map(|id| id.to_string()),
                    product_id: product_id.map(|id| id.to_string()),
                }),
            )
            .await
            .map_err(|e| Error::IonChannel(e.to_string()))?;

        if status_code != StatusCode::OK {
            return Err(Error::IonChannel(
                "ion-channel failed to list metrics".to_string(),
            ));
        }

        Ok(DebugEntity {
            id: "".to_string(),
            kind: Some("ionchannel".to_string()),
            data: Some(body),
        })
    }
}

/// Response to a SBOMS_ENDPOINT request.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct SbomsResponse {
    pub data: Option<SbomsResult>,
}

/// Data structure returned by a SbomResponse.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct SbomsResult {
    pub software_lists: Option<Vec<Software>>,
}

// See https://docs.ionchannel.io/v1/docs/sbom-endpoints#get-sboms for schema.
/// A software entity defined in the Ion Channel back end.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct Software {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub monitor_frequency: Option<String>,
    pub status: Option<String>,
}

/// Request to the SBOM_ENDPOINT.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct SbomRequest {
    pub sbom_id: String,
}

/// Response to a SBOM_ENDPOINT request.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct SbomResponse {
    pub data: Option<Bom>,
}

/// Request to the METRICS_ENDPOINT.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct MetricsRequest {
    /// Purl with repository encoded.
    pub repo_id: Option<String>,
    /// Canonical purl.
    pub package_id: Option<String>,
    /// CPE id.
    pub product_id: Option<String>,
}

/// Response from the METRICS_ENDPOINT.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct MetricsResponse {
    pub(crate) data: Option<MetricsResult>,
}

impl MetricsResponse {
    /// Consistently handles unwrap error handling for callers.
    pub fn to_metrics(&self) -> Result<Vec<Metric>, Error> {
        let result = match &self.data {
            None => {
                return Err(Error::IonChannel("metrics_data_empty".to_string()));
            }
            Some(r) => r,
        };

        let metrics = match &result.metrics {
            None => {
                return Err(Error::IonChannel("metrics_empty".to_string()));
            }
            Some(m) => m,
        };

        if metrics.is_empty() {
            return Err(Error::IonChannel("metrics_empty".to_string()));
        }

        Ok(metrics.clone())
    }
}

/// Metrics set returned from the METRICS_ENDPOINT.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct MetricsResult {
    /// Unique identifier passed in. One of repo_id, package_id, product_id.
    pub id: Option<String>,
    /// Vector of metric instances.
    pub metrics: Option<Vec<Metric>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct ProductCount {
    count: Option<u16>,
    source: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[skip_serializing_none]
#[serde(untagged)]
pub enum MetricValue {
    Float(f64),
    Int(u16),
    ProductCountVector(Vec<ProductCount>),
}

/// Individual Metric item returned from the METRICS_ENDPOINT.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct Metric {
    /// Harbor data store id of the metric.
    pub id: Option<String>,
    /// Purl the metric applies to.
    pub purl: Option<String>,
    /// Name of the metric.
    pub name: Option<String>,
    /// Metric value.
    pub value: Option<MetricValue>,
    /// Data type of the metric value.
    pub r#type: Option<String>,
    /// Metadata that describes the metric and what it applies to.
    pub bindings: Option<Vec<Binding>>,
    /// Severity of the metric.
    pub severity: Option<String>,
    /// Rank of the severity of the metric.
    pub severity_rank: Option<u16>,
    /// Data sources that were incorporated into the metric.
    pub sources: Option<Vec<String>>,
    /// Vector of related metrics.
    pub related_metrics: Option<Vec<RelatedMetric>>,
}

impl MongoDocument for Metric {
    fn id(&self) -> String {
        match &self.id {
            None => "".to_string(),
            Some(id) => id.clone(),
        }
    }

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    fn type_name() -> String {
        format!("{}", std::any::type_name::<Metric>())
    }

    fn collection() -> String {
        let type_name = Self::type_name();
        type_name.split(':').next_back().unwrap().to_string()
    }
}

/// Individual Metric item returned from the METRICS_ENDPOINT.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct RelatedMetric {
    /// Description of the related metric.
    pub name: Option<String>,
    /// Field name of the related metric.
    pub internal_name: Option<String>,
    /// Severity of the related metric.
    pub severity: Option<String>,
}

/// Metric metadata as dynamic  returned from the METRICS_ENDPOINT.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct Binding {
    /// Name of the binding.
    pub metric: Option<String>,
    /// Scope the metric gets applied to in terms of Ion Channel's custom metrics.
    pub scope: Option<String>,
    /// Category of the metric.
    pub category: Option<String>,
    /// Attribute of the binding.
    pub attribute: Option<String>,
    /// Data source of the metric.
    pub source: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::ionchannel::client::Client;
    use crate::Error;
    use harbcore::config::{ion_channel_org_id, ion_channel_token};

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_get_sboms() -> Result<(), Error> {
        let token = ion_channel_token().map_err(|e| Error::Config(e.to_string()))?;
        let org_id = ion_channel_org_id().map_err(|e| Error::Config(e.to_string()))?;

        let client = Client::new(token, org_id);
        let result = client.sboms().await?;
        let data: SbomsResult = result.data.expect("data is none");
        let softwares = data.software_lists.expect("software_lists is none");

        for software in softwares.iter() {
            assert!(software.id.is_some() && !software.id.clone().unwrap().is_empty());
        }

        Ok(())
    }

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_get_sbom() -> Result<(), Error> {
        let token = ion_channel_token().map_err(|e| Error::Config(e.to_string()))?;
        let org_id = ion_channel_org_id().map_err(|e| Error::Config(e.to_string()))?;

        let client = Client::new(token, org_id);
        let result = client.sboms().await?;
        let data: SbomsResult = result.data.expect("data is none");
        let softwares = data.software_lists.expect("software_lists is none");

        let mut passed = false;

        // Validate that at least one available SBOM has components.
        for software in softwares.iter() {
            let sbom_id = software.id.clone().unwrap();

            let sbom = client.sbom(sbom_id.as_str()).await?;
            let bom = sbom.data.expect("bom is empty");
            if bom.metadata.is_some() {
                passed = true;
                break;
            }
        }

        assert!(passed);

        Ok(())
    }

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_get_metrics() -> Result<(), Error> {
        let token = ion_channel_token().map_err(|e| Error::Config(e.to_string()))?;
        let org_id = ion_channel_org_id().map_err(|e| Error::Config(e.to_string()))?;

        let client = Client::new(token, org_id);
        let result = client.sboms().await?;
        let data: SbomsResult = result.data.expect("data is none");
        let softwares = data.software_lists.expect("software_lists is empty");

        for software in softwares.iter() {
            let sbom_id = software.id.clone().expect("sbom_id is none");
            let sbom = client.sbom(sbom_id.as_str()).await?;
            let bom = sbom.data.expect("bom is none");
            let components = bom.components.expect("components is none");

            let mut passed = false;
            for component in components.iter() {
                let purl = match &component.purl {
                    None => continue,
                    Some(p) => {
                        if p.is_empty() {
                            continue;
                        }
                        p.as_str()
                    }
                };

                let result = match client.metrics(None, Some(purl), None).await {
                    Ok(response) => response.data.expect("metrics result is none"),
                    Err(_) => continue,
                };

                let metrics = result.metrics.expect("metrics is none");
                assert!(!metrics.is_empty());

                for metric in metrics.iter() {
                    if metric.name.is_some() {
                        passed = true;
                        break;
                    }
                }
            }

            assert!(passed)
        }

        Ok(())
    }
}
