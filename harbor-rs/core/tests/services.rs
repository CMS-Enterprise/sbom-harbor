use std::sync::Arc;
use aqum::dynamo::{Service, Store};
use harbor_core::entities::{Team as TeamEntity};
use harbor_core::models::*;
use harbor_core::services::*;
use harbor_core::config::sdk_config_from_env;
use harbor_core::Error;

fn test_team_model(test_name: &str) -> Team {
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
    let config = sdk_config_from_env()
        .await
        .expect("failed to load config from environment");

    let store = Store::new(config);
    let service = TeamService::new(Arc::new(store));

    let model = test_team_model("can_crud_team");

    let ctx = CreateTeamContext{
        team: model.clone(),
        children: false,
    };

    let model = service.insert(&ctx).await?;
    assert!(!model.id.is_empty());

    let ctx = TeamContext{
        id: model.id.clone(),
        children: false
    };

    let saved = service.find(&ctx).await?;
    assert!(saved.is_some());

    let mut saved = saved.unwrap();
    assert_eq!(model.id, saved.id);
    assert_eq!(model.name, saved.name);

    let updated_name = format!("{}-{}", saved.name, "updated");
    saved.name = updated_name.clone();

    let ctx = UpdateTeamContext{
        id: saved.id.clone(),
        team: saved.clone(),
        children: false,
    };

    let updated = service.update(&ctx).await?;
    assert_eq!(saved.id, updated.id);
    assert_eq!(updated.name, updated_name);


    let ctx = ListTeamsContext::new(false);
    let teams = service.list(&ctx).await?;
    assert!(!teams.is_empty());

    let ctx = TeamContext{
        id: updated.id.clone(),
        children: false
    };

    service.delete(&ctx).await?;

    let deleted = service.find(&ctx).await?;
    assert!(deleted.is_none());
    Ok(())
}
