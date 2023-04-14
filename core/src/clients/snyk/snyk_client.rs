use std::collections::HashMap;

use async_trait::async_trait;
use platform::hyper::{ContentType, Error as HyperError, get, post};
use serde_json::Value;
use crate::{clients::{ProjectJson, SnykProviderError}, config::get_snyk_access_token};

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


///Snyk API trait implementation
pub struct SnykApiImpl{}

/// SnykAPI trait that contains all methods related to retrieving SBOM data from snyk
#[cfg_attr(test, automock)]
#[async_trait]
pub trait SnykAPI {
    /// Retrieves a list of all Orgs from Snyk and adds them into a new SnykData object
    async fn get_orgs(snyk_token: String) -> Result<Vec<Org>, SnykProviderError>;
    /// Returns a ProjectJson object that contains a list of all projects for an org
    async fn get_projects_from_org(snyk_token: String, org_id: String, org_name: String) -> Result<Option<ProjectJson>, SnykProviderError>;
    /// Returns a list of ProjectJson objects that each contain a list of projects for their specific org
    async fn get_projects_from_org_list(snyk_token: String, orgs: Vec<Org>) -> (Vec<ProjectJson>, Vec<SnykProviderError>);
    /// Attempts to get an SBOM from the SNYK Api from a org and project ID
    async fn get_sbom_from_snyk(snyk_token: String, org_id: String, proj_id: String) -> Result<Option<HashMap<String, Value>>, SnykProviderError>;
}

#[async_trait]
impl SnykAPI for SnykApiImpl {
    async fn get_orgs(snyk_token: String) -> Result<Vec<Org>, SnykProviderError>{
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

    async fn get_projects_from_org(snyk_token: String, org_id: String, org_name: String) -> Result<Option<ProjectJson>, SnykProviderError> {

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

    async fn get_projects_from_org_list(snyk_token: String, orgs: Vec<Org>) -> (Vec<ProjectJson>, Vec<SnykProviderError>) {

        let mut all_projects: Vec<ProjectJson> = Vec::new();
        let mut errors: Vec<SnykProviderError> = Vec::new();
        for org in orgs.iter() {
            let org_id = org.id.clone().unwrap_or_else(|| "Missing".to_string());
            let org_name = org.name.clone().unwrap_or_else(|| "Missing".to_string());
            let results = Self::get_projects_from_org(snyk_token.clone(), org_id.clone(), org_name.clone()).await;
            
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

    async fn get_sbom_from_snyk(snyk_token: String, org_id: String, proj_id: String) -> Result<Option<HashMap<String, Value>>, SnykProviderError> {
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
    //TODO: Validate results
    //TODO: Configure mock rest call
    let snyk_token = get_snyk_access_token().await;

    let orgs = SnykApiImpl::get_orgs(snyk_token).await;
    println!("{:#?}", orgs);
}

#[tokio::test]
async fn test_get_projects_from_org() {
    //TODO: Stub for rest call
    //TODO: Test for OK results
    //TODO: Test for Error 
    let org_id = format!("f288c129-6c28-4b65-aec2-bd753e095b13");
    let org_name= format!("Test Organization");
    // let org_id = format!("555797d4-7d2b-4588-ba4a-84776b2f9ee8");
    // let org_name= format!("BatCAVE (ISPG) [AWS]");
    let snyk_token = get_snyk_access_token().await;
    let projects = SnykApiImpl::get_projects_from_org(snyk_token, org_id, org_name).await;
    println!("{:#?}", projects);

    // Org {
    //     id: Some(
    //         "555797d4-7d2b-4588-ba4a-84776b2f9ee8",
    //     ),
    //     name: Some(
    //         "BatCAVE (ISPG) [AWS]",
    //     ),
    // },
    // Org {
    //     id: Some(
    //         "71e3fb2e-696c-426f-9774-5e62fa2ac44a",
    //     ),
    //     name: Some(
    //         "IDDOC|4 Innovation [ACO-OS]",
    //     ),
    // },
    // Org {
    //     id: Some(
    //         "00d7ace1-af35-4535-a1bb-c468d22336d0",
    //     ),
    //     name: Some(
    //         "ZONE [ZONE]",
    //     ),
    // },
    // Org {
    //     id: Some(
    //         "61163aee-82c0-4183-88a2-7df9470c154a",
    //     ),
    //     name: Some(
    //         "RASS (OIT) [RAS-RAPS]",
    //     ),
    // },
    // Org {
    //     id: Some(
    //         "b745377d-e6ac-465f-bc66-b642491805ad",
    //     ),
    //     name: Some(
    //         "CDRAP - FAS [CDRAP]",
    //     ),
    // },
    // Org {
    //     id: Some(
    //         "ad46f70d-5139-47cf-b4a1-bfbb4cf42ae4",
    //     ),
    //     name: Some(
    //         "CDRAP - QDAS [CDRAP]",
    //     ),
    // },
    // Org {
    //     id: Some(
    //         "0aacbfcb-6a16-4307-b38f-e1ef0cd017ca",
    //     ),
    //     name: Some(
    //         "EQRS - S&F [EQRS]",
    //     ),
    // },
    // Org {
    //     id: Some(
    //         "ea7509ec-8d27-45fe-a878-d0c1d4be1751",
    //     ),
    //     name: Some(
    //         "SC QCOR [SC QCOR]",
    //     ),
    // },
    // Org {
    //     id: Some(
    //         "7ca8a69a-014b-45b3-8c2a-45f28acfdbb1",
    //     ),
    //     name: Some(
    //         "AMS [AMS]",
    //     ),
    // },
    // Org {
    //     id: Some(
    //         "f288c129-6c28-4b65-aec2-bd753e095b13",
    //     ),
    //     name: Some(
    //         "Test Organization",
    //     ),
    // },
    // Org {
    //     id: Some(
    //         "a1738e18-1b90-4ee9-b3dc-10a3b7c51953",
    //     ),
    //     name: Some(
    //         "HIOS [HIOS]",
    //     ),
    // },
    // Org {
    //     id: Some(
    //         "3957864e-d4a0-4290-bc83-750a2ea495e8",
    //     ),
    //     name: Some(
    //         "MACFIN [MACFIN]",
    //     ),
    // },
    // Org {
    //     id: Some(
    //         "230a9cf3-8880-43bb-a8ff-c69f372d4fc3",
    //     ),
    //     name: Some(
    //         "PMPP [PMPP]",
    //     ),
    // },
}

#[tokio::test]
async fn test_get_projects_from_org_list() {
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
    let projects = SnykApiImpl::get_projects_from_org_list(snyk_token, org_list).await;

    for project_json in projects.0 {
        if project_json.org.id.is_some() {
            println!("{:#?}", project_json);
        }
    }

    for errors in projects.1 {
        println!("{}", errors)
    }
}

#[tokio::test]
async fn test_get_sbom_from_snyk() {
    // let fake_token = format!("123-abc");
    // let fake_org_id = format!("456-ORG");
    // let fake_proj_id = format!("789-proj");
    //TODO: Stub for rest call
    //TODO: Test for OK results
    //TODO: Test for Error 
}