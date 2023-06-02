use crate::Error;
use platform::hyper::ContentType;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(in crate::services::enrichments::vulnerabilities::epss) struct Client {
    inner: platform::hyper::Client,
}

impl Client {
    pub fn new() -> Client {
        let inner = platform::hyper::Client::new();

        Client { inner }
    }

    pub async fn score(&self, cve: String) -> Result<f32, Error> {
        let uri = format!("https://api.first.org/data/v1/epss?cve={}", cve);

        let response: Option<EpssResponse> = self
            .inner
            .get(uri.as_str(), ContentType::Json, "", None::<EpssResponse>)
            .await?;

        let response = match response {
            None => {
                return Err(Error::Vulnerability("epss_score_none".to_string()));
            }
            Some(response) => response,
        };

        if response.status_code != 200 {
            return Err(Error::Vulnerability(format!(
                "epss_status_code::{}",
                response.status_code
            )));
        }

        let score = match response.data.iter().find(|score| score.cve == cve) {
            None => {
                return Err(Error::Vulnerability("epss_score_not_found".to_string()));
            }
            Some(score) => &score.epss,
        };

        score
            .parse::<f32>()
            .map_err(|e| Error::InvalidFormat(format!("epss_score::{}", e)))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct EpssResponse {
    #[serde(rename = "status-code")]
    status_code: u8,
    data: Vec<EpssScore>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct EpssScore {
    cve: String,
    epss: String,
    percentile: String,
    date: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[async_std::test]
    async fn can_get_epss_score() -> Result<(), Error> {
        let client = Client::new();

        let score = client
            .score("CVE-2021-40438".to_string())
            .await
            .map_err(|e| Error::Vulnerability(e.to_string()))?;

        assert_ne!(0.0 as f32, score);

        Ok(())
    }
}
