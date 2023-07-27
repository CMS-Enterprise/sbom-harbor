use dagger_sdk;
use platform::secrets::aws;

use dagger_build::runner::Runner;
use dagger_build::Error;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let runner = Runner::new().await?;

    match runner.run().await {
        Ok(_) => {
            println!("build complete");
            Ok(())
        }
        Err(e) => {
            println!("build failed with err {}", e);
            Err(e)
        }
    }
}
