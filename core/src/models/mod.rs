
/// Data structures that support storing, analyzing, and enriching [SBOM] files and data over time.
pub mod sboms;

/// Data structures that support managing access to [SBOM] data from the perspective of the
/// a source control system. Also, useful for analysis.
pub mod teams;

/// Data structures generated from the CycloneDx JSON Spec.
pub mod cyclonedx;
