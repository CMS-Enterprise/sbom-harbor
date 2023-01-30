use async_std;

mod common;

use aquia::config::Config;
use aquia::dynamo::{Error, Store};
use harbor::entities::{Codebase, Discriminator, Entity, Project, Team};

#[async_std::test]
async fn can_get_team() -> anyhow::Result<()> {
    let config = Config::new().await?;
    let store = Store::new(config);

    let mut team = Team::new("".to_string());
    team.partition_key = "dawn-patrol".to_string();

    let team: Result<Option<Team>, Error> = store
        .find(Box::new(team)).await;

    assert!(!team.is_err(), "{:?}", team);

    let team = team.unwrap();
    assert!(team.is_some());

    let team = team.unwrap();

    assert!(!team.name.is_empty());
    assert!(team.is_child_of(team.id.clone()));

    Ok(())
}

#[test]
fn can_get_discriminator() -> Result<(), String> {
    let codebase = test_codebase(Some("test-get-discriminator".to_string()), None, None);

    println!("codebase discriminator: {:?}", codebase.discriminator());
    assert_eq!(codebase.discriminator(), Discriminator::Codebase);

    let team = Team::new("discriminator-test".to_string());

    println!("team discriminator: {}", team.discriminator());
    assert_eq!(team.discriminator(), Discriminator::Team);

    Ok(())
}

#[test]
fn can_get_entity_type_name() -> Result<(), String> {
    let entity = test_codebase(Some("test-get-entity-type-name".to_string()), None, None);
    let type_name = entity.type_name();

    println!("codebase type_name: {:?}", type_name);
    assert!(type_name.contains("Codebase"));

    Ok(())
}

#[test]
fn can_project_entity_to_api() -> Result<(), String> {
    let entity = test_codebase(Some("test-project-entity-to-api".to_string()), None, None);

    // Ensure the serialized entity struct has the required Dynamo structural attributes.
    let entity_json = serde_json::to_string(&entity).unwrap();
    println!("entity: {}", entity_json);
    assert!(entity_json.contains("TeamId"));

    // Ensure the serialized api struct omits the Dynamo structural attributes.
    let api_json = entity.to_body().unwrap();
    println!("api: {}", api_json);
    assert!(!api_json.contains("TeamId"));

    let entity: Codebase = serde_json::from_str(&entity_json).unwrap();
    let dto: harbor::api::Codebase = serde_json::from_str(&api_json).unwrap();

    assert_eq!(entity.id, dto.id);
    assert_eq!(entity.name, dto.name);
    assert_eq!(entity.language, dto.language);
    assert_eq!(entity.build_tool, dto.build_tool);
    assert_eq!(entity.clone_url, dto.clone_url);

    Ok(())
}

#[test]
fn can_project_subordinate_entity_to_api() -> Result<(), String> {
    // Build a test graph
    let graph = test_graph(Some("project-subordinate-entity-to-api".to_string()));
    println!("graph: {:?}", graph);
    assert_eq!(graph.projects.len(), 1);
    assert_eq!(graph.projects[0].codebases.len(), 1);
    assert!(!graph.id.is_empty());
    assert!(!graph.projects[0].id.is_empty());
    assert!(!graph.projects[0].codebases[0].id.is_empty());
    assert_eq!(graph.partition_key, graph.projects[0].partition_key);
    assert_eq!(graph.partition_key, graph.projects[0].codebases[0].partition_key);
    assert_eq!(graph.id, graph.projects[0].parent_id);
    assert_eq!(graph.projects[0].partition_key, graph.projects[0].codebases[0].partition_key);
    assert_eq!(graph.projects[0].id, graph.projects[0].codebases[0].parent_id);

    // Ensure the serialized entity structs have the required Dynamo structural attributes.
    let entity_json = serde_json::to_string(&graph).unwrap();
    println!("entity: {}", entity_json);
    assert!(entity_json.contains("TeamId"));

    // Ensure the serialized api structs omit the Dynamo structural attributes.
    let api_json = graph.to_body().unwrap();
    println!("api: {}", api_json);
    assert!(!api_json.contains("TeamId"));

    // Validate team
    let entity: Team = serde_json::from_str(&entity_json).unwrap();
    let dto: harbor::api::Team = serde_json::from_str(&api_json).unwrap();
    assert_eq!(entity.id, dto.id);
    assert_eq!(entity.name, dto.name);

    // Validate project
    let entity = entity.projects[0].clone();
    let dto = dto.projects[0].clone();

    assert_eq!(entity.id, dto.id);
    assert_eq!(entity.name, dto.name);
    assert_eq!(entity.codebases.len(), dto.codebases.len());

    // Validate codebase
    let entity = entity.codebases[0].clone();
    let dto = dto.codebases[0].clone();

    assert_eq!(entity.id, dto.id);
    assert_eq!(entity.name, dto.name);
    assert_eq!(entity.language, dto.language);
    assert_eq!(entity.build_tool, dto.build_tool);
    assert_eq!(entity.clone_url, dto.clone_url);

    Ok(())
}


fn test_graph(mut name: Option<String>) -> Team {
    if name.is_none() {
        name = Some("sbom-harbor".to_string());
    }

    let mut team = test_team(name.clone());
    let mut project = Project::new(&team, name.clone().unwrap(), None);
    team.projects(project);

    let mut project = &mut team.projects[0];
    let codebase = Codebase::new(project,
                                               name.clone().unwrap(),
                                               Some("rust".to_string()),
                                               Some("cargo".to_string()),
                                               Some("https://github.com/cms-enterprise/sbom-harbor".to_string()));

    project.codebases(codebase);
    team
}

fn test_team(mut name: Option<String>) -> Team {
    if name.is_none() {
        name = Some("sbom-harbor".to_string());
    }

    let team = Team::new(name.unwrap().to_string());

    team.clone()
}


fn test_project(mut name: Option<String>, mut team: Option<Team>) -> Project {
    if name.is_none() {
        name = Some("sbom-harbor".to_string());
    }

    let mut team = match team {
        None => test_team(name.clone()),
        Some(t) => t,
    };

    let project = Project::new(&team, name.unwrap(), None);
    let id = project.id.clone();
    team.projects(project);

    team.projects
        .into_iter()
        .find(|p| p.id == id).unwrap()
}

fn test_codebase(mut name: Option<String>, mut project: Option<Project>, mut team: Option<Team>) -> Codebase {
    if name.is_none() {
        name = Some("sbom-harbor".to_string());
    }

    let mut project = match project {
        None => Some(test_project(name.clone(), team)).unwrap(),
        Some(p) => p
    };

    Codebase::new(&project,
          name.unwrap(),
        Some("rust".to_string()),
        Some("cargo".to_string()),
        Some("https://github.com/cms-enterprise/sbom-harbor.git".to_string())
    )
}
