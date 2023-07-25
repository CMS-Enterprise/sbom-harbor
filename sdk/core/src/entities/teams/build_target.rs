use crate::Error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A BuildTarget conceptually represents code contained within a [Repository] that can be built
/// into a [Package]. A [Repository] may contain the source code for an arbitrary number of
/// packages. The BuildTarget encapsulates metadata around the [Package] such as what language the
/// package is developed in, the package manager that is used to manage dependencies, and the
/// tooling used to build the package.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct BuildTarget {
    /// The unique identifier for the build target.
    pub id: String,

    /// The package name of the build target.
    pub package_name: String,

    /// The unique identifier for the repository that contains the build target.
    pub repository_id: String,

    /// The package manager used to manage dependencies for the build target.
    pub package_manager: Option<String>,

    /// The primary programming language of the source code.
    pub language: Option<String>,

    /// The build tool used to build the package.
    pub build_tool: Option<String>,

    /// Path within the repository to the manifest file used by the build tool
    /// (e.g. package.json, Cargo.toml, requirements.txt).
    pub manifest_path: Option<String>,
}

impl BuildTarget {
    /// Factory method for new instance of type.
    pub fn new(
        package_name: String,
        repository_id: String,
        package_manager: Option<String>,
        language: Option<String>,
        build_tool: Option<String>,
        manifest_path: Option<String>,
    ) -> Result<BuildTarget, Error> {
        if package_name.is_empty() {
            return Err(Error::Entity("package name required".to_string()));
        }

        if package_name.len() < 2 {
            return Err(Error::Entity(
                "package name must be at least 2 characters in length".to_string(),
            ));
        }

        if repository_id.is_empty() {
            return Err(Error::Entity("team id required".to_string()));
        }

        if repository_id.len() < 2 {
            return Err(Error::Entity(
                "team id must be at least 2 characters in length".to_string(),
            ));
        }

        Ok(BuildTarget {
            id: "".to_string(),
            package_name,
            repository_id,
            package_manager,
            language,
            build_tool,
            manifest_path,
        })
    }
}
