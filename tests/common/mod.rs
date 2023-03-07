use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Once;

use anyhow::Result;
use aws_lambda_events::apigw::{
    ApiGatewayV2httpRequest, ApiGatewayV2httpRequestContext,
    ApiGatewayV2httpRequestContextHttpDescription,
};
use aws_lambda_events::http::HeaderMap;
use dotenv;
use harbor::lib::Client;
use harbor::entities::Team;
use hyper::http;
use uuid::Uuid;

use harbor::handler::PilotRequest;

mod api;

pub use api::{MockFactory as ApiMockFactory, TEST_TEAM_ID};

static INIT: Once = Once::new();

#[ctor::ctor]
fn init() {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
    });
}

#[allow(dead_code)]
pub fn log(entry: &str) -> std::io::Result<()> {
    let mut log_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    log_file.push("tests/fixtures/debug.log");

    let mut file_ref = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_file)?;

    file_ref.write_all(entry.as_bytes())
}

#[allow(dead_code)]
pub async fn get_client(cloud_front_domain: String) -> Result<Client> {
    let username = env::var("ADMIN_USERNAME").unwrap_or(String::from(""));
    let password = env::var("ADMIN_PASSWORD").unwrap_or(String::from(""));
    Client::new(cloud_front_domain, username, password).await
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TestContext {
    pub request: PilotRequest,
    pub client: Client,
    pub team: Team,
}

#[allow(dead_code)]
pub async fn get_test_context() -> Result<TestContext> {
    let mut request = get_pilot_request()?;

    let client = get_client(request.cloud_front_domain.to_string()).await?;

    let test_org = format!("test-{}", Uuid::new_v4());

    let mut team = client
        .get_or_create_team(String::from("7a33b3df-e5c8-4e81-a284-ed33abb53a68"), test_org)
        .await?;

    let project = client
        .create_project_with_codebase(team.id.clone(), &team.name.clone(), &team.name.clone())
        .await?;

    team.projects.push(project);

    request.team_id = team.id.clone();
    request.project_id = team.projects[0].id.clone();
    request.codebase_id = team.projects[0].codebases[0].id.clone();
    request.token = team.tokens[0].token.clone();

    Ok(TestContext {
        request,
        client,
        team,
    })
}

#[allow(dead_code)]
pub async fn teardown(ctx: TestContext) -> std::io::Result<()> {
    // Delete the team
    let resp = ctx.client.delete_team(ctx.team.id.clone()).await;

    assert!(!resp.is_err(), "{:?}", resp);

    Ok(())
}

// This function requires you to generate a pilot.json file with valid
// values from the target test environment
#[allow(dead_code)]
pub async fn generate_pilot_apigw_request() -> Result<(TestContext, ApiGatewayV2httpRequest)> {
    let ctx = get_test_context().await?;

    let body = serde_json::to_string(&ctx.request)?;

    let mut headers = HeaderMap::new();
    headers.append("content-type", "application/json".parse().unwrap());

    let http = ApiGatewayV2httpRequestContextHttpDescription {
        method: http::Method::POST,
        path: None,
        protocol: None,
        source_ip: None,
        user_agent: None,
    };

    let request_context = ApiGatewayV2httpRequestContext {
        route_key: None,
        account_id: None,
        stage: None,
        request_id: None,
        authorizer: None,
        apiid: None,
        domain_name: None,
        domain_prefix: None,
        time: None,
        time_epoch: 0,
        http,
        authentication: None,
    };

    Ok((
        ctx,
        ApiGatewayV2httpRequest {
            version: None,
            route_key: None,
            raw_path: None,
            raw_query_string: None,
            cookies: None,
            headers,
            query_string_parameters: Default::default(),
            path_parameters: Default::default(),
            request_context,
            stage_variables: Default::default(),
            body: Some(body),
            is_base64_encoded: false,
        },
    ))
}

#[allow(dead_code)]
pub fn get_pilot_request() -> std::io::Result<PilotRequest> {
    let api_gateway_url = env::var("API_GW_URL");

    assert!(!api_gateway_url.is_err());

    let api_gateway_url = Some(api_gateway_url.unwrap());

    let req = PilotRequest {
        team_id: "".to_string(),
        project_id: "".to_string(),
        codebase_id: "".to_string(),
        cloud_front_domain: env::var("CF_DOMAIN").unwrap_or(String::from("")),
        api_gateway_url,
        token: env::var("HARBOR_TOKEN").unwrap_or(String::from("")),
        github_url: env::var("HARBOR_TEST_GH_URL").unwrap_or(String::from("")),
    };

    Ok(req)
}

// This function ensures you have a valid pilot APIGW request in
// the tests/fixtures directory so that you can more easily test
// the lambda locally, or remotely with curl. See the README for usage.
#[async_std::test]
#[ignore = "manual run only"]
pub async fn save_test_fixtures() -> std::io::Result<()> {

    use base64::{Engine as _, engine::{general_purpose}};

    let tuple = generate_pilot_apigw_request().await;

    assert!(!tuple.is_err(), "{:?}", tuple);

    let tuple = tuple.unwrap();
    let pilot_req = tuple.1.clone().body.unwrap();
    let req = serde_json::to_string(&tuple.1).unwrap();

    let mut target_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    target_file.push("tests/fixtures/pilot-request.json");

    std::fs::write(target_file.as_path(), req)?;

    target_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    target_file.push("tests/fixtures/HARBOR_PILOT_REQUEST");

    let secret = general_purpose::STANDARD.encode(pilot_req);
    std::fs::write(target_file.as_path(), secret)
}

// Utility method that can be manually invoked to delete the team in the GW request and purge test files.
#[async_std::test]
#[ignore = "manual run only"]
pub async fn delete_test_fixtures() -> std::io::Result<()> {
    let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("tests/fixtures/pilot-request.json");

    let raw_req = std::fs::read_to_string(test_file.as_path())?;
    let apigw_req: ApiGatewayV2httpRequest = serde_json::from_str(&raw_req)?;

    let body = apigw_req.body.unwrap();

    let pilot_req: PilotRequest = serde_json::from_str(&body)?;

    let client = get_client(env::var("CF_DOMAIN").unwrap_or(String::from(""))).await;

    assert!(!client.is_err());

    let client = client.unwrap();

    let result = client.delete_team(pilot_req.team_id).await;

    assert!(!result.is_err());

    std::fs::remove_file(test_file)?;

    let mut secret = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    secret.push("tests/fixtures/HARBOR_PILOT_REQUEST");

    std::fs::remove_file(secret)?;

    Ok(())
}
