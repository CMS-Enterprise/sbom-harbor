use std::{collections::HashMap, fs, fmt::format};

use async_trait::async_trait;
use platform::hyper::{ContentType, Error as HyperError, get, post};
use serde_json::Value;
use crate::{clients::{OrgProjects, SnykProviderError}, config::get_snyk_access_token};

use super::{SnykData, Org};

#[cfg(test)]
use mockall::{automock, mock, predicate::*};

//URL used for getting org and project details
const API_V1_URL: &'static str = "https://snyk.io/api/v1";
//URL used for getting SBOMS
const API_V3_URL: &'static str = "https://api.snyk.io/rest";
const API_V3_VERSION: &'static str = "2023-02-15~beta";
//Format of SBOMs returned from Snyk
const SBOM_FORMAT: &'static str = "cyclonedx%2Bjson";

//TODO: Finish unit tests
///Snyk API trait implementation
pub struct SnykRestClientImpl{}

/// SnykAPI trait that contains all methods related to retrieving SBOM data from snyk
#[cfg_attr(test, automock)]
#[async_trait]
pub trait SnykRestClient {
    /// Retrieves a list of all Orgs from Snyk and adds them into a new SnykData object
    async fn get_orgs(&self, snyk_token: String) -> Result<Vec<Org>, SnykProviderError>;
    /// Returns a ProjectJson object that contains a list of all projects for an org
    async fn get_projects_from_org(&self, snyk_token: String, org_id: String, org_name: String) -> Result<Option<OrgProjects>, SnykProviderError>;
    /// Returns a list of ProjectJson objects that each contain a list of projects for their specific org
    async fn get_projects_from_org_list(&self, snyk_token: String, orgs: Vec<Org>) -> (Vec<OrgProjects>, Vec<SnykProviderError>);
    /// Attempts to get an SBOM from the SNYK Api from a org and project ID
    async fn get_sbom_from_snyk(&self, snyk_token: String, org_id: String, proj_id: String) -> Result<Option<HashMap<String, Value>>, SnykProviderError>;
}

#[async_trait]
impl SnykRestClient for SnykRestClientImpl {

    // Returns a list of Orgs within CMS Snyk
    async fn get_orgs(&self, snyk_token: String) -> Result<Vec<Org>, SnykProviderError>{
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

    // Returns an OrgProjects object that contains an Org and its associated list of Projects
    async fn get_projects_from_org(&self, snyk_token: String, org_id: String, org_name: String) -> Result<Option<OrgProjects>, SnykProviderError> {

        let url = format!("{}/org/{}/projects", API_V1_URL, org_id);
        let display_msg = format!("----------------------\nretrieving project list \nOrg: {}, \nOrg_ID: {}, \nURL: {}", org_name, org_id, url);

        println!("{}", display_msg);
        let response: Result<Option<OrgProjects>, HyperError> = post(
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

    // Returns a tuple that contains a list of OrgProject objects and a list of any errors encountered
    async fn get_projects_from_org_list(&self, snyk_token: String, orgs: Vec<Org>) -> (Vec<OrgProjects>, Vec<SnykProviderError>) {

        let mut all_projects: Vec<OrgProjects> = Vec::new();
        let mut errors: Vec<SnykProviderError> = Vec::new();
        for org in orgs.iter() {
            let org_id = org.id.clone().unwrap_or_else(|| "Missing".to_string());
            let org_name = org.name.clone().unwrap_or_else(|| "Missing".to_string());
            let results = self.get_projects_from_org(snyk_token.clone(), org_id.clone(), org_name.clone()).await;
            
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


    // Returns an sbom from Snyk based 
    async fn get_sbom_from_snyk(&self, snyk_token: String, org_id: String, proj_id: String) -> Result<Option<HashMap<String, Value>>, SnykProviderError> {
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


}

#[tokio::test]
pub async fn test_get_orgs() {
    let snyk_api = SnykRestClientImpl{};
    let snyk_token = get_snyk_access_token().await;

    let orgs = snyk_api.get_orgs(snyk_token).await;

    // Validate we have a successfull response
    assert!(orgs.is_ok(), "Response should be OK");

    // Validate the result has something in it
    assert!(!orgs.unwrap().is_empty(), "The resulting Vec<Org> should not be empty");
}

#[tokio::test]
async fn test_get_projects_from_org() {
    let snyk_api = SnykRestClientImpl{};
    let org_id = format!("f288c129-6c28-4b65-aec2-bd753e095b13");
    let org_name= format!("Test Organization");
    let snyk_token = get_snyk_access_token().await;
    let good_request = snyk_api.get_projects_from_org(snyk_token.clone(), org_id.clone(), org_name.clone()).await;
    println!("{:#?}", good_request);
    // Validate we have a successfull response
    assert!(good_request.is_ok(), "Response should be OK");

    // Validate Option is not empty
    assert!(good_request.as_ref().unwrap().is_some(), "Option in response should be SOME");

    let org_projects = good_request.unwrap().clone().unwrap();

    // Validate OrgProjects.projects field is not empty
    assert!(!org_projects.projects.is_empty(), "The org should have a list of projects");

    // Validate OrgProjects.org.id field
    assert!(org_projects.org.id.is_some(), "Org ID should not be missing");
    assert!(org_projects.org.id.unwrap() == org_id, "ID Should match what we used in the request");

    // Validate OrgProjects.org.name field
    assert!(org_projects.org.name.is_some(), "Org Name should not be missing");
    assert!(org_projects.org.name.unwrap() == org_name, "Name Should match what we used in the request");

    let bad_request = snyk_api.get_projects_from_org(snyk_token, format!("123-abc"), org_name.clone()).await;
    assert!(bad_request.is_err(), "We made a bad request and this should produce an error");

   

}

#[tokio::test]
async fn test_get_projects_from_org_list() {
    let snyk_api = SnykRestClientImpl{};
    let org_list: Vec<Org> = vec![
        Org{
            id: Some(format!("230a9cf3-8880-43bb-a8ff-c69f372d4fc3")), 
            name: Some(format!("PMPP [PMPP]"))
        },
        Org{
            id: Some(format!("3957864e-d4a0-4290-bc83-750a2ea495e8")), 
            name: Some(format!("MACFIN [MACFIN]"))
        },
        Org{
            id: Some(format!("a1738e18-1b90-4ee9-b3dc-10a3b7c51953")), 
            name: Some(format!("HIOS [HIOS]"))
        }
    ];
    let snyk_token = get_snyk_access_token().await;
    let get_proj_results = snyk_api.get_projects_from_org_list(snyk_token, org_list).await;
    println!("{:#?}", get_proj_results);
    // Validate we have a successfull response
    assert!(!get_proj_results.0.is_empty(), "The first part of the tuple should have data in it");
    assert!(get_proj_results.1.is_empty(), "The second part of the tuple should be empty (no errors found)");
}

#[tokio::test]
async fn test_get_sbom_from_snyk() {
    let snyk_api = SnykRestClientImpl{};
    let snyk_token = get_snyk_access_token().await;
    let result = snyk_api.get_sbom_from_snyk(snyk_token.clone(), format!("a1738e18-1b90-4ee9-b3dc-10a3b7c51953"), format!("b85a413d-b972-4271-9a78-87625c108c59")).await;

    // Validate we have a successfull response
    assert!(result.is_ok(), "Response should be OK");

    // Validate result is Some
    assert!(result.as_ref().unwrap().is_some(), "Option in response should be SOME");

    let sbom = result.unwrap().clone().unwrap();
    // Validate hashmap has something in it
    assert!(!sbom.is_empty(), "We should have a hashmap with data in it");
}