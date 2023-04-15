/// The [packages] module contains business and persistence logic related to the management of
/// [Packages] and [Dependencies].
///
/// - [Package] - An artifact for which an SBOM can be generated.
/// - [Dependency]- An artifact that a [Package] depends on.
pub mod packages;

/// The [packages] module contains business and persistence logic related to the management of
/// [SBOMs]. A [SBOM] is a document that lists all components and dependencies required to
/// produce a [Package].
pub mod sboms;

/// The [findings] module contains business and persistence logic related to the management of
/// [Findings]. A [Finding] in Harbor is defined as a security fact that pertains to a [Package]
/// or a [Dependency].
pub mod findings;

/// The Snyk module contains integration logic related to the management of [Packages],
/// [Dependencies], and [SBOMs] when an organization is leveraging the Snyk application.
pub mod snyk;

/// The [teams] module contains business and persistence logic related to the management of
/// [Teams]. A [Team] is used to group [Members] and [Projects].
///
/// - [Members] are [Users] that belong to a [Team].
/// - [Projects] are a way to group a related set of [Codebases].
/// - [Codebases] represent one or more source code targets that are used to produce a [Package].
pub mod teams;
