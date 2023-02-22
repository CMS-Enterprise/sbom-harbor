use aqum::mongodb::Context;
use aqum::mongodb::migrations::{LogEntry, MigrationService};

mod init_auth;

use aqum::Error;

pub async fn sync(ctx: Context) -> Result<Vec<LogEntry>, Error> {
    let service = MigrationService::new(&ctx).await?;
    let mut log_entries = vec![];

    // Initialize the auth subsystem
    let entry = init_auth::up(&service).await?;
    log_entries.push(entry);

    Ok(log_entries)
}
