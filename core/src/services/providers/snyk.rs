use std::{env, time::Instant};
use async_trait::async_trait;
use clients::{ProjectJson};
use platform::auth::get_secret;

use crate::{clients::{self, SnykApiImpl, SnykAPI, Org}, config::{get_cf_domain, get_cms_team_token, get_cms_team_id, get_snyk_access_token}};

use super::SbomProvider;

#[cfg(test)]
use mockall::{automock, mock, predicate::*};
#[cfg(test)]
use crate::clients::MockSnykAPI;

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
//TODO: Better unit tests
//TODO: Send SBOM somewhere and review solution works...
//TODO: Final Restructure and cleanup pass

impl SnykProvider {

    //Origins that can have associated SBOM data in Snyk
    const VALID_ORIGINS: &'static [&'static str] = &["cli", "github", "github-enterprise", "gitlab"];
    //Types that can have associated SBOM data in Snyk
    const VALID_TYPES: &'static [&'static str] = &["npm", "nuget", "gradle", "hex", "pip", "poetry", "rubygems", 
    "maven", "yarn", "yarn-workspace", "composer", "gomodules", "govendor", "golang", "golangdep", "gradle", "paket", "cocoapods", "cpp", "sbt"];

    // Iterates over list of projects and uploads any valid sboms to harbor
    async fn retrieve_and_upload_valid_sboms(snyk_token: String, project_json: ProjectJson) {
        let cloud_front_domain = get_cf_domain().unwrap();
        let sbom_token = get_cms_team_token().unwrap();
        let team_id = get_cms_team_id().unwrap();

        for project in project_json.projects.iter() {
            match project {
                project_detail if project_detail.id.is_empty() => println!("Missing project Id for: {}", project_detail.name), 
                project_detail if (Self::VALID_ORIGINS.contains(&project_detail.origin.as_str()) &&  Self::VALID_TYPES.contains(&project_detail.r#type.as_str()))=> {
                    let result = SnykApiImpl::get_sbom_from_snyk(snyk_token.clone(), project_json.org.id.clone().unwrap(), project_detail.id.clone()).await;
                    match result {
                        Ok(option_sbom) => {
                            match option_sbom {
                                Some(sbom) =>{
                                    println!("Uploading SBOM to harbor for project: {}, from org: {}", project.id, project_json.org.id.clone().unwrap());
                                    //TODO: Re-enable the next line
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
        
        let snyk_token = get_snyk_access_token().await;
        println!("Scanning Snyk for SBOMS...");
        let snyk_data = SnykApiImpl::get_orgs(snyk_token.clone()).await;

        match snyk_data {
            Ok(data) => {
                let project_list = SnykApiImpl::get_projects_from_org_list(snyk_token.clone(), data).await;

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
    let fake_token = format!("123-abc");
    let ctx = MockSnykAPI::get_orgs_context();
    ctx.expect().returning(|_| {
        let fake_org_1 = Org{id: Some(format!("Org_1_id")), name: Some(format!("Org_1_name"))};
        let fake_orgs_list: Vec<Org> = vec![fake_org_1];
        Ok(fake_orgs_list)
    });
    let res = MockSnykAPI::get_orgs(fake_token).await;
   // let mo = MockSnykApiImpl
    let provider = SnykProvider{};
    provider.provide_sboms().await;
}