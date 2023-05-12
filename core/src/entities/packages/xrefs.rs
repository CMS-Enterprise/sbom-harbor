use crate::entities::xrefs::{Xref, XrefKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Set of ids used to cross-reference an entity to a [Codebase].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodebaseRef {
    /// Id of the Team that contains the [Project] that contains the [Codebase].
    pub team_id: String,
    /// Id of the [Project] that contains the [Codebase]
    pub project_id: String,
    /// Id of the [Codebase].
    pub codebase_id: String,
}

impl From<CodebaseRef> for Xref {
    fn from(codebase_ref: CodebaseRef) -> Xref {
        Xref {
            kind: XrefKind::Codebase,
            map: {
                HashMap::from([
                    ("team_id".to_string(), codebase_ref.team_id.clone()),
                    ("project_id".to_string(), codebase_ref.project_id.clone()),
                    ("codebase_id".to_string(), codebase_ref.codebase_id),
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
