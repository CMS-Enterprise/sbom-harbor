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

/// The [teams] module contains domain and persistence logic related to the management of
/// [Teams]. A [Team] is used to group [Members] and [Projects].
///
/// - [Members] are [Users] that belong to a [Team].
/// - [Projects] are a way to group a related set of [Codebases].
/// - [Codebases] represent one or more source code targets that are used to produce a [Package].
pub mod teams;

/// The [xrefs] module contains domain and persistence logic related to the management of
/// cross-references for entities that implement the [Xrefs] trait.
pub mod xrefs;

/// The [analytics] module contains reporting services to make the data useful.
pub mod analytics;

/// The GitHub module contains integration logic related to the management of [Packages],
/// and [SBOMs] leveraging GitHub organizations.
pub mod github;

/// Module for Syft: library for generating a Software Bill of Materials (SBOM)
pub(crate) mod syft;
