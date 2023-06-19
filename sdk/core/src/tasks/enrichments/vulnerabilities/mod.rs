/// Contains the [TaskProvider] implementation that retrieves EPSS scores from the public API and
/// maps them to [Vulnerability] entities.
pub mod epss;

/// Contains the [TaskProvider] implementation that retrieves and correlates vulnerability data
/// from the Snyk API.
pub mod snyk;
