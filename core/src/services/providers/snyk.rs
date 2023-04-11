use std::{env, time::Instant};
use async_trait::async_trait;
use clients::{ProjectJson, get_orgs, get_projects_from_org_list, get_sbom_from_snyk};
use platform::auth::get_secret;

use crate::clients::{self, SnykAPI_Impl, SnykAPI};

use super::SbomProvider;

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
//TODO: Event logging
//TODO: Send SBOM somewhere and review solution works...
//TODO: Documentation
//TODO: Final Restructure and cleanup pass

impl SnykProvider {

    //Origins that can have associated SBOM data in Snyk
    const VALID_ORIGINS: &'static [&'static str] = &["cli", "github", "github-enterprise", "gitlab"];
    //Types that can have associated SBOM data in Snyk
    const VALID_TYPES: &'static [&'static str] = &["npm", "nuget", "gradle", "hex", "pip", "poetry", "rubygems", 
    "maven", "yarn", "yarn-workspace", "composer", "gomodules", "govendor", "golang", "golangdep", "gradle", "paket", "cocoapods", "cpp", "sbt"];
    const AWS_SECRET_NAME: &'static str = "dev-sbom-harbor-snyk-token-use1";

    // Iterates over list of projects and uploads any valid sboms to harbor
    async fn retrieve_and_upload_valid_sboms(snyk_token: String, project_json: ProjectJson) {
        let cloud_front_domain = env::var("CF_DOMAIN").unwrap_or(String::from("")); //TODO: get this only once
        let sbom_token = env::var("V1_CMS_TEAM_TOKEN").unwrap_or(String::from("")); //TODO: get this only once
        let team_id = env::var("V1_CMS_TEAM_ID").unwrap_or(String::from("")); //TODO: get this only once

        for project in project_json.projects.iter() {
            match project {
                project_detail if project_detail.id.is_empty() => println!("Missing project Id for: {}", project_detail.name), 
                project_detail if (Self::VALID_ORIGINS.contains(&project_detail.origin.as_str()) &&  Self::VALID_TYPES.contains(&project_detail.r#type.as_str()))=> {
                    let result = get_sbom_from_snyk(snyk_token.clone(), project_json.org.id.clone().unwrap(), project_detail.id.clone()).await;
                    match result {
                        Ok(option_sbom) => {
                            match option_sbom {
                                Some(sbom) =>{
                                    println!("Uploading SBOM to harbor for project: {}, from org: {}", project.id, project_json.org.id.clone().unwrap());
                                    //simple_upload_sbom(cloud_front_domain.clone(), sbom_token.clone(), team_id.clone(), project.browseUrl.clone(), project.r#type.clone(), sbom).await;
                                    },
                                None => println!("No SBOM found for project: {}, from org: {}", project.id, project_json.org.id.clone().unwrap()),
                            }
                        },
                        Err(err) => println!("{}", err),

                    }
                },
                _ => continue // Do nothing with projects that cannot make SBOMS
            }
        }
    }

}

#[async_trait]
impl SbomProvider for SnykProvider {

    async fn provide_sboms(&self) {
        let start = Instant::now();

        println!("Starting SnykProvider scan...");
        

        println!("Obtaining SNYK access key...");
        let response = get_secret(Self::AWS_SECRET_NAME).await;
        let snyk_token = match response {
            Ok(secret) => {
                match secret {
                    Some(s) => s,
                    None => panic!("No AWS token retrieved for secret: {}", Self::AWS_SECRET_NAME), //Stop everything if we dont get an access key
                }
            },
            Err(err) => panic!("Failed to retrieve token for secret: {}, with error: {}", Self::AWS_SECRET_NAME, err), //Stop everything if we dont get an access key
        };

        println!("Scanning Snyk for SBOMS...");
        SnykAPI_Impl::get_orgs(snyk_token.clone());
        let snyk_data = get_orgs(snyk_token.clone()).await;

        match snyk_data {
            Ok(data) => {
                let project_list = get_projects_from_org_list(snyk_token.clone(), data).await;

                for project_json in project_list.0 {
                    if project_json.org.id.is_some() {
                        Self::retrieve_and_upload_valid_sboms(snyk_token.clone(), project_json).await;
                    }
                }

                for errors in project_list.1 {
                    println!("{}", errors)
                }

            },
            Err(err) => panic!("{}", err), // If no orgs are found something went seriously wrong, no reason to go any further
        }

        println!("Finished SnykProvider scan, elapsed time in milis: ({:?})", start.elapsed().as_millis());
        //TODO: delete this
        // let TEMP_TEST_DIR: &'static str = "/home/jshattjr";
        // let data = format!("{:#?}", project_list); //TODO: Remove this when done debugging
        // let output_path = format!("{}/snyk_data.json", Self::TEMP_TEST_DIR);
        // fs::write(output_path, data).expect("Unable to write file"); //TODO: Remove this when done debugging

    }
}

#[tokio::test]
async fn test_get_snyk_data() {
    let provider = SnykProvider{};
    provider.provide_sboms().await;
}