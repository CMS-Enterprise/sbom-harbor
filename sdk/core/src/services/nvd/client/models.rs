use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::NaiveDateTime;
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct NvdVulnerabilityV2 {
    #[serde(rename = "resultsPerPage")]
    results_per_page: i32,
    #[serde(rename = "startIndex")]
    start_index: i32,
    #[serde(rename = "totalResults")]
    total_results: i32,
    #[serde(rename = "format")]
    format_str: Option<String>,
    version: Option<String>,
    timestamp: Option<NaiveDateTime>,
    vulnerabilities: Option<Vec<DefCveItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefCveItem {
    cve: Option<CveItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CveItem {
    id: Option<String>,
    #[serde(rename = "sourceIdentifier")]
    source_identifier: Option<String>,
    #[serde(rename = "vulnStatus")]
    vuln_status: Option<String>,
    published: Option<NaiveDateTime>,
    #[serde(rename = "lastModified")]
    last_modified: Option<NaiveDateTime>,
    #[serde(rename = "evaluatorComment")]
    evaluator_comment: Option<String>,
    #[serde(rename = "evaluatorSolution")]
    evaluator_solution: Option<String>,
    #[serde(rename = "evaluatorImpact")]
    evaluator_impact: Option<String>,
    #[serde(rename = "cisaExploitAdd")]
    cisa_exploit_add: Option<NaiveDateTime>,
    #[serde(rename = "cisaActionDue")]
    cisa_action_due: Option<NaiveDateTime>,
    #[serde(rename = "cisaRequiredAction")]
    cisa_required_action: Option<String>,
    #[serde(rename = "cisaVulnerabilityName")]
    cisa_vulnerability_name: Option<String>,
    descriptions: Option<Vec<LangString>>,
    references: Option<Vec<Reference>>,
    metrics: Option<Metrics>,
    weaknesses: Option<Vec<Weakness>>,
    configurations: Option<Vec<Config>>,
    #[serde(rename = "vendorComments")]
    vendor_comments: Option<Vec<VendorComment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LangString {
    lang: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    url: Option<String>,
    source: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VendorComment {
    organization: Option<String>,
    comment: Option<String>,
    #[serde(rename = "lastModified")]
    last_modified: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weakness {
    source: Option<String>,
    #[serde(rename = "type")]
    type_str: Option<String>,
    description: Option<Vec<LangString>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    operator: Option<String>,
    negate: Option<Option<bool>>,
    nodes: Option<Vec<Node>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    operator: Option<String>,
    negate: Option<bool>,
    #[serde(rename = "cpeMatch")]
    cpe_match: Option<Vec<CpeMatch>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpeMatch {
    vulnerable: Option<bool>,
    criteria: Option<String>,
    #[serde(rename = "matchCriteriaId")]
    match_criteria_id: Option<Uuid>,
    #[serde(rename = "versionStartExcluding")]
    version_start_excluding: Option<String>,
    #[serde(rename = "versionStartIncluding")]
    version_start_including: Option<String>,
    #[serde(rename = "versionEndExcluding")]
    version_end_excluding: Option<String>,
    #[serde(rename = "versionEndIncluding")]
    version_end_including: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    #[serde(rename = "cvssMetricV31")]
    cvss_metric_v31: Option<Vec<CvssV31>>,
    #[serde(rename = "cvssMetricV30")]
    cvss_metric_v30: Option<Vec<CvssV30>>,
    #[serde(rename = "cvssMetricV2")]
    cvss_metric_v2: Option<Vec<CvssV20>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cvss {
    source: Option<String>,
    #[serde(rename = "type")]
    type_str: Option<String>,
    #[serde(rename = "cvssData")]
    cvss_data: Option<HashMap<String, Value>>,
    #[serde(rename = "baseSeverity")]
    base_severity: Option<String>,
    #[serde(rename = "exploitabilityScore")]
    exploitability_score: Option<f64>,
    #[serde(rename = "impactScore")]
    impact_score: Option<f64>,
    #[serde(rename = "acInsufInfo")]
    ac_insuf_info: Option<bool>,
    #[serde(rename = "obtainAllPrivilege")]
    obtain_all_privilege: Option<bool>,
    #[serde(rename = "obtainUserPrivilege")]
    obtain_user_privilege: Option<bool>,
    #[serde(rename = "obtainOtherPrivilege")]
    obtain_other_privilege: Option<bool>,
    #[serde(rename = "userInteractionRequired")]
    user_interaction_required: Option<bool>,
}

/* CVSS 3.1 Schema Structs */

#[derive(Debug, Serialize, Deserialize)]
pub struct CvssV31 {
    license: Option<Vec<String>>,
    #[serde(rename = "$schema")]
    schema: Option<String>,
    title: Option<String>,
    #[serde(rename = "type")]
    type_: Option<String>,
    definitions: Option<HashMap<String, DefinitionTypeV31>>,
    properties: Option<PropertiesV31>,
    required: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefinitionTypeV31 {
    #[serde(rename = "type")]
    type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertiesV31 {
    version: Option<PropertyV31>,
    vector_string: Option<PropertyV31>,
    #[serde(rename = "attackVector")]
    attack_vector: Option<PropertyReferenceV31>,
    #[serde(rename = "attackComplexity")]
    attack_complexity: Option<PropertyReferenceV31>,
    #[serde(rename = "privilegesRequired")]
    privileges_required: Option<PropertyReferenceV31>,
    #[serde(rename = "userInteraction")]
    user_interaction: Option<PropertyReferenceV31>,
    scope: Option<PropertyReferenceV31>,
    #[serde(rename = "confidentialityImpact")]
    confidentiality_impact: Option<PropertyReferenceV31>,
    #[serde(rename = "integrityImpact")]
    integrity_impact: Option<PropertyReferenceV31>,
    #[serde(rename = "availabilityImpact")]
    availability_impact: Option<PropertyReferenceV31>,
    base_score: Option<PropertyReferenceV31>,
    base_severity: Option<PropertyReferenceV31>,
    #[serde(rename = "exploitCodeMaturity")]
    exploit_code_maturity: Option<PropertyReferenceV31>,
    #[serde(rename = "remediationLevel")]
    remediation_level: Option<PropertyReferenceV31>,
    #[serde(rename = "reportConfidence")]
    report_confidence: Option<PropertyReferenceV31>,
    temporal_score: Option<PropertyReferenceV31>,
    temporal_severity: Option<PropertyReferenceV31>,
    #[serde(rename = "confidentialityRequirement")]
    confidentiality_requirement: Option<PropertyReferenceV31>,
    #[serde(rename = "integrityRequirement")]
    integrity_requirement: Option<PropertyReferenceV31>,
    #[serde(rename = "availabilityRequirement")]
    availability_requirement: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedAttackVector")]
    modified_attack_vector: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedAttackComplexity")]
    modified_attack_complexity: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedPrivilegesRequired")]
    modified_privileges_required: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedUserInteraction")]
    modified_user_interaction: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedScope")]
    modified_scope: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedConfidentialityImpact")]
    modified_confidentiality_impact: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedIntegrityImpact")]
    modified_integrity_impact: Option<PropertyReferenceV31>,
    #[serde(rename = "modifiedAvailabilityImpact")]
    modified_availability_impact: Option<PropertyReferenceV31>,
    environmental_score: Option<PropertyReferenceV31>,
    environmental_severity: Option<PropertyReferenceV31>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyV31 {
    description: Option<String>,
    #[serde(rename = "type")]
    type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyReferenceV31 {
    #[serde(rename = "$ref")]
    ref_: Option<String>,
}

/* CVSS 3.0 Schema Structs */

#[derive(Debug, Serialize, Deserialize)]
pub struct CvssV30 {
    license: Option<Vec<String>>,
    #[serde(rename = "$schema")]
    schema: Option<String>,
    title: Option<String>,
    #[serde(rename = "type")]
    type_: Option<String>,
    definitions: Option<HashMap<String, DefinitionTypeV30>>,
    properties: Option<PropertiesV30>,
    required: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefinitionTypeV30 {
    #[serde(rename = "type")]
    type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertiesV30 {
    version: Option<PropertyV30>,
    vector_string: Option<PropertyV30>,
    #[serde(rename = "attackVector")]
    attack_vector: Option<PropertyReferenceV30>,
    #[serde(rename = "attackComplexity")]
    attack_complexity: Option<PropertyReferenceV30>,
    #[serde(rename = "privilegesRequired")]
    privileges_required: Option<PropertyReferenceV30>,
    #[serde(rename = "userInteraction")]
    user_interaction: Option<PropertyReferenceV30>,
    scope: Option<PropertyReferenceV30>,
    #[serde(rename = "confidentialityImpact")]
    confidentiality_impact: Option<PropertyReferenceV30>,
    #[serde(rename = "integrityImpact")]
    integrity_impact: Option<PropertyReferenceV30>,
    #[serde(rename = "availabilityImpact")]
    availability_impact: Option<PropertyReferenceV30>,
    base_score: Option<PropertyReferenceV30>,
    base_severity: Option<PropertyReferenceV30>,
    #[serde(rename = "exploitCodeMaturity")]
    exploit_code_maturity: Option<PropertyReferenceV30>,
    #[serde(rename = "remediationLevel")]
    remediation_level: Option<PropertyReferenceV30>,
    #[serde(rename = "reportConfidence")]
    report_confidence: Option<PropertyReferenceV30>,
    temporal_score: Option<PropertyReferenceV30>,
    temporal_severity: Option<PropertyReferenceV30>,
    #[serde(rename = "confidentialityRequirement")]
    confidentiality_requirement: Option<PropertyReferenceV30>,
    #[serde(rename = "integrityRequirement")]
    integrity_requirement: Option<PropertyReferenceV30>,
    #[serde(rename = "availabilityRequirement")]
    availability_requirement: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedAttackVector")]
    modified_attack_vector: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedAttackComplexity")]
    modified_attack_complexity: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedPrivilegesRequired")]
    modified_privileges_required: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedUserInteraction")]
    modified_user_interaction: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedScope")]
    modified_scope: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedConfidentialityImpact")]
    modified_confidentiality_impact: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedIntegrityImpact")]
    modified_integrity_impact: Option<PropertyReferenceV30>,
    #[serde(rename = "modifiedAvailabilityImpact")]
    modified_availability_impact: Option<PropertyReferenceV30>,
    environmental_score: Option<PropertyReferenceV30>,
    environmental_severity: Option<PropertyReferenceV30>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyV30 {
    description: Option<String>,
    #[serde(rename = "type")]
    type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyReferenceV30 {
    #[serde(rename = "$ref")]
    ref_: Option<String>,
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
