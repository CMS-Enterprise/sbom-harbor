//! This module provides abstractions to simplify emitting metrics to an OpenTelemetry Collector.
//! The first target use case is using OpenTelemetry Collector with a `Prometheus` backend, which
//! is how the Harbor team operates.
//!
//! This module is rapidly evolving, as is the underlying OpenTelemetry Metrics SDK. The interfaces
//! in this module are not yet stable, and are subject to change based on both changes in the
//! encapsulated SDK, and as new backend use cases are identified and supported.

mod batch;
pub use batch::*;

/// The `Prometheus` module encapsulates metrics conventions and functionality
/// specific to using `Prometheus` as the backend for an OpenTelemetry collector.
pub mod prometheus;
