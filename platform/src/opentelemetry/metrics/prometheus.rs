use std::sync::Arc;

use lazy_static::lazy_static;
use opentelemetry_sdk::metrics::sdk_api::Descriptor;
use opentelemetry_sdk::metrics::aggregators::Aggregator;

use crate::Error;
use crate::opentelemetry::metrics::MetricValue;

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
pub fn validate_metric_name(name: &str) -> Result<(), Error> {
    for valid in &*PROMETHEUS_SUFFIXES {
        if name.ends_with(*valid) {
            return Ok(());
        }
    }

    Err(Error::OpenTelemetry(format!("invalid metric name {}", name)))
}

/// A metric is a characteristic of a system that is being measured.
/// Examples of metrics in Harbor are:
///
/// - target_total
/// - start_timestamp_seconds
/// - avg_size_bytes
pub struct Metric {
    /// The name of the metric. See examples
    pub name: String,
    /// An expanded definition of what the metric measures.
    pub description: String,
    /// A label is used to distinguish what produced the metric. For example, `start_timestamp_seconds`
    /// is a metric produced by any batch job. The label is used to distinguish which batch job produced
    /// the metric.
    pub label: String,
    /// The kind or type of the metric.
    pub kind: MetricKind,
}

impl super::Metric for Metric{
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

/// Enumerates the four Prometheus metric types. See https://prometheus.io/docs/concepts/metric_types/.
pub enum MetricKind {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

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