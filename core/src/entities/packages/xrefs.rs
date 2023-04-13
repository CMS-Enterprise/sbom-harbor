use serde::{Deserialize, Serialize};

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FismaXRef {
    pub fisma_id: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodebaseXRef {
    pub team_id: String,
    pub project_id: String,
    pub codebase_id: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductXRef {
    pub vendor_id: String,
    pub product_id: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnykXRef {
    pub id: String,
    pub active: bool,
    pub org_id: String,
    pub org_name: String,
    pub group_id: String,
    pub group_name: String,
    pub project_id: String,
    pub project_name: String,
}

impl SnykXRef {
    pub fn eq(&self, xref: &SnykXRef) -> bool {
        self.org_id == xref.org_id
            && self.group_id == xref.group_id
            && self.project_id == xref.project_id
    }
}
