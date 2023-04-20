use std::{env, time::Instant, collections::HashMap};
use async_trait::async_trait;
use clients::{OrgProjects};
use harbor_client::client::simple_upload_sbom;
use platform::auth::get_secret;
use serde_json::Value;

use crate::{clients::{self, SnykRestClientImpl, SnykRestClient, Org, ProjectDetails}, config::{get_cf_domain, get_cms_team_token, get_cms_team_id, get_snyk_access_token}};

use super::SbomProvider;

#[cfg(test)]
use mockall::{automock, mock, predicate::*};
#[cfg(test)]
use crate::clients::MockSnykRestClient;

//DONE: Add method call
//DONE: See if it runs
//DONE: Pull in HTTP library
//DONE: Get org count from snyk repo
//DONE: Get projects for each org
//DONE: Get SBOMS for each project
//DONE: Save SBOMS to temp location
//DONE: Restructure and cleanup pass 1
//DONE: Merge Quinns code
//DONE: Restructure and cleanup pass 2
//DONE: Rebase main
//DONE: Event logging
//DONE: Documentation
//TODO: Rename retrieve_and_upload_valid_sboms to: Retrieve SBOMS
//TODO: Modify method to return a list of SBOMS
//TODO: New method to upload sboms 
//TODO: Rework Unit tests
//TODO: Send SBOM somewhere and review solution works...
//TODO: Final Restructure and cleanup pass


impl SnykProvider {

    pub fn new() -> SnykProvider {
        SnykProvider { snyk_api: Box::new(SnykRestClientImpl{}) }
    }

    //Origins that can have associated SBOM data in Snyk
    const VALID_ORIGINS: &'static [&'static str] = &["cli", "github", "github-enterprise", "gitlab"];
    //Types that can have associated SBOM data in Snyk
    const VALID_TYPES: &'static [&'static str] = &["npm", "nuget", "gradle", "hex", "pip", "poetry", "rubygems",
    "maven", "yarn", "yarn-workspace", "composer", "gomodules", "govendor", "golang", "golangdep", "gradle", "paket", "cocoapods", "cpp", "sbt"];
    const AWS_SECRET_NAME: &'static str = "dev-sbom-harbor-snyk-token-use1";

    // Collects and returns a tuple for each Org that contains a validated SBOM and associated project details 
    async fn get_all_snyk_sboms(&self, snyk_token: String, org_projects: OrgProjects, valid_sboms: &mut Vec<(HashMap<String, Value>, ProjectDetails)>) {
        
        //let mut valid_sboms:  Vec<(HashMap<String, Value>, ProjectDetails)> = Vec::new();
        for project in org_projects.projects.iter() {
            match project {
                project_detail if project_detail.id.is_empty() => println!("Missing project Id for: {}", project_detail.name), 
                project_detail if (Self::VALID_ORIGINS.contains(&project_detail.origin.as_str()) &&  Self::VALID_TYPES.contains(&project_detail.r#type.as_str()))=> {
                    let result = self.snyk_api.get_sbom_from_snyk(snyk_token.clone(), org_projects.org.id.clone().unwrap(), project_detail.id.clone()).await;
                    match result {
                        Ok(option_sbom) => {
                            match option_sbom {
                                #[allow(unused_variables)]
                                Some(sbom) =>{
                                    valid_sboms.push((sbom, project.clone()));
                                    println!("Adding valid SBOM to list for project: {}, from org: {}", project.id, org_projects.org.id.clone().unwrap());
                                    
                                },
                                None => println!("No SBOM found for project: {}, from org: {}", project.id, org_projects.org.id.clone().unwrap()),
                            }
                        },
                        Err(err) => println!("{}", err),

                    }
                },
                _ => continue 
            }
        }
       // return valid_sboms;
    }
    
    async fn get_valid_sboms(&self) -> Vec<(HashMap<String, Value>, ProjectDetails)>{

        let start = Instant::now();
        let mut valid_sboms:  Vec<(HashMap<String, Value>, ProjectDetails)> = Vec::new();

        println!("Starting SnykProvider scan...");
        
        let snyk_token = get_snyk_access_token().await;
        println!("Scanning Snyk for SBOMS...");

        let snyk_data = self.snyk_api.get_orgs(snyk_token.clone()).await;

        match snyk_data {
            Ok(data) => {
                let get_projects_tuple_result = self.snyk_api.get_projects_from_org_list(snyk_token.clone(), data).await;

                for errors in get_projects_tuple_result.1 {
                    println!("{}", errors)
                }

                for orgs in get_projects_tuple_result.0.iter() {
                    self.get_all_snyk_sboms(snyk_token.clone(), orgs.clone(), &mut valid_sboms).await;
                }
            },
            Err(err) => panic!("{}", err), // If no orgs are found something went seriously wrong, no reason to go any further
        }

        println!("Finished SnykProvider scan, elapsed time in milis: ({:?})", start.elapsed().as_millis());

        return valid_sboms;
    }

}

pub struct SnykProvider{
    snyk_api: Box<dyn Send + Sync + SnykRestClient>,
}

#[async_trait]
impl SbomProvider for SnykProvider {

    async fn provide_sboms(&self) {
        println!("Providing sboms");
        let valid_sboms = self.get_valid_sboms().await;

        for sbom_results in valid_sboms.iter() {
            //println!("Uploading SBOM to harbor for project: {}, from org: {}", sbom_results.1.id, orgs.org.id.clone().unwrap());
            println!("Uploading SBOM to harbor for project: {}", sbom_results.1.id);
            //TODO: Re-enable the next line
            //simple_upload_sbom(cloud_front_domain.clone(), sbom_token.clone(), team_id.clone(), sbom_results.1.browse_url.clone(), sbom_results.1.r#type.clone(), sbom_results.0).await;
        }
    }
}

#[tokio::test]
async fn booger() {
   let provider = SnykProvider::new(); 
   provider.provide_sboms().await;
}

#[tokio::test]
async fn test_get_snyk_data() {
    let fake_token = format!("123-abc");
    let mut mock_snyk_client = MockSnykRestClient::new();
    let orgs = Org{id: Some(format!("123abc")), name: Some(format!("test"))};
    let orgs_vec = vec![orgs];

    // Mock for get_orgs in snyk_client
    mock_snyk_client.expect_get_orgs().returning(move |_| Ok(orgs_vec.clone()));

    // Mock for get_projects_from_org in snyk_client
    // Mock for get_projects_from_org_list in snyk_client
    // Mock for get_sbom_from_snyk in snyk_client

    let provider = SnykProvider{snyk_api: Box::new(mock_snyk_client)};
    
    let sboms = provider.get_valid_sboms().await;
    //let snyk_data = provider.snyk_api.get_orgs(fake_token.clone()).await;
    println!("{:#?}", sboms);
}