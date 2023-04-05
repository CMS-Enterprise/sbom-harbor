use std::collections::HashMap;

use platform::hyper::{ContentType, Error as HyperError, get, post};
use serde_json::Value;
use crate::clients::ProjectJson;

use super::{SnykData, Org};

//URL used for getting org and project details
const API_V1_URL: &'static str = "https://snyk.io/api/v1";
//URL used for getting SBOMS
const API_V3_URL: &'static str = "https://api.snyk.io/rest";
const API_V3_VERSION: &'static str = "2023-02-15~beta";
//Format of SBOMs returned from Snyk
const SBOM_FORMAT: &'static str = "cyclonedx%2Bjson";

/// Retrieves a list of all Orgs from Snyk and adds them into a new SnykData object
pub async fn get_orgs(snyk_token: String) -> Vec<Option<Org>>{
    println!("Getting list of Orgs from Snyk...");

    let url = format!("{}/orgs", API_V1_URL);
    let response: Result<Option<SnykData>, HyperError> = get(
        url.as_str(),
        ContentType::Json,
        snyk_token.as_str(),
        None::<String>,
    ).await;

    match response {
        Ok(option) => 
            match option {
            Some(value) => {return value.orgs},
            None => todo!(), //TODO: fix this!!
        },
        Err(err) => panic!("Error in the response: {}", err), //TODO: fix this!!
    }
}

/// Returns a ProjectJson object that contains a list of all projects for an org
pub async fn get_projects_from_org(snyk_token: String, org_id: String, org_name: String) -> ProjectJson{

    println!("Retrieving project info for Org: {}, with ID: {}", org_name, org_id);
    let url = format!("{}/org/{}/projects", API_V1_URL, org_id);
    let response: Result<Option<ProjectJson>, HyperError> = post(
        url.as_str(),
        ContentType::Json,
        snyk_token.as_str(),
        None::<String>,
    ).await;

    match response {
        Ok(option) =>  {
            match option {
                Some(project_json) => return project_json,
                None => todo!(), //TODO: fix this!!
            }
        },
        Err(err) => panic!("Error in the response: {}", err), //TODO: fix this!!
    }
}

/// Returns a list of ProjectJson objects that each contain a list of projects for their specific org
pub async fn get_projects_from_org_list(snyk_token: String, orgs: Vec<Option<Org>>) -> Vec<ProjectJson>{

    let mut all_projects: Vec<ProjectJson> = Vec::new();
    for org in orgs.iter() {
        match org {
            Some(valid_org) => {
                let org_id = valid_org.id.clone().unwrap_or_else(|| "Missing".to_string());
                let org_name = valid_org.name.clone().unwrap_or_else(|| "Missing".to_string());
                let org_projects = get_projects_from_org(snyk_token.clone(), org_id, org_name).await;
                all_projects.push(org_projects);
            },
            None => todo!(), //TODO: fix this!!
        }
    }
    return all_projects;
}

/// Attempts to get an SBOM from the SNYK Api from a org and project ID
pub async fn get_sbom_from_snyk(snyk_token: String, org_id: String, proj_id: String) -> Option<HashMap<String, Value>> {
    let sbom_url = format!("{}/orgs/{}/projects/{}/sbom?version={}&format={}", API_V3_URL, org_id, proj_id, API_V3_VERSION, SBOM_FORMAT);
    println!("Fetching sbom for project: {}, in Org: {}", proj_id, org_id);
    println!("with url: {}", sbom_url);
    //TODO: rest call here?
    let response: Result<Option<HashMap<String, Value>>, HyperError> = get(
        &sbom_url,
        ContentType::Json,
        snyk_token.as_str(),
        None::<String>,
    ).await;

    match response {
        Ok(sbom) => return sbom,
        Err(_) => todo!(),//TODO: fix this!!
    }
}

#[tokio::test]
async fn test_get_orgs() {
}

#[tokio::test]
async fn test_get_projects_from_org() {
}

#[tokio::test]
async fn test_get_projects_from_org_list() {
}

#[tokio::test]
async fn test_get_sbom_from_snyk() {
}