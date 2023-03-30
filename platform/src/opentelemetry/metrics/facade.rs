// use opentelemetry::{
//     Collector,
//     CollectorConfig,
//     Exporter,
//     ExporterConfig,
//     ExporterFacade,
// };
// use prometheus::{
//     CollectorRegistry,
//     Counter,
//     Gauge,
//     Histogram,
//     PushCollector,
//     PushCollectorConfig,
// };
//
// fn main() {
//     let collector = Collector::new(CollectorConfig {
//         exporter: Exporter::new(ExporterConfig {
//             type: "prometheus",
//             push_collector: PushCollector::new(
//                 PushCollectorConfig {
//                     registry: CollectorRegistry::new(),
//                 },
//             },
//         },
//     });
//
//     let counter = collector.register(Counter::new("my-counter"));
//     counter.inc();
//     counter.inc();
//
//     let gauge = collector.register(Gauge::new("my-gauge"));
//     gauge.set(1.0);
//     gauge.set(2.0);
//
//     let histogram = collector.register(Histogram::new("my-histogram"));
//     histogram.add_bucket(1.0);
//     histogram.add_bucket(2.0);
//     histogram.add_bucket(3.0);
//
//     collector.start();
//
//     // Wait for some time
//
//     collector.stop();
// }
