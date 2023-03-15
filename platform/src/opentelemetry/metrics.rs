use std::time::Duration;
use opentelemetry::runtime;
use opentelemetry_api::{Context, global};
use opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector;
use opentelemetry::sdk::metrics::controllers::BasicController;
use opentelemetry::sdk::metrics::selectors;
use opentelemetry::trace::FutureExt;
use opentelemetry_api::metrics::Meter;
use opentelemetry_otlp::{ExportConfig, WithExportConfig};

use crate::Error;

const DEFAULT_ENDPOINT:&str = "http://localhost:4317";

pub struct Pusher {
    ctx: Context,
    controller: BasicController,
    meter: Meter,
}

impl Pusher {
    pub fn new(meter: &str, endpoint: Option<&str>, ctx: Option<Context>) -> Result<Pusher, Error> {
        let ctx = match ctx {
            None => Context::new(),
            Some(c) => c,
        };

        let controller = new_controller(endpoint)?;
        let meter= global::meter(meter);

        Ok(Pusher {
            ctx,
            controller,
            meter,
        })
    }

    pub fn running(&self) -> bool {
        self.controller.is_running()
    }

    pub fn stop(&self) -> Result<(), Error> {
        self.controller
            .stop(&self.ctx)
            .map_err(|e| Error::OpenTelemetry(e.to_string()))
    }

    pub fn with_gauge<T>(name: &str, description: &str, value: T) -> Result<(), Error> {
        let gauge =
        Ok(())
    }
}

fn new_controller(endpoint: Option<&str>) -> Result<BasicController, Error> {
    let endpoint = match endpoint {
        None => DEFAULT_ENDPOINT,
        Some(e) => e,
    };

    let export_config = ExportConfig {
        endpoint: endpoint.to_string(),
        ..ExportConfig::default()
    };

    let controller = opentelemetry_otlp::new_pipeline()
        .metrics(
            selectors::simple::inexpensive(),
            cumulative_temporality_selector(),
            runtime::Tokio,
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
        )
        .with_period(Duration::from_secs(3))
        .with_timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| Error::OpenTelemetry(e.to_string()))?;

    Ok(controller)
}

#[cfg(test)]
mod tests {
    use opentelemetry_api::Context;
    use crate::opentelemetry::metrics::{new_controller, Pusher};
    use crate::Error;

    fn can_get_new_controller() -> Result<(), Error> {
        let controller = new_controller(None)?;

        assert!(controller.is_running());

        controller.stop(&Context::new()).expect("could not stop");
        Ok(())
    }

    fn can_get_pusher() -> Result<(), Error> {
        let pusher = Pusher::new("metrics_test", None, None)?;

        assert!(pusher.running());

        pusher.stop().expect("could not stop");
        Ok(())
    }

    // TODO: Marking as manual until we can find a way to test OpenTelemetry in memory.
    #[async_std::test]
    #[ignore = "manual run only"]
    async fn can_push_metric() -> Result<(), Error> {
        let pusher = Pusher::new("metrics_test", None, None)?;

        assert!(pusher.running());


        pusher.stop().expect("could not stop");
        Ok(())
    }
}