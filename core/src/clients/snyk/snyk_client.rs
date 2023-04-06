use std::collections::HashMap;

use platform::hyper::{ContentType, Error as HyperError, get, post};
use serde_json::Value;
use crate::clients::{ProjectJson, SnykProviderError};

use super::{SnykData, Org};

//URL used for getting org and project details
const API_V1_URL: &'static str = "https://snyk.io/api/v1";
//URL used for getting SBOMS
const API_V3_URL: &'static str = "https://api.snyk.io/rest";
const API_V3_VERSION: &'static str = "2023-02-15~beta";
//Format of SBOMs returned from Snyk
const SBOM_FORMAT: &'static str = "cyclonedx%2Bjson";

/// Retrieves a list of all Orgs from Snyk and adds them into a new SnykData object
pub async fn get_orgs(snyk_token: String) -> Result<Vec<Org>, SnykProviderError>{
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
            Some(value) => {return Ok(value.orgs)},
            None => {
                println!("No orgs found using URL: {}", url);
                return Err(SnykProviderError::SnykDataValidationError(format!("No orgs found using URL: {}", url)));
            }
        },
        Err(err) => return Err(SnykProviderError::SnykConnection(format!("Error in the response: {}", err)))
    }
}

/// Returns a ProjectJson object that contains a list of all projects for an org
pub async fn get_projects_from_org(snyk_token: String, org_id: String, org_name: String) -> Result<Option<ProjectJson>, SnykProviderError> {

    let url = format!("{}/org/{}/projects", API_V1_URL, org_id);
    let display_msg = format!("----------------------\nretrieving project list \nOrg: {}, \nOrg_ID: {}, \nURL: {}", org_name, org_id, url);

    println!("{}", display_msg);
    let response: Result<Option<ProjectJson>, HyperError> = post(
        url.as_str(),
        ContentType::Json,
        snyk_token.as_str(),
        None::<String>,
    ).await;

    match response {
        Ok(option) => return Ok(option),
        Err(err) => return Err(SnykProviderError::SnykConnection(format!("Error with: {}, \nadditional errors: {}", display_msg, err))),
    }
}

/// Returns a list of ProjectJson objects that each contain a list of projects for their specific org
pub async fn get_projects_from_org_list(snyk_token: String, orgs: Vec<Org>) -> (Vec<ProjectJson>, Vec<SnykProviderError>) {

    let mut all_projects: Vec<ProjectJson> = Vec::new();
    let mut errors: Vec<SnykProviderError> = Vec::new();
    for org in orgs.iter() {
        let org_id = org.id.clone().unwrap_or_else(|| "Missing".to_string());
        let org_name = org.name.clone().unwrap_or_else(|| "Missing".to_string());
        let results = get_projects_from_org(snyk_token.clone(), org_id.clone(), org_name.clone()).await;
        
        match results {
            Ok(option_proj_json) => {
                match option_proj_json {
                    Some(org_proj) => all_projects.push(org_proj),
                    None => {
                        let err_msg = SnykProviderError::SnykNoProjectsFoundError(format!("----------------------\nNo projects found for \n--org_id: {}, \n--org_name: {}", org_id, org_name));
                        errors.push(err_msg);
                    }
                }
            },
            Err(err) => errors.push(err),
        }
    }
    return (all_projects, errors);
}

/// Attempts to get an SBOM from the SNYK Api from a org and project ID
pub async fn get_sbom_from_snyk(snyk_token: String, org_id: String, proj_id: String) -> Result<Option<HashMap<String, Value>>, SnykProviderError> {
    let sbom_url = format!("{}/orgs/{}/projects/{}/sbom?version={}&format={}", API_V3_URL, org_id, proj_id, API_V3_VERSION, SBOM_FORMAT);
    let display_msg = format!("----------------------\nFetching sbom from Snyk \nproject: {}, \nOrg: {}, \nUrl: {}", proj_id, org_id, sbom_url);
    println!("{}", display_msg);
    let response: Result<Option<HashMap<String, Value>>, HyperError> = get(
        &sbom_url,
        ContentType::Json,
        snyk_token.as_str(),
        None::<String>,
    ).await;

    match response {
        Ok(sbom) => return Ok(sbom),
        Err(err) => Err(SnykProviderError::SnykSBOMError(format!("Error with: {}, \nadditional errors: {}", display_msg, err)))
    }
}

#[tokio::test]
async fn test_get_orgs() {
    let fake_token = format!("123-abc");
    //TODO: Stub for rest call
    //TODO: Test for OK results
    //TODO: Test for Error 
}

#[tokio::test]
async fn test_get_projects_from_org() {
    let fake_token = format!("123-abc");
    let fake_org_id = format!("456-ORG");
    let fake_org_name = format!("Test_ORG");
    //TODO: Stub for rest call
    //TODO: Test for OK results
    //TODO: Test for Error 
}

#[tokio::test]
async fn test_get_projects_from_org_list() {
    let fake_token = format!("123-abc");
    let fake_orgs_list: Vec<Org> = Vec::new();
    //TODO: Add fake data to list
    //TODO: Stub for rest call
    //TODO: Test for OK results
    //TODO: Test for Errors 
}

#[tokio::test]
async fn test_get_sbom_from_snyk() {
    let fake_token = format!("123-abc");
    let fake_org_id = format!("456-ORG");
    let fake_proj_id = format!("789-proj");
    //TODO: Stub for rest call
    //TODO: Test for OK results
    //TODO: Test for Error 
}