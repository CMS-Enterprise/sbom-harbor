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

/// The [enrichments] module contains domain and persistence logic related to enriching an [Sbom]
/// with additional metadata.
pub mod enrichments;

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

/// The [tasks] module contains domain logic and traits related to processing tasks.
pub mod tasks;
