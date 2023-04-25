use platform::mongodb::auth::init_default_auth::apply_all;
use platform::mongodb::migrations::{LogEntry, MigrationService};
use platform::mongodb::Context;
use platform::Error;

#[allow(dead_code)]
pub async fn sync(ctx: Context) -> Result<Vec<LogEntry>, Error> {
    let service = MigrationService::new(&ctx).await?;
    let mut log_entries = vec![];

    // Initialize the auth subsystem
    let entry = apply_all(&service).await?;
    log_entries.push(entry);

    Ok(log_entries)
}
