use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Hash {
    #[serde(rename = "alg")]
    pub alg: crate::entities::cyclonedx::models::HashAlg,
    #[serde(rename = "content")]
    pub content: String,
}

impl Hash {
    pub fn new(alg: crate::entities::cyclonedx::models::HashAlg, content: String) -> Hash {
        Hash { alg, content }
    }
}
