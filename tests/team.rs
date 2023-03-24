use aquia::config::Config;
use aquia::dynamo::Store;
use uuid::Uuid;
use harbor::entities::{Entity, Team};
use harbor::team::get_team::handle_request as handle_get_team;
use harbor::team::get_teams::handle_request as handle_get_teams;
use harbor::team::post_team::handle_request as handle_post_team;
use harbor::team::put_team::handle_request as handle_put_team;
use harbor::team::delete_team::handle_request as handle_delete_team;

mod common;

fn test_team() -> Team {
    let id = Uuid::new_v4();
    let team_name = format!("team-test-{}", id);
    let mut team = Team::new(team_name);
    team.id = id.to_string();
    team
}

#[async_std::test]
async fn can_handle_get_team() -> Result<(), String> {
    let config = Config::new().await.map_err(|e| e.to_string())?;
    let store = Store::new(config);
    let mut team = test_team();

    // Insert the test team.
    store.insert::<Team>(Box::new(team.clone())).await.map_err(|e| e.to_string())?;

    team.load().map_err(|e| format!("{}", e) )?;

    // Validate it was written.
    let query = team.clone();
    let query: Option<Team> = store.find(Box::new(query)).await.map_err(|e| e.to_string())?;
    assert!(query.is_some());

    let query = query.unwrap();
    assert_eq!(query.id, team.id);
    assert_eq!(query.name, team.name);

    // Get Team from handler.
    let resp = handle_get_team(team.id.as_str()).await;
    assert!(!resp.is_err(), "{:?}", resp);

    let resp = resp.unwrap();
    assert!(resp.is_some());

    let resp = resp.unwrap();
    assert_eq!(team.id, resp.id);
    assert_eq!(team.name, resp.name);

    println!("get_team found team {:?}", team.clone());

    store.delete::<Team>(team.key_context()).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[async_std::test]
async fn can_handle_get_teams() -> Result<(), String> {
    let config = Config::new().await.map_err(|e| e.to_string())?;
    let store = Store::new(config);
    let mut team = test_team();

    // Insert the test team.
    store.insert::<Team>(Box::new(team.clone())).await.map_err(|e| e.to_string())?;

    team.load().map_err(|e| format!("{}", e) )?;

    // Validate it was written.
    let query = team.clone();
    let query: Option<Team> = store.find(Box::new(query)).await.map_err(|e| e.to_string())?;
    assert!(query.is_some());

    let query = query.unwrap();
    assert_eq!(query.id, team.id);
    assert_eq!(query.name, team.name);

    // Validate handler can get teams.
    let resp = handle_get_teams().await;
    assert!(!resp.is_err(), "{:?}", resp);

    let resp = resp.unwrap();
    assert!(!resp.is_empty());

    let resp = resp.into_iter()
            .find(|t| t.id == team.id);

    assert!(resp.is_some(), "{:?}", resp);

    let resp = resp.unwrap();
    assert_eq!(resp.id, team.id);

    store.delete::<Team>(team.key_context()).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[async_std::test]
async fn can_handle_post_team() -> Result<(), String> {
    let mut team = test_team();
    team.id = "".to_string();

    let resp = handle_post_team(team.clone()).await.map_err(|e| e.to_string())?;

    assert!(resp.is_some(), "{:?}", resp);
    team = resp.unwrap();

    let config = Config::new().await.map_err(|e| e.to_string())?;
    let store = Store::new(config);

    let query = team.clone();
    let query: Option<Team> = store.find(Box::new(query)).await.map_err(|e| e.to_string())?;

    assert!(query.is_some());

    let mut query = query.unwrap();
    team.load().map_err(|e| format!("{}", e) )?;
    query.load().map_err(|e| format!("{}", e) )?;

    assert_eq!(team.id, query.id);
    assert_eq!(team.partition_key(), query.partition_key());
    assert_eq!(team.sort_key(), query.sort_key());
    assert_eq!(team.name, query.name);
    assert!(query.is_child_of(team.partition_key().unwrap()));

    store.delete::<Team>(team.key_context()).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[async_std::test]
async fn can_handle_put_team() -> Result<(), String> {
    let config = Config::new().await.map_err(|e| e.to_string())?;
    let store = Store::new(config);
    let team = test_team();

    // Insert the test team.
    store.insert::<Team>(Box::new(team.clone())).await.map_err(|e| e.to_string())?;

    // Validate it was written.
    let query = team.clone();
    let query: Option<Team> = store.find(Box::new(query)).await.map_err(|e| e.to_string())?;
    assert!(query.is_some());

    let query = query.unwrap();
    assert_eq!(query.id, team.id);
    assert_eq!(query.name, team.name);

    let mut update= team.clone();
    update.name = "update-name".to_string();

    let resp = handle_put_team(update.clone()).await.map_err(|e| e.to_string())?;

    assert!(resp.is_some(), "{:?}", resp);

    let query: Option<Team> = store.find(Box::new(query)).await.map_err(|e| e.to_string())?;

    assert!(query.is_some());

    let mut query = query.unwrap();
    update.load().map_err(|e| format!("{}", e) )?;
    query.load().map_err(|e| format!("{}", e) )?;

    assert_eq!(update.id, query.id);
    assert_eq!(update.partition_key, query.partition_key);
    assert_eq!(update.sort_key(), query.sort_key());
    assert_eq!(update.name, query.name);
    assert!(query.is_child_of(update.partition_key.clone()));

    store.delete::<Team>(update.key_context()).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[async_std::test]
async fn can_handle_delete_team() -> Result<(), String> {
    let config = Config::new().await.map_err(|e| e.to_string())?;
    let store = Store::new(config);
    let team = test_team();

    // Insert the test team.
    store.insert::<Team>(Box::new(team.clone())).await.map_err(|e| e.to_string())?;

    // Validate it was written.
    let query = team.clone();
    let query: Option<Team> = store.find(Box::new(query)).await.map_err(|e| e.to_string())?;

    assert!(query.is_some());
    let query = query.unwrap();
    assert_eq!(query.id, team.id);
    assert_eq!(query.name, team.name);

    // Delete the team by id.
    handle_delete_team(team.id).await.map_err(|e| e.to_string())?;

    // Validate it is deleted.
    let query: Option<Team> = store.find(Box::new(query)).await.map_err(|e| e.to_string())?;
    assert!(query.is_none());

    Ok(())
}
