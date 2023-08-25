use crate::commands::construct::ConstructArgs;
use crate::Error;
use harbcore::entities::tasks::{Task, TaskKind};
use harbcore::tasks::TaskProvider;
use platform::persistence::mongodb::Store;
use std::sync::Arc;

use harbcore::entities::datasets::ConstructionProviderKind;

use harbcore::services::purl2cpe::service::Purl2CpeService;
use harbcore::tasks::construction::dataset::purl2cpe::ConstructionTask;

pub(crate) async fn execute(_args: &ConstructArgs) -> Result<(), Error> {
    println!("==> Beginning purl2cpe dataset construction");

    let cx = harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string()))?;

    let store = Arc::new(
        Store::new(&cx)
            .await
            .map_err(|e| Error::Construction(e.to_string()))?,
    );

    let service = Purl2CpeService::new(store);
    let provider = ConstructionTask::new(service);
    let provider_kind = ConstructionProviderKind::Purl2Cpe;
    let task_kind = TaskKind::Construction(provider_kind);
    let mut task: Task = Task::new(task_kind).map_err(|e| Error::Construction(e.to_string()))?;

    provider
        .execute(&mut task)
        .await
        .map_err(|e| Error::Construction(e.to_string()))?;

    println!("==> Purl2Cpe data set construction complete");
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::commands::construct::{execute, ConstructArgs, ConstructionProviderKind};
    use crate::Error;

    #[tokio::test]
    #[ignore = "debug manual only"]
    async fn execute_purl2cpe_dataset_construction() -> Result<(), Error> {
        let args = ConstructArgs {
            provider: ConstructionProviderKind::Purl2Cpe,
        };

        execute(&args)
            .await
            .expect("Panic at github ingest provider execute!");

        Ok(())
    }
}
