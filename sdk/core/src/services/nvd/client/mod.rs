use thiserror::Error;
use platform::hyper::ContentType;
use crate::services::nvd::client::models::NvdVulnerabilityV2;
use platform::hyper::Client as HttpClient;
use platform::hyper::token::Token;
use crate::services::nvd::client::Error::NvdClient;

/// NVD response specific models
pub mod models;

const BASE_URL: &str = "https://services.nvd.nist.gov/rest/json/cves/2.0";

#[derive(Debug, Clone)]
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

    pub async fn get_total_vulnerabilities(&self) -> Result<i32, Error> {
        let response = self.get_page_of_vulnerabilities(0, 1).await?;
        Ok(response.total_results)
    }

    pub async fn get_page_of_vulnerabilities(
        &self,
        start_index: i32,
        results_per_page: i32
    ) -> Result<NvdVulnerabilityV2, Error> {

        let url = format!("{}/?startIndex={}&resultsPerPage={}",
              BASE_URL, start_index, results_per_page);

        println!("Http Call: {}", url);

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
            .map_err(Error::Http)?;

        match option {
            None => Err(
                NvdClient(
                    String::from("Response form NVD is empty")
                )
            ),
            Some(nvd_response) => {
                let tr = nvd_response.total_results;
                let si = nvd_response.start_index;
                let rpp = nvd_response.results_per_page;
                println!("Total Results: {}, Start index: {}, Results Per Page: {}", tr, si, rpp);
                Ok(nvd_response)
            },
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    /// Error derived from our Http Client
    #[error(transparent)]
    Http(#[from] platform::hyper::Error),

    #[error("vulnerability provider error: {0}")]
    NvdClient(String)
}

#[cfg(test)]
mod tests {
    use serde_json::{Map, Value};
    use crate::config::nvd_api_key;
    use crate::services::nvd::client::Client;
    use crate::services::nvd::client::Error;
    use platform::hyper::Client as HttpClient;
    use crate::services::nvd::client::models::{DefCveItem, NvdVulnerabilityV2};


    #[tokio::test]
    async fn test_get_page_of_vulnerabilities() -> Result<(), Error> {

        let num_per_page = 3;

        let key = nvd_api_key().unwrap();
        let client = Client::new(HttpClient::new(), key);
        let page = client.get_page_of_vulnerabilities(0, num_per_page).await?;
        println!("{:#?}", page);

        let vulnerabilities: Vec<DefCveItem> = page.vulnerabilities.unwrap();
        assert_eq!(num_per_page, vulnerabilities.len() as i32);
        Ok(())
    }
}