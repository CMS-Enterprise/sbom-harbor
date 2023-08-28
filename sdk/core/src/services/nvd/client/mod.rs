use thiserror::Error;
use platform::hyper::ContentType;
use crate::services::nvd::client::models::NvdVulnerabilityV2;
use platform::hyper::Client as HttpClient;
use platform::hyper::token::Token;

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

    pub async fn get_page_of_vulnerabilities(&self) -> Result<Vec<NvdVulnerabilityV2>, Error> {

        let start_index = 0;
        let results_per_page = 2;

        let url = format!("{}/?startIndex={}&resultsPerPage={}",
              BASE_URL, start_index, results_per_page);

        let option: Option<Vec<NvdVulnerabilityV2>> = self
            .http_client
            .get(
                url.as_str(),
                ContentType::Json,
                Some(Token::new(self.api_key.clone())),
                None::<String>,
            )
            .await
            .map_err(Error::NvdResponse)?;

        Ok(vec![])
    }

}

#[derive(Error, Debug)]
pub enum Error {
    /// Error derived from our Http Client
    #[error(transparent)]
    NvdResponse(#[from] platform::hyper::Error)
}