use thiserror::Error;
use platform::hyper::ContentType;
use crate::services::nvd::client::models::NvdVulnerabilityV2;
use platform::hyper::Client as HttpClient;
use platform::hyper::token::Token;
use crate::services::nvd::client::Error::NvdClientError;

/// NVD response specific models
mod models;

const BASE_URL: &str = "https://services.nvd.nist.gov/rest/json/cves/2.0";

pub struct Client {
    http_client: HttpClient,
    api_key: String,
}

impl Client {

    pub fn new(http_client: HttpClient, api_key: String) -> Self {
        Self {
            http_client,
            api_key
        }
    }

    pub async fn get_page_of_vulnerabilities(&self) -> Result<NvdVulnerabilityV2, Error> {

        let start_index = 0;
        let results_per_page = 2;

        let url = format!("{}/?startIndex={}&resultsPerPage={}",
              BASE_URL, start_index, results_per_page);

        let option: Option<NvdVulnerabilityV2> = self
            .http_client
            .get(
                url.as_str(),
                ContentType::Json,
                Some(
                    Token::new_with_header_name(
                        String::from("apiKey"),
                        self.api_key.clone()
                    )
                ),
                None::<String>,
            )
            .await
            .map_err(Error::NvdResponse)?;

        match option {
            None => Err(
                NvdClientError(
                    String::from("Response form NVD is empty")
                )
            ),
            Some(nvd_response) => Ok(nvd_response),
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    /// Error derived from our Http Client
    #[error(transparent)]
    NvdResponse(#[from] platform::hyper::Error),

    #[error("vulnerability provider error: {0}")]
    NvdClientError(String)
}

#[cfg(test)]
mod tests {
    use serde_json::{Map, Value};
    use crate::config::nvd_api_key;
    use crate::services::nvd::client::Client;
    use crate::services::nvd::client::Error;
    use platform::hyper::Client as HttpClient;
    use crate::services::nvd::client::models::NvdVulnerabilityV2;


    #[tokio::test]
    async fn test_get_page_of_vulnerabilities() -> Result<(), Error> {
        let key = nvd_api_key().unwrap();
        let client = Client::new(HttpClient::new(), key);
        let page = client.get_page_of_vulnerabilities().await?;
        println!("{:#?}", page);
        Ok(())
    }
}