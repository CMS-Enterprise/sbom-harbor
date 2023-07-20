/// The [packages] module contains domain and persistence logic related to the management of
/// a [Package].
///
/// - [Package] - An artifact for which an [Sbom] can be generated.
/// - [Dependency]- An artifact that a [Package] depends on.
pub mod packages;

/// The [sboms] module contains domain and persistence logic related to the management of an
/// [Sbom]. An [Sbom] is a document that lists all components and dependencies required to
/// produce a [Package].
pub mod sboms;

/// The [vulnerabilities] module contains domain and persistence logic related to the tracking of
/// a [Vulnerability] related to a [Package].
pub mod vulnerabilities;

/// The Snyk module contains integration logic related to the management of [Packages],
/// and [SBOMs] when an organization is using the Snyk application.
pub mod snyk;

/// The Syft module contains logic related to the analysis and parsing of [SBOMs] with Syft.
pub mod syft;

/// The [teams] module contains domain and persistence logic related to the management of
/// [Team] entities.
pub mod teams;

/// The [xrefs] module contains domain and persistence logic related to the management of
/// cross-references for entities that implement the [Xrefs] trait.
pub mod xrefs;

/// The [analytics] module contains reporting services to make the data useful.
pub mod analytics;

/// The GitHub module contains integration logic related to the management of [Packages],
/// and [SBOMs] leveraging GitHub organizations.
pub mod github;

/// The [vendors] module contains domain and persistence logic related to the management of
/// [Vendor] entities.
pub mod vendors;

/// The [products] module contains domain and persistence logic related to the management of
/// [Product] entities.
pub mod products;
