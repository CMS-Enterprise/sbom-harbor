use crate::Error;
use platform::hyper;
use platform::hyper::ContentType;
use crate::clients::snyk::models::{CommonIssueModel, Group, IssuesResponse, ListOrgProjects200Response, ListOrgProjects200ResponseDataInner, Org, OrgsResponse, OrgV1, SbomResource, SbomResponse};

const V1_URL:&str = "https://snyk.io/api/v1";
const V3_URL:&str = "https://api.snyk.io/rest";
const ORGS_ROUTE: &str = "/orgs";
#[allow(dead_code)]
const ACCEPT:&str = "Accept: application/vnd.cyclonedx+json";

#[allow(dead_code)]
fn orgs_url() -> String {
    format!("{}{}", V1_URL, ORGS_ROUTE)
}

#[allow(dead_code)]
fn org_issues_url(org_id: &str) -> String {
    let route = format!("/orgs/{}/packages/issues", org_id);
    format!("{}{}", V3_URL, route)
}
#[allow(dead_code)]
fn issues_url(org_id: &str, purl: &str) -> String {
    let route = format!("/orgs/{}/packages/{}/issues", org_id, purl);
    format!("{}{}", V3_URL, route)
}

#[allow(dead_code)]
fn projects_url(org_id: &str) -> String {
    let route = format!("/orgs/{}/projects?version=2023-03-08~beta", org_id);
    format!("{}{}", V3_URL, route)
}

#[allow(dead_code)]
fn sbom_url(org_id: &str, project_id: &str) -> String {
    let route = format!("/orgs/{}/projects/{}/sbom", org_id, project_id);
    format!("{}{}", V3_URL, route)
}

/// A purpose build Snyk HTTP Client.
pub(crate) struct Client {
    token: String,
}

impl Client {
    /// Factory method for creating new instances of a Client.
    pub fn new(token: String) -> Self {
        Self{token}
    }

    fn token(&self) -> String {
        format!("token {}", self.token)
    }

    #[allow(dead_code)]
    pub async fn orgs(&self) -> Result<Option<Vec<OrgV1>>, Error> {
        let response:Option<OrgsResponse> = hyper::get(
            &orgs_url(),
            ContentType::Json,
            &self.token(),
            None::<OrgsResponse>).await?;

        match response {
            None => Err(Error::Runtime("snyk failed to list orgs".to_string())),
            Some(r) => Ok(r.orgs),
        }

    }

    #[allow(dead_code)]
    pub async fn projects(&self, org_id: &str) -> Result<Option<Vec<ListOrgProjects200ResponseDataInner>>, Error> {
        let response:Option<ListOrgProjects200Response> = hyper::get(
            &projects_url(org_id),
            ContentType::Json,
            &self.token(),
            None::<ListOrgProjects200Response>).await?;

        match response {
            None => Err(Error::Runtime("snyk failed to list projects".to_string())),
            Some(r) => Ok(r.data),
        }
    }

    #[allow(dead_code)]
    pub async fn sbom(&self, org_id: &str, project_id: &str) -> Result<SbomResource, Error> {
        let response:Option<SbomResponse> = hyper::get(
            &sbom_url(org_id, project_id),
            ContentType::Json,
            &self.token(),
            None::<SbomResponse>).await?;

        match response {
            None => Err(Error::Runtime("snyk failed to generate SBOM".to_string())),
            Some(r) => Ok(*r.data),
        }

    }

    #[allow(dead_code)]
    pub async fn get_org_issues(&self, org_id: &str) -> Result<Option<Vec<CommonIssueModel>>, Error> {
        let response:Option<IssuesResponse> = hyper::get(
            &org_issues_url(org_id),
                ContentType::Json,
            &self.token(),
            None::<IssuesResponse>).await?;

        match response {
            None => Err(Error::Runtime("snyk failed to return org issues".to_string())),
            Some(r) => Ok(r.data),
        }
    }

    #[allow(dead_code)]
    pub async fn get_issues(&self, org_id: &str, purl: &str) -> Result<Option<Vec<CommonIssueModel>>, Error> {
        let response:Option<IssuesResponse> = hyper::get(
            &issues_url(org_id, purl),
                ContentType::Json,
            &self.token(),
            None::<IssuesResponse>).await?;

        match response {
            None => Err(Error::Runtime("snyk failed to return issues".to_string())),
            Some(r) => Ok(r.data),
        }
    }
}

