use crate::Error;
use clap::Parser;
use platform::persistence::mongodb::Store;

/// The CommandFactory function for the `health` command.
pub async fn execute(args: &HealthArgs) -> Result<(), Error> {
    let cx = match &args.debug {
        false => harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?,
        true => harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string()))?,
    };

    match Store::new(&cx)
        .await
        .map_err(|e| Error::Enrich(e.to_string()))
    {
        Ok(_) => {
            println!("Mongo: OK");
            Ok(())
        }
        Err(e) => {
            println!("Mongo: cannot connect");
            Err(e)
        }
    }

    // Ok(())
}
/// Specifies the CLI args for the `health` command.
#[derive(Debug, Parser)]
pub struct HealthArgs {
    /// Specifies to run the command against the local debug environment.
    #[arg(long)]
    debug: bool,
}
