use aqum::Error;
use aqum::mongodb::auth::init_default_auth::apply_all;
use aqum::mongodb::Context;
use aqum::mongodb::migrations::{Effect, MigrationService};


// TODO: Dynamic test config.  Currently requires you to run docker instance before running tests.
fn test_context() -> Context {
    Context{
        connection_uri: "mongodb://localhost:27017".to_string(),
        db_name: "harbor".to_string(),
        key_name: "id".to_string(),
    }
}

#[async_std::test]
async fn can_init_auth() -> Result<(), Error> {
    // TODO: Idempotent teardown.
    let ctx = test_context();

    // TODO: Assert clean DB.
    let service = MigrationService::new(&ctx).await?;

    // Initialize the db.
    let log_entry = apply_all(&service).await?;

    assert!(!log_entry.id.is_empty());
    assert_eq!("init_default_auth", log_entry.name);
    assert_eq!(log_entry.effect, Effect::Commit);

    // TODO: Assert single log entry and populated db.

    // TODO: Rerun and assert same ids, with 2 log entries.

    Ok(())
}
