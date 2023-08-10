/// Rust structs that represent the models/schemas relevant to the endpoints the client supports from
/// the Snyk OpenAPI specification.
pub mod models;
mod rawclient;

use tracing::debug;

use platform::encoding::url_encode;
use platform::hyper;
use platform::hyper::ContentType;

use crate::services::snyk::{SbomFormat, API_VERSION};
use crate::Error;
use models::{
    CommonIssueModel, IssuesResponse, ListOrgProjects200Response,
    ListOrgProjects200ResponseDataInner, OrgV1, OrgsResponse,
};

const V1_URL: &str = "https://snyk.io/api/v1";
const V3_URL: &str = "https://api.snyk.io/rest";
const ORGS_ROUTE: &str = "/orgs";

pub fn orgs_url() -> String {
    format!("{}{}", V1_URL, ORGS_ROUTE)
}

fn issues_url(org_id: &str, purl: &str) -> String {
    let route = format!(
        "/orgs/{}/packages/{}/issues?version={}",
        org_id,
        url_encode(purl),
        API_VERSION
    );

    format!("{}{}", V3_URL, route)
}

pub fn projects_url(org_id: &str) -> String {
    let route = format!("/orgs/{}/projects?version={}", org_id, API_VERSION);
    format!("{}{}", V3_URL, route)
}

fn sbom_url(org_id: &str, project_id: &str, format: SbomFormat) -> String {
    let route = format!(
        "/orgs/{}/projects/{}/sbom?version={}&format={}",
        org_id, project_id, API_VERSION, format
    );
    format!("{}{}", V3_URL, route)
}

/// A purpose build Snyk HTTP Client.
#[derive(Debug)]
pub struct Client {
    token: String,
    inner: hyper::Client,
}

impl Client {
    /// Factory method for creating new instances of a Client.
    pub fn new(token: String) -> Self {
        let inner = hyper::Client::new();
        Self { token, inner }
    }

    fn token(&self) -> String {
        format!("token {}", self.token)
    }

    pub async fn orgs(&self) -> Result<Option<Vec<OrgV1>>, Error> {
        let response: Option<OrgsResponse> = self
            .inner
            .get(
                &orgs_url(),
                ContentType::Json,
                &self.token(),
                None::<OrgsResponse>,
            )
            .await?;

        match response {
            None => Err(Error::Runtime("snyk failed to list orgs".to_string())),
            Some(r) => Ok(r.orgs),
        }
    }

    pub async fn projects(
        &self,
        org_id: &str,
    ) -> Result<Option<Vec<ListOrgProjects200ResponseDataInner>>, Error> {
        let response: Option<ListOrgProjects200Response> = self
            .inner
            .get(
                &projects_url(org_id),
                ContentType::Json,
                &self.token(),
                None::<ListOrgProjects200Response>,
            )
            .await?;

        match response {
            None => Err(Error::Runtime("snyk failed to list projects".to_string())),
            Some(r) => Ok(r.data),
        }
    }

    pub async fn sbom_raw(
        &self,
        org_id: &str,
        project_id: &str,
        format: SbomFormat,
    ) -> Result<Option<String>, Error> {
        let url = &sbom_url(org_id, project_id, format.clone());
        debug!(url);
        let response = self
            .inner
            .raw(
                hyper::Method::GET,
                &sbom_url(org_id, project_id, format),
                ContentType::Json,
                self.token(),
                None::<String>,
            )
            .await?;

        if response.0 != hyper::StatusCode::OK {
            return Err(Error::Runtime(format!(
                "remote returned error: {}",
                response.1
            )));
        }

        if response.1.is_empty() {
            return Ok(None);
        }

        Ok(Some(response.1))
    }

    pub async fn get_issues(
        &self,
        org_id: &str,
        purl: &str,
    ) -> Result<Option<Vec<CommonIssueModel>>, Error> {
        // println!("getting issues for purl: {}", purl);

        let response: Option<IssuesResponse> = self
            .inner
            .get(
                &issues_url(org_id, purl),
                ContentType::Json,
                &self.token(),
                None::<IssuesResponse>,
            )
            .await?;

        match response {
            None => Err(Error::Runtime("snyk failed to return issues".to_string())),
            Some(r) => Ok(r.data),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::services::snyk::client::Client;
    use crate::Error;

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_list_orgs() -> Result<(), Error> {
        let token = std::env::var("SNYK_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let client = Client::new(token);
        let orgs = client.orgs().await?;
        assert!(orgs.is_some());

        let orgs = orgs.unwrap();
        assert!(!orgs.is_empty());

        Ok(())
    }

    #[async_std::test]
    #[ignore = "manual_debug_test"]
    async fn can_list_projects() -> Result<(), Error> {
        let token = std::env::var("SNYK_TOKEN")
            .map_err(|e| Error::Config(e.to_string()))
            .unwrap();

        let client = Client::new(token);
        let orgs = client.orgs().await?;

        let org = orgs.unwrap()[0].clone();
        let org_id = org.id.unwrap();

        let projects = client.projects(&org_id).await?;
        assert!(projects.is_some());

        Ok(())
    }
}
