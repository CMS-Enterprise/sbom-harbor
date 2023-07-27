use dagger_build::{Error, Runner};

#[async_std::test]
async fn can_run_main() -> Result<(), Error> {
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
