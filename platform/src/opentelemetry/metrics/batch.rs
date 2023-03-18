use std::collections::HashMap;
use std::time::Duration;

use opentelemetry::runtime;
use opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector;
use opentelemetry::sdk::export::metrics::AggregatorSelector;
use opentelemetry::sdk::metrics::controllers::BasicController;
use opentelemetry_api::{Context, global, KeyValue};
use opentelemetry_api::metrics::{Counter, Meter};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::Resource;
use tonic::metadata::{MetadataKey, MetadataMap, MetadataValue};

use crate::Error;

// Default to local OpenTelemetry Collector instance provided by devenv.
const DEFAULT_ENDPOINT: &str = "http://localhost:4317";
static BATCH_EXPORT_CONTROLLER: once_cell::sync::OnceCell<BasicController> = once_cell::sync::OnceCell::new();

pub trait Metric {
    fn name(&self) -> &str;
}

/// Enum that allows handling of variant metric data types.
pub enum MetricValue {
    F64(Box<dyn Metric>, f64),
    U64(Box<dyn Metric>, u64),
}

/// Configuration for a [BatchExporter].
pub struct BatchExportConfig {
    cx: Context,
    meter_name: &'static str,
    push_interval_seconds: u64,
    endpoint: Option<String>,
    resource_attrs: HashMap<String, String>,
    collector_metadata: Option<HashMap<String, String>>,
    metric_name_validator: fn(&str) -> Result<(), Error>,
    metrics: Vec<Metric>,
}

/// [BatchExporter] provides a high-level abstraction over the complexities of constructing the metrics pipeline
/// specifically for batch tasks. Long-running services are expected to use [tracing] instead.
pub struct BatchExporter {
    config: BatchExportConfig,
    meter: Meter,
    resource_attrs: Vec<KeyValue>,
    collector_metadata: MetadataMap,
    u64_counters: HashMap<String, Counter<u64>>,
    f64_counters: HashMap<String, Counter<f64>>,
}

impl BatchExporter {
    /// Constructs a new metrics controller for the meter.
    pub fn new(config: BatchExportConfig) -> Result<BatchExporter, Error> {
        let meter = global::meter(config.meter_name);
        let resource_attrs = key_value(&config.resource_attrs);
        let collector_metadata = metadata_map(&config.collector_metadata)?;

        let exporter = BatchExporter {
            config,
            meter,
            resource_attrs,
            collector_metadata,
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
                    .with_metadata(self.collector_metadata.clone()),
            )
            .with_period(Duration::from_secs(self.config.push_interval_seconds))
            .with_timeout(Duration::from_secs(10))
            .with_resource(Resource::new(self.resource_attrs.clone()))
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
        let _ = (self.config.metric_name_validator)(name.as_str())?;

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

    pub fn add(&self, value: MetricValue) -> Result<(), Error> {
        match value {
            MetricValue::F64(metric, val) => {
                if !self.f64_counters.contains_key(metric.name()) {
                    return Err(Error::OpenTelemetry(format!("BatchExporter::add_u64 - invalid key {}", metric.name())));
                }
                self.f64_counters.get(metric.name()).unwrap().add(&self.config.cx, val, &self.resource_attrs);
            },
            MetricValue::U64(metric, val) => {
                if !self.u64_counters.contains_key(metric.name()) {
                    return Err(Error::OpenTelemetry(format!("BatchExporter::add_u64 - invalid key {}", metric.name())));
                }
                self.u64_counters.get(metric.name()).unwrap().add(&self.config.cx, val, &self.resource_attrs);
            }
        }
        Ok(())
    }

    /// Initializes an f64 counter that can be incremented.
    pub fn with_f64_counter(&mut self, name: String) -> Result<(), Error> {
        let _ = (self.config.metric_name_validator)(name.as_str())?;

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
                        .map_err(|e| Error::OpenTelemetry(format! {"metrics::metadata_map::key - {}", e}))?;

                    let val = v.clone();
                    let val = MetadataValue::try_from(val.as_bytes())
                        .map_err(|e| Error::OpenTelemetry(format! {"metrics::metadata_map::key - {}", e}))?;

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
    use opentelemetry_api::global::Error::Metric;
    use opentelemetry_sdk::export::metrics::aggregation::{cumulative_temporality_selector, Sum};
    use opentelemetry_sdk::export::metrics::InstrumentationLibraryReader;
    use opentelemetry_sdk::metrics::aggregators::{Aggregator, SumAggregator};
    use opentelemetry_sdk::metrics::sdk_api::NumberKind;

    use crate::Error;
    use crate::opentelemetry::metrics::MetricValue;
    use crate::opentelemetry::metrics::prometheus::validate_metric_name;
    use super::{BatchExportConfig, BatchExporter};

    fn prometheus_test_config(test_name: String) -> BatchExportConfig {
        BatchExportConfig {
            cx: Context::new(),
            meter_name: "unit_tests",
            push_interval_seconds: 1,
            endpoint: None,
            resource_attrs: HashMap::from([("service.name".to_string(), test_name)]),
            collector_metadata: None,
            metric_name_validator: validate_metric_name,
        }
    }

    fn can_get_batch_exporter() -> Result<(), Error> {
        let config = prometheus_test_config("can_get_batch_exporter".to_string());
        let exporter = BatchExporter::new(config)?;

        assert!(exporter.running());

        exporter.stop().expect("could not stop");
        Ok(())
    }

    // TODO: Marking as manual until we can find a way to test OpenTelemetry in memory.
    #[async_std::test]
    #[ignore = "manual run only"]
    async fn can_push_prometheus_metric() -> Result<(), Error> {
        let config = prometheus_test_config("can_push_prometheus_metric".to_string());
        let mut exporter = BatchExporter::new(config)?;

        assert!(exporter.running());

        exporter.with_u64_counter("metrics_test_total".to_string()).expect("unable to add u64_counter");

        for _ in 0..100 {
            let metric = super::super::prometheus::Metric{
                name: "metrics_test_total".to_string(),
                description: "Metrics for unit tests".to_string(),
                label: "unit_tests".to_string(),
                kind: super::super::prometheus::MetricKind::Counter,
            };
            let value = MetricValue::U64(Box::new(metric), 1);
            exporter.add(value)?;
            std::thread::sleep(Duration::from_secs(2));
        }

        // PromQL to verify results
        //rate(
        //   harbor_metrics_test_total{exported_job="unit_tests"}[5m]
        // )


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