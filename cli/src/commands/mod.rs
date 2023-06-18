use crate::Error;
use platform::persistence::mongodb::Store;
use std::sync::Arc;

/// Contains the types and functions to support the `enrich` Command.
pub mod enrich;

/// Contains the types and functions to support the `ingest` Command.
pub mod ingest;

/// Contains the type and functions to support `analyze` Command.
pub mod analyze;

pub(crate) struct CliContext {
    pub store: Arc<Store>,
}

impl CliContext {
    pub async fn new(debug: bool) -> Result<CliContext, Error> {
        let cx = match debug {
            false => harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string())),
            true => harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string())),
        }?;

        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Config(e.to_string()))?,
        );

        Ok(CliContext { store })
    }
}
