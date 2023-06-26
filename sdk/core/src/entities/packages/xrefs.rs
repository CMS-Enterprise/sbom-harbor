use crate::entities::xrefs::{Xref, XrefKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Set of ids used to cross-reference an entity to a [BuildTarget].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTargetRef {
    /// Id of the Team that contains the [Repository] that contains the [BuildTarget].
    pub team_id: String,
    /// Id of the [Repository] that contains the [BuildTarget]
    pub repository_id: String,
    /// Id of the [BuildTarget].
    pub build_target_id: String,
}

impl From<BuildTargetRef> for Xref {
    fn from(build_target_ref: BuildTargetRef) -> Xref {
        Xref {
            kind: XrefKind::BuildTarget,
            map: {
                HashMap::from([
                    ("team_id".to_string(), build_target_ref.team_id.clone()),
                    (
                        "project_id".to_string(),
                        build_target_ref.repository_id.clone(),
                    ),
                    ("codebase_id".to_string(), build_target_ref.build_target_id),
                ])
            },
        }
    }
}

/// Set of ids used to cross-reference an entity to a [Product].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductRef {
    /// Id of the [Vendor] that produces the [Product].
    pub vendor_id: String,
    /// Id of the [Product].
    pub product_id: String,
}

impl From<ProductRef> for Xref {
    fn from(product_ref: ProductRef) -> Self {
        Xref {
            kind: XrefKind::Product,
            map: {
                HashMap::from([
                    ("vendor_id".to_string(), product_ref.vendor_id.clone()),
                    ("product_id".to_string(), product_ref.product_id),
                ])
            },
        }
    }
}
