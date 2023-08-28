use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct NvdVulnerabilityV2 {
    #[serde(rename = "resultsPerPage")]
    results_per_page: i32,
    #[serde(rename = "startIndex")]
    start_index: i32,
    #[serde(rename = "totalResults")]
    total_results: i32,
    #[serde(rename = "format")]
    format_str: String,
    version: String,
    timestamp: NaiveDateTime,
    vulnerabilities: Vec<DefCveItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefCveItem {
    cve: CveItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CveItem {
    id: String,
    #[serde(rename = "sourceIdentifier")]
    source_identifier: Option<String>,
    #[serde(rename = "vulnStatus")]
    vuln_status: Option<String>,
    published: NaiveDateTime,
    #[serde(rename = "lastModified")]
    last_modified: NaiveDateTime,
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
    descriptions: Vec<LangString>,
    references: Vec<Reference>,
    metrics: Option<Metrics>,
    weaknesses: Option<Vec<Weakness>>,
    configurations: Option<Vec<Config>>,
    #[serde(rename = "vendorComments")]
    vendor_comments: Option<Vec<VendorComment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LangString {
    lang: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    url: String,
    source: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VendorComment {
    organization: String,
    comment: String,
    #[serde(rename = "lastModified")]
    last_modified: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weakness {
    source: String,
    #[serde(rename = "type")]
    type_str: String,
    description: Vec<LangString>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    operator: String,
    negate: Option<bool>,
    nodes: Vec<Node>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    operator: String,
    negate: Option<bool>,
    #[serde(rename = "cpeMatch")]
    cpe_match: Vec<CpeMatch>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpeMatch {
    vulnerable: bool,
    criteria: String,
    #[serde(rename = "matchCriteriaId")]
    match_criteria_id: uuid::Uuid,
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
    source: String,
    #[serde(rename = "type")]
    type_str: String,
    #[serde(rename = "cvssData")]
    cvss_data: HashMap<String, serde_json::Value>,
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
    license: Vec<String>,
    #[serde(rename = "$schema")]
    schema: String,
    title: String,
    #[serde(rename = "type")]
    type_: String,
    definitions: HashMap<String, DefinitionTypeV31>,
    properties: PropertiesV31,
    required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefinitionTypeV31 {
    #[serde(rename = "type")]
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertiesV31 {
    version: PropertyV31,
    vector_string: PropertyV31,
    #[serde(rename = "attackVector")]
    attack_vector: PropertyReferenceV31,
    #[serde(rename = "attackComplexity")]
    attack_complexity: PropertyReferenceV31,
    #[serde(rename = "privilegesRequired")]
    privileges_required: PropertyReferenceV31,
    #[serde(rename = "userInteraction")]
    user_interaction: PropertyReferenceV31,
    scope: PropertyReferenceV31,
    #[serde(rename = "confidentialityImpact")]
    confidentiality_impact: PropertyReferenceV31,
    #[serde(rename = "integrityImpact")]
    integrity_impact: PropertyReferenceV31,
    #[serde(rename = "availabilityImpact")]
    availability_impact: PropertyReferenceV31,
    base_score: PropertyReferenceV31,
    base_severity: PropertyReferenceV31,
    #[serde(rename = "exploitCodeMaturity")]
    exploit_code_maturity: PropertyReferenceV31,
    #[serde(rename = "remediationLevel")]
    remediation_level: PropertyReferenceV31,
    #[serde(rename = "reportConfidence")]
    report_confidence: PropertyReferenceV31,
    temporal_score: PropertyReferenceV31,
    temporal_severity: PropertyReferenceV31,
    #[serde(rename = "confidentialityRequirement")]
    confidentiality_requirement: PropertyReferenceV31,
    #[serde(rename = "integrityRequirement")]
    integrity_requirement: PropertyReferenceV31,
    #[serde(rename = "availabilityRequirement")]
    availability_requirement: PropertyReferenceV31,
    #[serde(rename = "modifiedAttackVector")]
    modified_attack_vector: PropertyReferenceV31,
    #[serde(rename = "modifiedAttackComplexity")]
    modified_attack_complexity: PropertyReferenceV31,
    #[serde(rename = "modifiedPrivilegesRequired")]
    modified_privileges_required: PropertyReferenceV31,
    #[serde(rename = "modifiedUserInteraction")]
    modified_user_interaction: PropertyReferenceV31,
    #[serde(rename = "modifiedScope")]
    modified_scope: PropertyReferenceV31,
    #[serde(rename = "modifiedConfidentialityImpact")]
    modified_confidentiality_impact: PropertyReferenceV31,
    #[serde(rename = "modifiedIntegrityImpact")]
    modified_integrity_impact: PropertyReferenceV31,
    #[serde(rename = "modifiedAvailabilityImpact")]
    modified_availability_impact: PropertyReferenceV31,
    environmental_score: PropertyReferenceV31,
    environmental_severity: PropertyReferenceV31,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyV31 {
    description: Option<String>,
    #[serde(rename = "type")]
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyReferenceV31 {
    #[serde(rename = "$ref")]
    ref_: String,
}

/* CVSS 3.0 Schema Structs */

#[derive(Debug, Serialize, Deserialize)]
pub struct CvssV30 {
    license: Vec<String>,
    #[serde(rename = "$schema")]
    schema: String,
    title: String,
    #[serde(rename = "type")]
    type_: String,
    definitions: HashMap<String, DefinitionTypeV30>,
    properties: PropertiesV30,
    required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefinitionTypeV30 {
    #[serde(rename = "type")]
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertiesV30 {
    version: PropertyV30,
    vector_string: PropertyV30,
    #[serde(rename = "attackVector")]
    attack_vector: PropertyReferenceV30,
    #[serde(rename = "attackComplexity")]
    attack_complexity: PropertyReferenceV30,
    #[serde(rename = "privilegesRequired")]
    privileges_required: PropertyReferenceV30,
    #[serde(rename = "userInteraction")]
    user_interaction: PropertyReferenceV30,
    scope: PropertyReferenceV30,
    #[serde(rename = "confidentialityImpact")]
    confidentiality_impact: PropertyReferenceV30,
    #[serde(rename = "integrityImpact")]
    integrity_impact: PropertyReferenceV30,
    #[serde(rename = "availabilityImpact")]
    availability_impact: PropertyReferenceV30,
    base_score: PropertyReferenceV30,
    base_severity: PropertyReferenceV30,
    #[serde(rename = "exploitCodeMaturity")]
    exploit_code_maturity: PropertyReferenceV30,
    #[serde(rename = "remediationLevel")]
    remediation_level: PropertyReferenceV30,
    #[serde(rename = "reportConfidence")]
    report_confidence: PropertyReferenceV30,
    temporal_score: PropertyReferenceV30,
    temporal_severity: PropertyReferenceV30,
    #[serde(rename = "confidentialityRequirement")]
    confidentiality_requirement: PropertyReferenceV30,
    #[serde(rename = "integrityRequirement")]
    integrity_requirement: PropertyReferenceV30,
    #[serde(rename = "availabilityRequirement")]
    availability_requirement: PropertyReferenceV30,
    #[serde(rename = "modifiedAttackVector")]
    modified_attack_vector: PropertyReferenceV30,
    #[serde(rename = "modifiedAttackComplexity")]
    modified_attack_complexity: PropertyReferenceV30,
    #[serde(rename = "modifiedPrivilegesRequired")]
    modified_privileges_required: PropertyReferenceV30,
    #[serde(rename = "modifiedUserInteraction")]
    modified_user_interaction: PropertyReferenceV30,
    #[serde(rename = "modifiedScope")]
    modified_scope: PropertyReferenceV30,
    #[serde(rename = "modifiedConfidentialityImpact")]
    modified_confidentiality_impact: PropertyReferenceV30,
    #[serde(rename = "modifiedIntegrityImpact")]
    modified_integrity_impact: PropertyReferenceV30,
    #[serde(rename = "modifiedAvailabilityImpact")]
    modified_availability_impact: PropertyReferenceV30,
    environmental_score: PropertyReferenceV30,
    environmental_severity: PropertyReferenceV30,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyV30 {
    description: Option<String>,
    #[serde(rename = "type")]
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyReferenceV30 {
    #[serde(rename = "$ref")]
    ref_: String,
}

/* CVSS 2.0 Schema Structs */

#[derive(Debug, Serialize, Deserialize)]
pub struct CvssV20 {
    pub version: String,
    #[serde(rename = "vectorString")]
    pub vector_string: String,
    #[serde(rename = "accessVector")]
    pub access_vector: AccessVectorTypeV20,
    #[serde(rename = "accessComplexity")]
    pub access_complexity: AccessComplexityTypeV20,
    #[serde(rename = "authentication")]
    pub authentication: AuthenticationTypeV20,
    #[serde(rename = "confidentialityImpact")]
    pub confidentiality_impact: CiaTypeV20,
    #[serde(rename = "integrityImpact")]
    pub integrity_impact: CiaTypeV20,
    #[serde(rename = "availabilityImpact")]
    pub availability_impact: CiaTypeV20,
    #[serde(rename = "baseScore")]
    pub base_score: ScoreTypeV20,
    #[serde(rename = "exploitability")]
    pub exploitability: ExploitabilityTypeV20,
    #[serde(rename = "remediationLevel")]
    pub remediation_level: RemediationLevelTypeV20,
    #[serde(rename = "reportConfidence")]
    pub report_confidence: ReportConfidenceTypeV20,
    #[serde(rename = "temporalScore")]
    pub temporal_score: ScoreTypeV20,
    #[serde(rename = "collateralDamagePotential")]
    pub collateral_damage_potential: CollateralDamagePotentialTypeV20,
    #[serde(rename = "targetDistribution")]
    pub target_distribution: TargetDistributionTypeV20,
    #[serde(rename = "confidentialityRequirement")]
    pub confidentiality_requirement: CiaRequirementTypeV20,
    #[serde(rename = "integrityRequirement")]
    pub integrity_requirement: CiaRequirementTypeV20,
    #[serde(rename = "availabilityRequirement")]
    pub availability_requirement: CiaRequirementTypeV20,
    #[serde(rename = "environmentalScore")]
    pub environmental_score: ScoreTypeV20,
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
    pub type_field: String,
    #[serde(rename = "minimum")]
    pub minimum: f64,
    #[serde(rename = "maximum")]
    pub maximum: f64,
}
