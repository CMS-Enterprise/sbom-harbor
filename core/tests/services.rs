use harbcore::config::dev_context;
use harbcore::entities::teams::*;
use harbcore::services::teams::TeamService;
use harbcore::Error;
use platform::mongodb::Service;

fn test_team_model(test_name: &str) -> Team {
    Team {
        id: "".to_string(),
        name: test_name.to_string(),
        members: vec![],
        projects: vec![],
        tokens: vec![],
    }
}

#[async_std::test]
async fn can_crud_team() -> Result<(), Error> {
    let cx = dev_context(Some("core-test"))?;
    let service = TeamService::new(cx);

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
