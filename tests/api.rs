use std::string::ToString;
use harbor::api;
use harbor::api::TeamContext;
mod common;

use common::{ApiMockFactory as MockFactory, TEST_TEAM_ID};

#[test]
fn can_create_mock_request() -> Result<(), String> {
    let request = MockFactory::get_team_request(Some(TEST_TEAM_ID.to_string()))?;

    let payload = request.payload::<api::Team>()
        .map_err(|e| e.to_string())?;
    let path_params = request.path_parameters();
    let team_id = path_params.first("teamId").unwrap();
    let qs_params = request.query_string_parameters();
    let children = qs_params.first("children").unwrap();

    assert_eq!(request.method(), Method::GET);
    assert!(payload.is_none());
    assert_eq!(team_id, TEST_TEAM_ID);
    assert!(!qs_params.is_empty());
    assert_eq!(children, "true");

    Ok(())
}

#[test]
fn can_create_team_context() -> Result<(), String> {
    // GET Team
    let request = MockFactory::get_team_request(None)?;
    let ctx = TeamContext::from_request(&request).map_err(|e| e.to_string())?;

    assert_eq!(ctx.id.clone().unwrap(), TEST_TEAM_ID);

    // DELETE TEAM
    let request = MockFactory::delete_team_request(Some(ctx.id.clone().unwrap().clone()))?;
    let ctx = TeamContext::from_request(&request).map_err(|e| e.to_string())?;

    assert_eq!(ctx.id.clone().unwrap(), TEST_TEAM_ID);

    // GET TEAMS
    let request = MockFactory::get_teams_request()?;
    let ctx = TeamContext::from_request(&request).map_err(|e| e.to_string())?;
    assert!(ctx.id.clone().is_none());

    let query = DynamoContext::as_dynamo_entity(&ctx);
    assert!(!query.is_err());

    Ok(())
}
