use std::sync::Arc;
use harbcore::config::db_connection;
use platform::mongodb::{Context, Service, Store};
use harbcore::models::*;
use harbcore::services::*;
use harbcore::Error;


fn test_context() -> Context {
    Context{
        connection_uri: db_connection().unwrap(),
        db_name: "harbor".to_string(),
        key_name: "id".to_string(),
    }
}

fn test_team_model(test_name: &str) -> harbcore::models::Team {
    Team{
        id: "".to_string(),
        name: test_name.to_string(),
        members: vec![],
        projects: vec![],
        tokens: vec![],
    }
}

#[async_std::test]
async fn can_crud_team() -> Result<(), Error> {
    let ctx = test_context();
        // sdk_config_from_env()
        // .await
        // .expect("failed to load config from environment");

    let store = Store::new(&ctx).await;
    let store = store.unwrap();
    let service = TeamService::new(Arc::new(store));

    let mut model = test_team_model("can_crud_team");
    service.insert(&mut model).await?;

    let saved = service.find(model.id.clone().as_str()).await?;
    assert!(saved.is_some());

    let saved = saved.unwrap();
    assert_eq!(model.id, saved.id);
    assert_eq!(model.name, saved.name);

    let updated_name = format!("{}-{}", saved.name, "updated");
    let mut updated = saved.clone();
    updated.name = updated_name.clone();

    service.update(&updated).await?;

    let saved = service.find(model.id.clone().as_str()).await?;
    assert!(saved.is_some());

    let saved = saved.unwrap();
    assert_eq!(saved.id, updated.id);
    assert_eq!(updated.name, updated_name);

    let teams = service.list().await?;
    assert!(!teams.is_empty());

    service.delete(updated.id.as_str()).await?;

    let deleted = service.find(updated.id.as_str()).await?;
    assert!(deleted.is_none());
    Ok(())
}
