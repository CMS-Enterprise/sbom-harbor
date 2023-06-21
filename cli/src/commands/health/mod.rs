use crate::Error;
use clap::Parser;
use harbcore::config::*;
use platform::persistence::mongodb::client_from_context;
use platform::persistence::s3::Store;

/// The CommandFactory function for the `health` command.
pub async fn execute(args: &HealthArgs) -> Result<(), Error> {
    // ensure snyk token is set
    match snyk_token() {
        Ok(_) => {}
        Err(e) => {
            return Err(Error::System(e.to_string()));
        }
    }

    // ensure db is reachable
    let cx = match &args.debug {
        false => harbor_context().map_err(|e| Error::Config(e.to_string()))?,
        true => dev_context(None).map_err(|e| Error::Config(e.to_string()))?,
    };

    let client = client_from_context(&cx).await?;
    match client.list_database_names(None, None).await {
        Ok(_) => {}
        Err(e) => {
            return Err(Error::System(e.to_string()));
        }
    }

    // ensure s3 bucket is reachable
    let s3_store = Store::new_from_env().await?;
    let bucket_name = harbor_bucket().map_err(|e| Error::Config(e.to_string()))?;
    match s3_store.list(bucket_name).await {
        Ok(_) => {}
        Err(e) => {
            return Err(Error::System(e.to_string()));
        }
    }
    Ok(())
}
/// Specifies the CLI args for the `health` command.
#[derive(Debug, Parser)]
pub struct HealthArgs {
    /// Specifies to run the command against the local debug environment.
    #[arg(long)]
    debug: bool,
}
