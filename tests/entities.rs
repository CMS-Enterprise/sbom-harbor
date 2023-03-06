use async_std;

mod common;

use aqum::Error;
use harbor::api;
use harbor::entities::{Codebase, Discriminator, Entity, Project, Team};

#[async_std::test]
async fn can_get_team() -> anyhow::Result<()> {
    let config = Config::new().await?;
    let store = Store::new(config);

    let mut team = Team::new("".to_string());
    team.partition_key = "dawn-patrol".to_string();

    let team: Result<Option<Team>, Error> = store
        .find(&mut team).await;

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
    let dto: api::Codebase = from_entity(&entity).map_err(|e| e.to_string())?;

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
    let entity = test_graph(Some("project-subordinate-entity-to-api".to_string()));
    println!("graph: {:?}", entity);
    assert_eq!(entity.projects.len(), 1);
    assert_eq!(entity.projects[0].codebases.len(), 1);
    assert!(!entity.id.is_empty());
    assert!(!entity.projects[0].id.is_empty());
    assert!(!entity.projects[0].codebases[0].id.is_empty());
    assert_eq!(entity.partition_key, entity.projects[0].partition_key);
    assert_eq!(entity.partition_key, entity.projects[0].codebases[0].partition_key);
    assert_eq!(entity.id, entity.projects[0].parent_id);
    assert_eq!(entity.projects[0].partition_key, entity.projects[0].codebases[0].partition_key);
    assert_eq!(entity.projects[0].id, entity.projects[0].codebases[0].parent_id);

    let dto: harbor::api::Team = from_entity(&entity).map_err(|e| e.to_string())?;
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
    let project = Project::new(&team, name.clone().unwrap(), None);
    team.projects(project);

    let project = &mut team.projects[0];
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


fn test_project(mut name: Option<String>, team: Option<Team>) -> Project {
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

fn test_codebase(mut name: Option<String>, project: Option<Project>, team: Option<Team>) -> Codebase {
    if name.is_none() {
        name = Some("sbom-harbor".to_string());
    }

    let project = match project {
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
