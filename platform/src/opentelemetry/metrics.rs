use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use lazy_static::lazy_static;
use opentelemetry::runtime;
use opentelemetry_api::{Context, global, KeyValue};
use opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector;
use opentelemetry::sdk::metrics::controllers::BasicController;
use opentelemetry_api::metrics::{Counter, Meter};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::metrics::aggregators::Aggregator;
use opentelemetry_sdk::metrics::sdk_api::Descriptor;
use opentelemetry_sdk::Resource;
use tonic::metadata::{MetadataKey, MetadataMap, MetadataValue};

use crate::Error;

// Default to local running instance provided by devenv.
const DEFAULT_ENDPOINT:&str = "http://localhost:4317";
static BATCH_EXPORT_CONTROLLER: once_cell::sync::OnceCell<BasicController> = once_cell::sync::OnceCell::new();

// WATCH: Will need to allow configuring the list of allowed suffixes if as downstream users
// begin using systems other than Prometheus.
// See https://prometheus.io/docs/practices/naming/#metric-names for how we derived this list.
lazy_static!(
    static ref PROMETHEUS_SUFFIXES: Vec<&'static str> = init_prometheus_suffixes();
);

// Initializes the static vector of conventional suffixes.
fn init_prometheus_suffixes() -> Vec<&'static str> {
    vec![
        "total",
        "seconds",
        "bytes",
        "ratio",
        "info",
    ]
}

/// Ensures metrics conform to naming conventions.
fn validate_metric_name(name: &str) -> Result<(), Error> {
    for valid in &*PROMETHEUS_SUFFIXES {
        if name.ends_with(*valid) {
            return Ok(());
        }
    }

    Err(Error::OpenTelemetry(format!("invalid metric name {}", name)))
}

pub struct BatchExportConfig {
    cx: Context,
    meter_name: &'static str,
    push_interval_seconds: u64,
    endpoint: Option<String>,
    resource_attrs: HashMap<String, String>,
    collector_metadata: Option<HashMap<String, String>>
}

/// BatchExporter provides a high-level abstraction over the complexities of constructing the metrics pipeline
/// specifically for batch tasks. Long-running services are expected to use [tracing] instead.
pub struct BatchExporter {
    config: BatchExportConfig,
    meter: Meter,
    u64_counters: HashMap<String, Counter<u64>>,
    f64_counters: HashMap<String, Counter<f64>>,
}

pub trait Metric {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn kind(&self) -> String;
    fn label(&self) -> String;
    fn value(&self) -> MetricValue;
}

/// Enumerates the four Prometheus metric types. See https://prometheus.io/docs/concepts/metric_types/.
pub enum Kind {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

pub enum Value {
    U64(u64),
    F64(f64),
}

pub struct CountMetric {
    name: String,
    value: Value,
}

// impl Metric for CountMetric {
//
// }

/// Ensures metric aggregators are consistent according to naming convention.
#[derive(Debug, Default)]
struct AggregatorSelector;

// TODO: Make this dynamic and comprehensive over all valid metrics types.
impl opentelemetry::sdk::export::metrics::AggregatorSelector for AggregatorSelector {
    fn aggregator_for(&self, descriptor: &Descriptor) -> Option<Arc<dyn Aggregator + Send + Sync>> {
        match descriptor.name() {
            name if name.ends_with("total") => Some(Arc::new(opentelemetry::sdk::metrics::aggregators::sum())),
            name if name.ends_with("seconds") => Some(Arc::new(opentelemetry::sdk::metrics::aggregators::histogram(&[]))),
            _ => panic!("Invalid instrument name for test AggregatorSelector: {}", descriptor.name()),
        }
    }
}

impl BatchExporter {
    /// Constructs a new metrics controller for the meter.
    pub fn new(config: BatchExportConfig) -> Result<BatchExporter, Error> {
        let meter= global::meter(config.meter_name);
        let exporter = BatchExporter {
            config,
            meter,
            u64_counters: HashMap::new(),
            f64_counters: HashMap::new(),
        };

        let controller = exporter.init_controller()?;

        match BATCH_EXPORT_CONTROLLER.set(controller) {
            Ok(_) => {
                Ok(exporter)
            }
            Err(_) => Err(Error::OpenTelemetry("batch exporter already initialized".to_string()))
        }

    }

    /// Constructs a new BasicController with metadata for the resource being observed (e.g. service name,
    /// enrichment type, instance tags) and the OpenTelemetry Collector (e.g. Api-Key).
    fn init_controller(&self) -> Result<BasicController, Error> {
        let endpoint = match self.config.endpoint.clone() {
            None => DEFAULT_ENDPOINT.to_string(),
            Some(e) => e,
        };

        let resource_attrs = key_value(&self.config.resource_attrs);
        let collector_metadata = metadata_map(&self.config.collector_metadata)?;

        let export_config = ExportConfig {
            endpoint,
            ..ExportConfig::default()
        };

        let controller = opentelemetry_otlp::new_pipeline()
            .metrics(
                opentelemetry_sdk::metrics::selectors::simple::inexpensive(),
                cumulative_temporality_selector(), // @@@@ Would have to change if we need up/down temporality
                runtime::Tokio,
            )
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_export_config(export_config)
                    .with_metadata(collector_metadata),
            )
            .with_period(Duration::from_secs(self.config.push_interval_seconds))
            .with_timeout(Duration::from_secs(10))
            .with_resource(Resource::new(resource_attrs))
            .build()
            .map_err(|e| Error::OpenTelemetry(e.to_string()))?;

        Ok(controller)
    }

    /// Used by unit tests to ensure a controller instance has been initialized.
    pub(crate) fn running(&self) -> bool {
        BATCH_EXPORT_CONTROLLER
            .get()
            .unwrap()
            .is_running()
    }

    /// Used by unit tests to force a metrics collection.
    pub(crate) fn collect(&self) -> Result<(), Error> {
        BATCH_EXPORT_CONTROLLER
            .get()
            .unwrap()
            .collect(&self.config.cx)
            .map_err(|e| Error::OpenTelemetry(format!("BatchExporter::collect - {}", e.to_string())))
    }

    /// Stops the metrics controller.
    pub fn stop(&self) -> Result<(), Error> {
        BATCH_EXPORT_CONTROLLER
            .get()
            .unwrap()
            .stop(&self.config.cx)
            .map_err(|e| Error::OpenTelemetry(e.to_string()))
    }

    /// Initializes a u64 counter that can be incremented.
    pub fn with_u64_counter(&mut self, name: String) -> Result<(), Error> {
        let _ = validate_metric_name(name.as_str())?;

        if self.u64_counters.contains_key(&name) {
            return Ok(());
        }

        let counter = global::meter(self.config.meter_name)
            .u64_counter(name.clone())
            .try_init()
            .map_err(|e| Error::OpenTelemetry(format!("BatchExporter::with_u64_counter - {}", e.to_string())))?;

        self.u64_counters.insert(name, counter);

        Ok(())
    }

    pub fn add(&self, key: &str, value: u64, metric: CountMetric) -> Result<(), Error> {
        if !self.u64_counters.contains_key(name) {
            return Err(Error::OpenTelemetry(format!("BatchExporter::add_u64 - invalid key {}", key)));
        }

        self.u64_counters.get(key).unwrap().add(&self.config.cx, value, None)
    }

    /// Initializes an f64 counter that can be incremented.
    pub fn with_f64_counter(&mut self, name: String) -> Result<(), Error> {
        let _ = validate_metric_name(name.as_str())?;

        if self.f64_counters.contains_key(&name) {
            return Ok(());
        }

        let counter = global::meter(self.config.meter_name)
            .f64_counter(name.clone())
            .try_init()
            .map_err(|e| Error::OpenTelemetry(format!("BatchExporter::with_f64_counter - {}", e.to_string())))?;

        self.f64_counters.insert(name, counter);

        Ok(())
    }
}

/// Converts a [HashMap] to a Tonic MetadataMap.
fn metadata_map(hash_map: &Option<HashMap<String, String>>) -> Result<MetadataMap, Error> {
    let mut map = MetadataMap::new();

    match hash_map {
        None => {}
        Some(metadata) => {
            metadata
                .iter()
                .map(|(k, v)| {
                    let key = k.clone();
                    let key = MetadataKey::from_bytes(key.as_bytes())
                        .map_err(|e| Error::OpenTelemetry(format!{"metrics::metadata_map::key - {}", e}))?;

                    let val = v.clone();
                    let val = MetadataValue::try_from(val.as_bytes())
                        .map_err(|e| Error::OpenTelemetry(format!{"metrics::metadata_map::key - {}", e}))?;

                    map.insert(key, val);
                    Ok(())
                }).collect::<Result<(), Error>>()?;
        }
    }

    Ok(map)
}

/// Converts a [HashMap] to an OpenTelemetry [KeyValue] vector.
fn key_value(hash_map: &HashMap<String, String>) -> Vec<KeyValue> {
    let mut kvs = vec![];

    hash_map
        .iter()
        .for_each(|(k, v)| {
            kvs.push(KeyValue::new(k.clone(), v.clone()));
        });

    kvs
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Duration;
    use opentelemetry::metrics::Counter;
    use opentelemetry_api::{Context, global, KeyValue};
    use opentelemetry_sdk::export::metrics::aggregation::{cumulative_temporality_selector, Sum};
    use opentelemetry_sdk::export::metrics::InstrumentationLibraryReader;
    use opentelemetry_sdk::metrics::aggregators::{Aggregator, SumAggregator};
    use opentelemetry_sdk::metrics::sdk_api::NumberKind;
    use crate::opentelemetry::metrics::{BatchExporter, BatchExportConfig};
    use crate::Error;

    static TEST_COUNTER: once_cell::sync::Lazy<Counter<u64>> = once_cell::sync::Lazy::new(|| {
        global::meter("unit_tests")
            .u64_counter("metrics_test_total")
            .init()
    });

    fn test_config() -> BatchExportConfig {
        BatchExportConfig {
            cx: Context::new(),
            meter_name: "unit_tests",
            push_interval_seconds: 1,
            endpoint: None,
            resource_attrs: HashMap::from([("service.name".to_string(), "unit_tests".to_string())]),
            collector_metadata: None,
        }
    }

    fn can_get_batch_exporter() -> Result<(), Error> {
        let config = test_config();
        let exporter = BatchExporter::new(config)?;

        assert!(exporter.running());

        exporter.stop().expect("could not stop");
        Ok(())
    }

    // TODO: Marking as manual until we can find a way to test OpenTelemetry in memory.
    #[async_std::test]
    #[ignore = "manual run only"]
    async fn can_push_metric() -> Result<(), Error> {
        let config = test_config();
        let mut exporter = BatchExporter::new(config)?;

        assert!(exporter.running());

        exporter.with_u64_counter("metrics_test_total".to_string()).expect("unable to add u64_counter");

        for _ in 0..100 {
            exporer.add(&exporter.config.cx, 1, &[KeyValue::new("metrics_test_total", 1)]);
            std::thread::sleep(Duration::from_secs(5));
        }

        // let mut results:Vec<(String, u64)> = Vec::new();
        // BATCH_EXPORT_CONTROLLER.get().unwrap().try_for_each(&mut |_library, reader| {
        //     let selector = cumulative_temporality_selector();
        //     reader.try_for_each(&selector, &mut |record| {
        //
        //         if let Some(sum_agg) = record
        //             .aggregator()
        //             .unwrap()
        //             .as_any()
        //             .downcast_ref::<SumAggregator>() {
        //
        //             results.push((
        //                 record.descriptor().name().to_owned(),
        //                 sum_agg.sum().unwrap().to_u64(&NumberKind::U64),
        //             ));
        //         }
        //
        //         Ok(())
        //     })?;
        //     Ok(())
        // }).expect("cannot read metrics");
        //
        // assert_eq!(1, results[0].1);

        exporter.stop().expect("could not stop");
        Ok(())
    }
}