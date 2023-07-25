use harbcore::entities::teams::BuildTarget;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::Error;

/// Validatable insert type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct BuildTargetInsert {
    /// The package name of the build target.
    pub package_name: Option<String>,

    /// The unique identifier for the repository that contains the build target.
    pub repository_id: Option<String>,

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

impl BuildTargetInsert {
    /// Validates insert type and converts to entity.
    #[allow(dead_code)]
    pub fn to_entity(&self) -> Result<BuildTarget, Error> {
        let package_name = match &self.package_name {
            None => {
                return Err(Error::InvalidParameters(
                    "package name required".to_string(),
                ));
            }
            Some(package_name) => package_name.clone(),
        };

        let repository_id = match &self.repository_id {
            None => {
                return Err(Error::InvalidParameters(
                    "repository id required".to_string(),
                ));
            }
            Some(repository_id) => repository_id.clone(),
        };

        BuildTarget::new(
            package_name,
            repository_id,
            self.package_manager.clone(),
            self.language.clone(),
            self.build_tool.clone(),
            self.manifest_path.clone(),
        )
        .map_err(|e| Error::InvalidParameters(e.to_string()))
    }
}
