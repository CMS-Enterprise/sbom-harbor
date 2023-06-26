use crate::Error;
use platform::persistence::mongodb::Store;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::{Serializer, Value};
use std::sync::Arc;

/// Application logic related to enrichments.
pub mod enrichments;

/// Application logic related to the ingestion and management of sboms.
pub mod ingestion;

/// Testing logic and types related to testing the cli application.
pub mod testing;

/// Application logic related to the analysis of sboms and related enrichment data.
pub mod analytics;

/// Encapsulates the execution context of a CLI command.
pub(crate) struct CommandContext {
    pub store: Arc<Store>,
}

impl CommandContext {
    pub async fn new(debug: bool) -> Result<CommandContext, Error> {
        let cx = match debug {
            false => harbcore::config::harbor_context().map_err(|e| Error::Config(e.to_string())),
            true => harbcore::config::dev_context(None).map_err(|e| Error::Config(e.to_string())),
        }?;

        let store = Arc::new(
            Store::new(&cx)
                .await
                .map_err(|e| Error::Config(e.to_string()))?,
        );

        Ok(CommandContext { store })
    }
}

/// Pretty prints a json string. Prints the passed String if unable to serialize. Callers must
/// are responsible for obfuscating sensitive data.
pub(crate) fn pretty_print_json(raw: &str) {
    let value: Value = match serde_json::from_str(raw) {
        Ok(value) => value,
        Err(e) => {
            println!("error in pretty_print_json: {}", e);
            println!("raw value: {}", raw);
            return;
        }
    };

    let mut buf = Vec::new();

    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(&mut buf, formatter);

    value.serialize(&mut ser).unwrap();

    println!("{}", String::from_utf8(buf).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    #[ignore = "debug"]
    fn debug_pretty_print_json() -> Result<(), Error> {
        let raw = testing::sbom_raw()?;

        pretty_print_json(raw.as_str());

        Ok(())
    }
}
