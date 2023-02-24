use aqum::mongodb::Context;
use aqum::mongodb::auth::init_default_auth::apply_all;
use aqum::mongodb::migrations::{LogEntry, MigrationService};
use aqum::Error;

#[allow(dead_code)]
pub async fn sync(ctx: Context) -> Result<Vec<LogEntry>, Error> {
    let service = MigrationService::new(&ctx).await?;
    let mut log_entries = vec![];

    // Initialize the auth subsystem
    let entry = apply_all(&service).await?;
    log_entries.push(entry);

    Ok(log_entries)
}
