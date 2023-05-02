use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

/// Declares the current state of an occurrence of a vulnerability, after automated or manual analysis.   * __resolved__ = the vulnerability has been remediated.  * __resolved\\_with\\_pedigree__ = the vulnerability has been remediated and evidence of the changes are provided in the affected components pedigree containing verifiable commit history and/or diff(s).  * __exploitable__ = the vulnerability may be directly or indirectly exploitable.  * __in\\_triage__ = the vulnerability is being investigated.  * __false\\_positive__ = the vulnerability is not specific to the component or service and was falsely identified or associated.  * __not\\_affected__ = the component or service is not affected by the vulnerability. Justification should be specified for all not_affected cases.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImpactAnalysisState {
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "resolved_with_pedigree")]
    ResolvedWithPedigree,
    #[serde(rename = "exploitable")]
    Exploitable,
    #[serde(rename = "in_triage")]
    InTriage,
    #[serde(rename = "false_positive")]
    FalsePositive,
    #[serde(rename = "not_affected")]
    NotAffected,
}

impl ToString for ImpactAnalysisState {
    fn to_string(&self) -> String {
        match self {
            Self::Resolved => String::from("resolved"),
            Self::ResolvedWithPedigree => String::from("resolved_with_pedigree"),
            Self::Exploitable => String::from("exploitable"),
            Self::InTriage => String::from("in_triage"),
            Self::FalsePositive => String::from("false_positive"),
            Self::NotAffected => String::from("not_affected"),
        }
    }
}

impl Default for ImpactAnalysisState {
    fn default() -> ImpactAnalysisState {
        Self::Resolved
    }
}
