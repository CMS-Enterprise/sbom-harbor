use harbcore::config::dev_context;
use harbcore::entities::teams::*;
use harbcore::services::teams::TeamService;
use harbcore::Error;
use platform::persistence::mongodb::{Service, Store};
use std::sync::Arc;

fn test_team_model(test_name: &str) -> Team {
    Team {
        id: "".to_string(),
        name: test_name.to_string(),
        repositories: None,
        tokens: None,
        members: None,
        products: None,
    }
}

#[async_std::test]
async fn can_crud_team() -> Result<(), Error> {
    let cx = dev_context(None)?;
    let store = Arc::new(Store::new(&cx).await?);
    let service = TeamService::new(store.clone());

    let mut model = test_team_model("can_crud_team");
    service.insert(&mut model).await?;

    let saved = service.find(model.id.clone().as_str()).await?;
    assert!(saved.is_some());

    let saved = saved.unwrap();
    assert_eq!(model.id, saved.id);
    assert_eq!(model.name, saved.name);

    let updated_name = format!("{}-{}", saved.name.clone(), "updated");
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
