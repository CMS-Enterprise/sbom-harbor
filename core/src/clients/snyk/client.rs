use crate::Error;
use platform::hyper;
use platform::hyper::ContentType;
use crate::clients::snyk::models::{CommonIssueModel, Group, IssuesResponse, ListOrgProjects200Response, ListOrgProjects200ResponseDataInner, Org, OrgsResponse, OrgV1, SbomResource, SbomResponse};

const V1_URL:&str = "https://snyk.io/api/v1";
const V3_URL:&str = "https://api.snyk.io/rest";
const ORGS_ROUTE: &str = "/orgs";
const ACCEPT:&str = "Accept: application/vnd.cyclonedx+json";

fn orgs_url() -> String {
    format!("{}{}", V1_URL, ORGS_ROUTE)
}

fn issues_url(org_id: &str, purl: &str) -> String {
    format!("{}{}", V3_URL, format!("/orgs/{}/packages/{}/issues", org_id, purl))
}

fn projects_url(org_id: &str) -> String {
    format!("{}{}", V3_URL, format!("/orgs/{}/projects?version=2023-03-08~beta", org_id))
}

fn sbom_url(org_id: &str, project_id: &str) -> String {
    format!("{}{}", V3_URL, format!("/orgs/{}/projects/{}/sbom", org_id, project_id))
}

pub fn org_groups(orgs: Vec<OrgV1>) -> Vec<Group> {
    let mut groups:Vec<Group> = vec![];

    orgs
        .iter()
        .for_each(|org| {
            if org.group.is_some() {
                let group = org.group.as_ref().unwrap();
                let existing = groups
                    .iter()
                    .find(|g| g.id == group.id);

                if existing.is_none() {
                    groups.push(group.clone());
                }
            }
        });

    groups
}

pub struct Client {
    token: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self{token}
    }

    fn token(&self) -> String {
        format!("token {}", self.token)
    }

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

