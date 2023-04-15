use serde::{Deserialize, Serialize};

use crate::entities::packages::xrefs::SnykXRef;

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Unsupported {
    pub id: String,
    pub name: String,
    pub package_manager: String,
    snyk_refs: Option<Vec<SnykXRef>>,
}
