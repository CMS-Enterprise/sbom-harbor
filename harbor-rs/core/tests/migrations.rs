use aqum::Error;
use aqum::mongodb::Context;
use aqum::mongodb::migrations::{Direction, MigrationService};

use harbor_core::auth::migrations::init_auth;

fn test_context() -> Context {
    Context{
        connection_uri: "mongodb://localhost:27017".to_string(),
        database: "harbor".to_string(),
    }
}
#[async_std::test]
async fn can_init_auth() -> Result<(), Error> {
    let ctx = test_context();
    let service = MigrationService::new(&ctx).await?;

    let log_entry = init_auth(&service).await?;

    assert!(!log_entry.id.is_empty());
    assert_eq!("init_auth", log_entry.name);
    assert_eq!(log_entry.direction, Direction::Up);

    Ok(())
}
