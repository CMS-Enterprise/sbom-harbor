use harbcore::config_util::dev_context;
use platform::persistence::mongodb::auth::init_default_auth::apply_all;
use platform::persistence::mongodb::migrations::{Effect, MigrationService};
use platform::Error;

#[async_std::test]
async fn can_init_auth() -> Result<(), Error> {
    let cx = dev_context(None).map_err(|e| Error::Config(e.to_string()))?;

    // TODO: Assert clean DB.
    let service = MigrationService::new(&cx).await?;

    // Initialize the db.
    let log_entry = apply_all(&service).await?;

    assert!(!log_entry.id.is_empty());
    assert_eq!("init_default_auth", log_entry.name);
    assert_eq!(log_entry.effect, Effect::Commit);

    // TODO: Assert single log entry and populated db.
    // TODO: Rerun and assert same ids, with 2 log entries.

    Ok(())
}
