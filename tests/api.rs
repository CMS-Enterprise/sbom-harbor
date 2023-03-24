use std::collections::HashMap;
use std::string::ToString;
use aquia::dynamo::Context as DynamoContext;
use aquia::lambdahttp::{Context, DynamoRestService};
use async_std;

mod common;

use aws_lambda_events::query_map::QueryMap;
use lambda_http::Request;
use lambda_http::ext::RequestExt;
use lambda_http::http::Method;
use serde::Serialize;
use harbor::{api, entities};
use harbor::api::{TeamService, TeamContext};

const TEST_TEAM_ID: &str = "11111111-1111-1111-1111-111111111111";

struct MockFactory {}
impl MockFactory {
    fn team_id_path_params() -> HashMap<String, String> {
        HashMap::from([("teamId".to_string(), TEST_TEAM_ID.to_string())])
    }

    fn with_children_qs_params() -> HashMap<String, String> {
        HashMap::from([("children".to_string(), "True".to_string())])
    }

    fn request<D>(method: Method,
        payload: Option<D>,
        path_params: HashMap<String, String>,
        qs_params: HashMap<String, String>) -> Result<Request, String>
        where D: Serialize {

        let mut method = method;
        let mut body = "".to_string();
        if payload.is_some() {
            let payload = payload.unwrap();
            let json = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
            body = json;
        }

        let mut request = Request::new(aws_lambda_events::encodings::Body::from(body))
        .with_path_parameters(QueryMap::from(path_params))
        .with_query_string_parameters(qs_params);

        *request.method_mut() = method;

        Ok(request)
    }

    fn get<D>(path_params: HashMap<String, String>, qs_params: HashMap<String, String>) -> Result<Request, String>
        where D: Serialize {
        Self::request::<D>(Method::GET, None, path_params, qs_params)
    }

    fn put<D>(payload: D, path_params: HashMap<String, String>, qs_params: HashMap<String, String>) -> Result<Request, String>
        where D: Serialize {
        Self::request::<D>(Method::PUT, Some(payload), path_params, qs_params)
    }

    fn post<D>(payload: D, path_params: HashMap<String, String>, qs_params: HashMap<String, String>) -> Result<Request, String>
        where D: Serialize {
        Self::request::<D>(Method::POST, Some(payload), path_params, qs_params)
    }

    fn delete<D>(path_params: HashMap<String, String>, qs_params: HashMap<String, String>) -> Result<Request, String>
        where D: Serialize {
        Self::request::<D>(Method::DELETE, None, path_params, qs_params)
    }

    fn get_team_request() -> Result<Request, String> {
        Self::get::<api::Team>(Self::team_id_path_params(), Self::with_children_qs_params())
    }

    fn get_teams_request() -> Result<Request, String> {
        Self::get::<api::Team>(HashMap::default(), HashMap::default())
    }

    fn delete_team_request() -> Result<Request, String> {
        Self::delete::<api::Team>(Self::team_id_path_params(), Self::with_children_qs_params())
    }
}

#[test]
fn can_create_mock_request() -> Result<(), String> {
    let request = MockFactory::get_team_request()?;

    let payload = request.payload::<api::Team>().map_err(|e| e.to_string())?;
    let path_params = request.path_parameters();
    let teamId = path_params.first("teamId").unwrap();
    let qs_params = request.query_string_parameters();

    assert_eq!(request.method(), Method::GET);
    assert!(payload.is_none());
    assert_eq!(teamId, "test");
    assert!(qs_params.is_empty());

    Ok(())
}

#[test]
fn can_create_team_context() -> Result<(), String> {
    // GET Team
    let request = MockFactory::get_team_request()?;
    let ctx = TeamContext::from_request(&request).map_err(|e| e.to_string())?;

    assert_eq!(ctx.id, TEST_TEAM_ID);

    // DELETE TEAM
    let request = MockFactory::delete_team_request()?;
    let ctx = TeamContext::from_request(&request).map_err(|e| e.to_string())?;

    assert_eq!(ctx.id, TEST_TEAM_ID);

    // GET TEAMS
    let request = MockFactory::get_teams_request()?;
    let ctx = TeamContext::from_request(&request).map_err(|e| e.to_string())?;

    let query = DynamoContext::as_dynamo_entity(&ctx);
    assert!(!query.is_err());

    Ok(())
}

#[async_std::test]
async fn can_find_from_team_context() -> Result<(), String> {
    // GET Team
    let request = MockFactory::get_team_request()?;
    let ctx = TeamContext::from_request(&request).map_err(|e| e.to_string())?;

    assert_eq!(ctx.id, TEST_TEAM_ID);

    let team = TeamService::get(&ctx).await.map_err(|e| e.to_string())?;

    assert!(!team.is_none());

    let team = team.unwrap();
    assert!(!team.id.is_empty());

    Ok(())
}
