use serde::{Deserialize, Serialize};

/// A BuildTarget conceptually represents code contained within a [Repository] that can be built
/// into a [Package]. A [Repository] may contain the source code for an arbitrary number of
/// packages. The BuildTarget encapsulates metadata around the [Package] such as what language the
/// package is developed in, the package manager that is used to manage dependencies, and the
/// tooling used to build the package.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTarget {
    /// The unique identifier for the build target.
    pub id: String,
    /// The package name of the build target.
    pub package_name: String,
    /// The package manager used to manage dependencies for the build target.
    pub package_manager: Option<String>,
    /// The primary programming language of the source code.
    pub language: Option<String>,
    /// The build tool used to build the package.
    pub build_tool: Option<String>,
}
