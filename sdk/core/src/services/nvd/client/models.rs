use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::NaiveDateTime;
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct NvdVulnerabilityV2 {
    #[serde(rename = "resultsPerPage")]
    pub results_per_page: i32,
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    #[serde(rename = "totalResults")]
    pub total_results: i32,
    #[serde(rename = "format")]
    pub format_str: Option<String>,
    pub version: Option<String>,
    pub timestamp: Option<NaiveDateTime>,
    pub vulnerabilities: Option<Vec<DefCveItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefCveItem {
    pub cve: Option<CveItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CveItem {
    pub id: Option<String>,
    #[serde(rename = "sourceIdentifier")]
    pub source_identifier: Option<String>,
    #[serde(rename = "vulnStatus")]
    pub vuln_status: Option<String>,
    pub published: Option<NaiveDateTime>,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<NaiveDateTime>,
    #[serde(rename = "evaluatorComment")]
    pub evaluator_comment: Option<String>,
    #[serde(rename = "evaluatorSolution")]
    pub evaluator_solution: Option<String>,
    #[serde(rename = "evaluatorImpact")]
    pub evaluator_impact: Option<String>,
    #[serde(rename = "cisaExploitAdd")]
    pub cisa_exploit_add: Option<NaiveDateTime>,
    #[serde(rename = "cisaActionDue")]
    pub cisa_action_due: Option<NaiveDateTime>,
    #[serde(rename = "cisaRequiredAction")]
    pub cisa_required_action: Option<String>,
    #[serde(rename = "cisaVulnerabilityName")]
    pub cisa_vulnerability_name: Option<String>,
    pub descriptions: Option<Vec<LangString>>,
    pub references: Option<Vec<Reference>>,
    pub metrics: Option<Metrics>,
    pub weaknesses: Option<Vec<Weakness>>,
    pub configurations: Option<Vec<Config>>,
    #[serde(rename = "vendorComments")]
    pub vendor_comments: Option<Vec<VendorComment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LangString {
    pub lang: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub url: Option<String>,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VendorComment {
    pub organization: Option<String>,
    pub comment: Option<String>,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weakness {
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_str: Option<String>,
    pub description: Option<Vec<LangString>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub operator: Option<String>,
    pub negate: Option<Option<bool>>,
    pub nodes: Option<Vec<Node>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub operator: Option<String>,
    pub negate: Option<bool>,
    #[serde(rename = "cpeMatch")]
    pub cpe_match: Option<Vec<CpeMatch>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpeMatch {
    pub vulnerable: Option<bool>,
    pub criteria: Option<String>,
    #[serde(rename = "matchCriteriaId")]
    pub match_criteria_id: Option<Uuid>,
    #[serde(rename = "versionStartExcluding")]
    pub version_start_excluding: Option<String>,
    #[serde(rename = "versionStartIncluding")]
    pub version_start_including: Option<String>,
    #[serde(rename = "versionEndExcluding")]
    pub version_end_excluding: Option<String>,
    #[serde(rename = "versionEndIncluding")]
    pub version_end_including: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    #[serde(rename = "cvssMetricV31")]
    pub cvss_metric_v31: Option<Vec<CvssV31>>,
    #[serde(rename = "cvssMetricV30")]
    pub cvss_metric_v30: Option<Vec<CvssV30>>,
    #[serde(rename = "cvssMetricV2")]
    pub cvss_metric_v2: Option<Vec<CvssV20>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cvss {
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_str: Option<String>,
    #[serde(rename = "cvssData")]
    pub cvss_data: Option<HashMap<String, Value>>,
    #[serde(rename = "baseSeverity")]
    pub base_severity: Option<String>,
    #[serde(rename = "exploitabilityScore")]
    pub exploitability_score: Option<f64>,
    #[serde(rename = "impactScore")]
    pub impact_score: Option<f64>,
    #[serde(rename = "acInsufInfo")]
    pub ac_insuf_info: Option<bool>,
    #[serde(rename = "obtainAllPrivilege")]
    pub obtain_all_privilege: Option<bool>,
    #[serde(rename = "obtainUserPrivilege")]
    pub obtain_user_privilege: Option<bool>,
    #[serde(rename = "obtainOtherPrivilege")]
    pub obtain_other_privilege: Option<bool>,
    #[serde(rename = "userInteractionRequired")]
    pub user_interaction_required: Option<bool>,
}

/* CVSS 3.1 Schema Structs */

#[derive(Debug, Serialize, Deserialize)]
pub struct CvssV31 {
    pub license: Option<Vec<String>>,
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub definitions: Option<HashMap<String, DefinitionTypeV31>>,
    pub properties: Option<PropertiesV31>,
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefinitionTypeV31 {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertiesV31 {
    pub version: Option<PropertyV31>,
    pub vector_string: Option<PropertyV31>,
    #[serde(rename = "attackVector")]
    pub attack_vector: Option<PropertyReferenceV31>,
    #[serde(rename = "attackComplexity")]
    pub attack_complexity: Option<PropertyReferenceV31>,
    #[serde(rename = "privilegesRequired")]
    privileges_required: Option<PropertyReferenceV31>,
    #[serde(rename = "userInteraction")]
    pub user_interaction: Option<PropertyReferenceV31>,
    pub scope: Option<PropertyReferenceV31>,
    #[serde(rename = "confidentialityImpact")]
    pub confidentiality_impact: Option<PropertyReferenceV31>,
    #[serde(rename = "integrityImpact")]
    pub integrity_impact: Option<PropertyReferenceV31>,
    #[serde(rename = "availabilityImpact")]
    pub availability_impact: Option<PropertyReferenceV31>,
    pub base_score: Option<PropertyReferenceV31>,
    pub base_severity: Option<PropertyReferenceV31>,
    #[serde(rename = "exploitCodeMaturity")]
    pub exploit_code_maturity: Option<PropertyReferenceV31>,
    #[serde(rename = "remediationLevel")]
    pub remediation_level: Option<PropertyReferenceV31>,
    #[serde(rename = "reportConfidence")]
    pub report_confidence: Option<PropertyReferenceV31>,
    pub temporal_score: Option<PropertyReferenceV31>,
    pub temporal_severity: Option<PropertyReferenceV31>,
    #[serde(rename = "confidentialityRequirement")]
    pub confidentiality_requirement: Option<PropertyReferenceV31>,
    #[serde(rename = "integrityRequirement")]
    pub integrity_requirement: Option<PropertyReferenceV31>,
    #[serde(rename = "availabilityRequirement")]
    pub availability_requirement: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedAttackVector")]
    pub modified_attack_vector: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedAttackComplexity")]
    pub modified_attack_complexity: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedPrivilegesRequired")]
    pub modified_privileges_required: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedUserInteraction")]
    pub modified_user_interaction: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedScope")]
    pub modified_scope: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedConfidentialityImpact")]
    pub modified_confidentiality_impact: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedIntegrityImpact")]
    pub modified_integrity_impact: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedAvailabilityImpact")]
    pub modified_availability_impact: Option<PropertyReferenceV31>,
    pub environmental_score: Option<PropertyReferenceV31>,
    pub environmental_severity: Option<PropertyReferenceV31>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyV31 {
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyReferenceV31 {
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
}

/* CVSS 3.0 Schema Structs */

#[derive(Debug, Serialize, Deserialize)]
pub struct CvssV30 {
    pub license: Option<Vec<String>>,
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub definitions: Option<HashMap<String, DefinitionTypeV30>>,
    pub properties: Option<PropertiesV30>,
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefinitionTypeV30 {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertiesV30 {
    pub version: Option<PropertyV30>,
    pub vector_string: Option<PropertyV30>,
    #[serde(rename = "attackVector")]
    pub attack_vector: Option<PropertyReferenceV30>,
    #[serde(rename = "attackComplexity")]
    pub attack_complexity: Option<PropertyReferenceV30>,
    #[serde(rename = "privilegesRequired")]
    pub privileges_required: Option<PropertyReferenceV30>,
    #[serde(rename = "userInteraction")]
    pub user_interaction: Option<PropertyReferenceV30>,
    pub scope: Option<PropertyReferenceV30>,
    #[serde(rename = "confidentialityImpact")]
    pub confidentiality_impact: Option<PropertyReferenceV30>,
    #[serde(rename = "integrityImpact")]
    pub integrity_impact: Option<PropertyReferenceV30>,
    #[serde(rename = "availabilityImpact")]
    pub availability_impact: Option<PropertyReferenceV30>,
    pub base_score: Option<PropertyReferenceV30>,
    pub base_severity: Option<PropertyReferenceV30>,
    #[serde(rename = "exploitCodeMaturity")]
    pub exploit_code_maturity: Option<PropertyReferenceV30>,
    #[serde(rename = "remediationLevel")]
    pub remediation_level: Option<PropertyReferenceV30>,
    #[serde(rename = "reportConfidence")]
    pub report_confidence: Option<PropertyReferenceV30>,
    pub temporal_score: Option<PropertyReferenceV30>,
    pub temporal_severity: Option<PropertyReferenceV30>,
    #[serde(rename = "confidentialityRequirement")]
    pub confidentiality_requirement: Option<PropertyReferenceV30>,
    #[serde(rename = "integrityRequirement")]
    pub integrity_requirement: Option<PropertyReferenceV30>,
    #[serde(rename = "availabilityRequirement")]
    pub availability_requirement: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedAttackVector")]
    pub modified_attack_vector: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedAttackComplexity")]
    pub modified_attack_complexity: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedPrivilegesRequired")]
    pub modified_privileges_required: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedUserInteraction")]
    pub modified_user_interaction: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedScope")]
    pub modified_scope: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedConfidentialityImpact")]
    pub modified_confidentiality_impact: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedIntegrityImpact")]
    pub modified_integrity_impact: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedAvailabilityImpact")]
    pub modified_availability_impact: Option<PropertyReferenceV30>,
    pub environmental_score: Option<PropertyReferenceV30>,
    pub environmental_severity: Option<PropertyReferenceV30>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyV30 {
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyReferenceV30 {
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
}

/* CVSS 2.0 Schema Structs */

#[derive(Debug, Serialize, Deserialize)]
pub struct CvssV20 {
    pub version: Option<String>,
    #[serde(rename = "vectorString")]
    pub vector_string: Option<String>,
    #[serde(rename = "accessVector")]
    pub access_vector: Option<AccessVectorTypeV20>,
    #[serde(rename = "accessComplexity")]
    pub access_complexity: Option<AccessComplexityTypeV20>,
    #[serde(rename = "authentication")]
    pub authentication: Option<AuthenticationTypeV20>,
    #[serde(rename = "confidentialityImpact")]
    pub confidentiality_impact: Option<CiaTypeV20>,
    #[serde(rename = "integrityImpact")]
    pub integrity_impact: Option<CiaTypeV20>,
    #[serde(rename = "availabilityImpact")]
    pub availability_impact: Option<CiaTypeV20>,
    #[serde(rename = "baseScore")]
    pub base_score: Option<ScoreTypeV20>,
    #[serde(rename = "exploitability")]
    pub exploitability: Option<ExploitabilityTypeV20>,
    #[serde(rename = "remediationLevel")]
    pub remediation_level: Option<RemediationLevelTypeV20>,
    #[serde(rename = "reportConfidence")]
    pub report_confidence: Option<ReportConfidenceTypeV20>,
    #[serde(rename = "temporalScore")]
    pub temporal_score: Option<ScoreTypeV20>,
    #[serde(rename = "collateralDamagePotential")]
    pub collateral_damage_potential: Option<CollateralDamagePotentialTypeV20>,
    #[serde(rename = "targetDistribution")]
    pub target_distribution: Option<TargetDistributionTypeV20>,
    #[serde(rename = "confidentialityRequirement")]
    pub confidentiality_requirement: Option<CiaRequirementTypeV20>,
    #[serde(rename = "integrityRequirement")]
    pub integrity_requirement: Option<CiaRequirementTypeV20>,
    #[serde(rename = "availabilityRequirement")]
    pub availability_requirement: Option<CiaRequirementTypeV20>,
    #[serde(rename = "environmentalScore")]
    pub environmental_score: Option<ScoreTypeV20>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccessVectorTypeV20 {
    Network,
    AdjacentNetwork,
    Local,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccessComplexityTypeV20 {
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthenticationTypeV20 {
    Multiple,
    Single,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CiaTypeV20 {
    None,
    Partial,
    Complete,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExploitabilityTypeV20 {
    Unproven,
    ProofOfConcept,
    Functional,
    High,
    NotDefined,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RemediationLevelTypeV20 {
    OfficialFix,
    TemporaryFix,
    Workaround,
    Unavailable,
    NotDefined,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportConfidenceTypeV20 {
    Unconfirmed,
    Uncorroborated,
    Confirmed,
    NotDefined,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CollateralDamagePotentialTypeV20 {
    None,
    Low,
    LowMedium,
    MediumHigh,
    High,
    NotDefined,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TargetDistributionTypeV20 {
    None,
    Low,
    Medium,
    High,
    NotDefined,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CiaRequirementTypeV20 {
    Low,
    Medium,
    High,
    NotDefined,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreTypeV20 {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "minimum")]
    pub minimum: Option<f64>,
    #[serde(rename = "maximum")]
    pub maximum: Option<f64>,
}
