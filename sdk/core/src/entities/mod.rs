/// Models generated by the CycloneDx JSON Specification and extensions that have been added to
/// support working with CycloneDx documents.
pub mod cyclonedx;

/// Persistence models that support storing, analyzing, and tracking software packages over time and
/// across system and organizational boundaries.
pub mod packages;

/// Persistence models that support storing, analyzing, and tracking SBOMs over time.
pub mod sboms;

/// Persistence and domain models that support running processing tasks.
pub mod tasks;

/// Persistence models that support storing and managing groups of people that produce and manage
/// SBOMs.
pub mod teams;

/// Persistence models that support storing, analyzing, and tracking SBOMs for vendors.
pub mod vendors;

/// Persistence models that support storing, analyzing, and tracking SBOMs for products sold by
/// vendors.
pub mod products;

/// Model extensions that allow cross-referencing to external systems.
pub mod xrefs;

/// Persistence and domain models that support enhancing SBOMs with additional metadata.
pub mod enrichments;

/// Models for Analytics
pub mod analytics;

/// Models for data set construction
pub mod datasets;
