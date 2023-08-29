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
        let results_per_page = 50;

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

    #[test]
    fn json_test() {
        let resp_body = "{\"resultsPerPage\":2,\"startIndex\":0,\"totalResults\":223594,\"format\":\"NVD_CVE\",\"version\":\"2.0\",\"timestamp\":\"2023-08-29T10:34:23.710\",\"vulnerabilities\":[{\"cve\":{\"id\":\"CVE-1999-0095\",\"sourceIdentifier\":\"cve@mitre.org\",\"published\":\"1988-10-01T04:00:00.000\",\"lastModified\":\"2019-06-11T20:29:00.263\",\"vulnStatus\":\"Modified\",\"descriptions\":[{\"lang\":\"en\",\"value\":\"The debug command in Sendmail is enabled, allowing attackers to execute commands as root.\"},{\"lang\":\"es\",\"value\":\"El comando de depuración de Sendmail está activado, permitiendo a atacantes ejecutar comandos como root.\"}],\"metrics\":{\"cvssMetricV2\":[{\"source\":\"nvd@nist.gov\",\"type\":\"Primary\",\"cvssData\":{\"version\":\"2.0\",\"vectorString\":\"AV:N\\/AC:L\\/Au:N\\/C:C\\/I:C\\/A:C\",\"accessVector\":\"NETWORK\",\"accessComplexity\":\"LOW\",\"authentication\":\"NONE\",\"confidentialityImpact\":\"COMPLETE\",\"integrityImpact\":\"COMPLETE\",\"availabilityImpact\":\"COMPLETE\",\"baseScore\":10.0},\"baseSeverity\":\"HIGH\",\"exploitabilityScore\":10.0,\"impactScore\":10.0,\"acInsufInfo\":false,\"obtainAllPrivilege\":true,\"obtainUserPrivilege\":false,\"obtainOtherPrivilege\":false,\"userInteractionRequired\":false}]},\"weaknesses\":[{\"source\":\"nvd@nist.gov\",\"type\":\"Primary\",\"description\":[{\"lang\":\"en\",\"value\":\"NVD-CWE-Other\"}]}],\"configurations\":[{\"nodes\":[{\"operator\":\"OR\",\"negate\":false,\"cpeMatch\":[{\"vulnerable\":true,\"criteria\":\"cpe:2.3:a:eric_allman:sendmail:5.58:*:*:*:*:*:*:*\",\"matchCriteriaId\":\"1D07F493-9C8D-44A4-8652-F28B46CBA27C\"}]}]}],\"references\":[{\"url\":\"http:\\/\\/seclists.org\\/fulldisclosure\\/2019\\/Jun\\/16\",\"source\":\"cve@mitre.org\"},{\"url\":\"http:\\/\\/www.openwall.com\\/lists\\/oss-security\\/2019\\/06\\/05\\/4\",\"source\":\"cve@mitre.org\"},{\"url\":\"http:\\/\\/www.openwall.com\\/lists\\/oss-security\\/2019\\/06\\/06\\/1\",\"source\":\"cve@mitre.org\"},{\"url\":\"http:\\/\\/www.osvdb.org\\/195\",\"source\":\"cve@mitre.org\"},{\"url\":\"http:\\/\\/www.securityfocus.com\\/bid\\/1\",\"source\":\"cve@mitre.org\"}]}},{\"cve\":{\"id\":\"CVE-1999-0082\",\"sourceIdentifier\":\"cve@mitre.org\",\"published\":\"1988-11-11T05:00:00.000\",\"lastModified\":\"2008-09-09T12:33:40.853\",\"vulnStatus\":\"Analyzed\",\"descriptions\":[{\"lang\":\"en\",\"value\":\"CWD ~root command in ftpd allows root access.\"}],\"metrics\":{\"cvssMetricV2\":[{\"source\":\"nvd@nist.gov\",\"type\":\"Primary\",\"cvssData\":{\"version\":\"2.0\",\"vectorString\":\"AV:N\\/AC:L\\/Au:N\\/C:C\\/I:C\\/A:C\",\"accessVector\":\"NETWORK\",\"accessComplexity\":\"LOW\",\"authentication\":\"NONE\",\"confidentialityImpact\":\"COMPLETE\",\"integrityImpact\":\"COMPLETE\",\"availabilityImpact\":\"COMPLETE\",\"baseScore\":10.0},\"baseSeverity\":\"HIGH\",\"exploitabilityScore\":10.0,\"impactScore\":10.0,\"acInsufInfo\":false,\"obtainAllPrivilege\":true,\"obtainUserPrivilege\":false,\"obtainOtherPrivilege\":false,\"userInteractionRequired\":false}]},\"weaknesses\":[{\"source\":\"nvd@nist.gov\",\"type\":\"Primary\",\"description\":[{\"lang\":\"en\",\"value\":\"NVD-CWE-Other\"}]}],\"configurations\":[{\"nodes\":[{\"operator\":\"OR\",\"negate\":false,\"cpeMatch\":[{\"vulnerable\":true,\"criteria\":\"cpe:2.3:a:ftp:ftp:*:*:*:*:*:*:*:*\",\"matchCriteriaId\":\"30D7F58F-4C55-4D19-984C-79B6C9525BEB\"},{\"vulnerable\":true,\"criteria\":\"cpe:2.3:a:ftpcd:ftpcd:*:*:*:*:*:*:*:*\",\"matchCriteriaId\":\"1D85A7F5-C187-4707-8681-F96A91F58318\"}]}]}],\"references\":[{\"url\":\"http:\\/\\/www.alw.nih.gov\\/Security\\/Docs\\/admin-guide-to-cracking.101.html\",\"source\":\"cve@mitre.org\"}]}}]}";
        // let resp_body = "{}";
        let result = match serde_json::from_slice(resp_body.as_ref()) {
            Ok(r) => {
                println!("Got Response: {:#?}", r);
                r
            },
            Err(err) => {
                println!("!!!! Error: {}", err);
                Value::Object(Map::new())
            }
        };

        let ds: Option<NvdVulnerabilityV2> = serde_json::from_value(result)
            .expect("Error converting from value");

        println!("{:#?}", ds);
    }

}