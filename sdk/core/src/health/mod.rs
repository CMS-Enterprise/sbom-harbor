use crate::config::{dev_context, harbor_bucket, harbor_context};
use crate::Error;
use platform::persistence::mongodb::client_from_context;
use platform::persistence::s3::Store;

/// Performs a basic system health check.
pub async fn check(debug: bool) -> Result<(), Error> {
    // ensure db is reachable
    print!("DB ACCESS: ");
    let cx = match debug {
        false => harbor_context()?,
        true => dev_context(None)?,
    };

    let client = client_from_context(&cx).await?;
    match client.list_database_names(None, None).await {
        Ok(_) => {
            print!("OK\n");
        }
        Err(e) => {
            return Err(Error::Config(e.to_string()));
        }
    }

    // ensure s3 bucket is reachable
    println!("S3 ACCESS: ");
    let s3_store = Store::new_from_env().await?;
    let bucket_name = harbor_bucket().map_err(|e| Error::Config(e.to_string()))?;
    match s3_store.list(bucket_name).await {
        Ok(_) => {
            println!("OK\n");
        }
        Err(e) => {
            return Err(Error::Config(e.to_string()));
        }
    }

    println!("HEALTHY");

    Ok(())
}
