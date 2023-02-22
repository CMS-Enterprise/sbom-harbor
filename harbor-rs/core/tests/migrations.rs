use aqum::Error;
use aqum::mongodb::Context;
use aqum::mongodb::migrations::{Direction, MigrationService};

use harbor_core::auth::migrations::init_auth;

// TODO: Dynamic test config.  Currently requires you to run docker instance before running tests.
fn test_context() -> Context {
    Context{
        connection_uri: "mongodb://localhost:27017".to_string(),
        database: "harbor".to_string(),
    }
}

#[async_std::test]
async fn can_init_auth() -> Result<(), Error> {
    // TODO: Idempotent teardown.
    let ctx = test_context();

    // TODO: Assert clean DB.
    let service = MigrationService::new(&ctx).await?;

    // Initialize the db.
    let log_entry = init_auth(&service).await?;

    assert!(!log_entry.id.is_empty());
    assert_eq!("init_auth", log_entry.name);
    assert_eq!(log_entry.direction, Direction::Up);

    // TODO: Assert single log entry and populated db.

    // TODO: Rerun and assert same ids, with 2 log entries.

    Ok(())
}
