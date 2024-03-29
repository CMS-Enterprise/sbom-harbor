use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataClassification {
    #[serde(rename = "flow")]
    pub flow: crate::entities::cyclonedx::models::DataFlow,
    /// Data classification tags data according to its type, sensitivity, and value if altered, stolen, or destroyed.
    #[serde(rename = "classification")]
    pub classification: String,
}

impl DataClassification {
    pub fn new(
        flow: crate::entities::cyclonedx::models::DataFlow,
        classification: String,
    ) -> DataClassification {
        DataClassification {
            flow,
            classification,
        }
    }
}
