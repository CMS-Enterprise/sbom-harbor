use crate::entities::xrefs::{Xref, XrefKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(missing_docs)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodebaseRef {
    pub team_id: String,
    pub project_id: String,
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
                    ("codebase_id".to_string(), codebase_ref.codebase_id.clone()),
                ])
            },
        }
    }
}

#[allow(missing_docs)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductRef {
    pub vendor_id: String,
    pub product_id: String,
}

impl From<ProductRef> for Xref {
    fn from(product_ref: ProductRef) -> Self {
        Xref {
            kind: XrefKind::Product,
            map: {
                HashMap::from([
                    ("vendor_id".to_string(), product_ref.vendor_id.clone()),
                    ("product_id".to_string(), product_ref.product_id.clone()),
                ])
            },
        }
    }
}
