#![allow(missing_docs)]
/// Generated and then manipulated from Snyk Rest API v2023-03-08
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrgV1 {
    pub id: Option<String>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub url: Option<String>,
    pub group: Option<Group>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrgsResponse {
    pub(crate) orgs: Option<Vec<OrgV1>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Group {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JsonApi {
    /// Version of the JSON API specification this server supports.
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListOrgProjects200Response {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ListOrgProjects200ResponseDataInner>>,
    #[serde(rename = "jsonapi")]
    pub jsonapi: Box<JsonApi>,
    #[serde(rename = "links")]
    pub links: Box<Links>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<ListOrgProjects200ResponseMeta>>,
}


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListOrgProjects200ResponseDataInner {
    // WARN: Had to change this to an option despite the spec definition.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<ProjectAttributes>>,
    /// Resource ID.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<ListOrgProjects200ResponseDataInnerMeta>>,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Box<ProjectRelationships>>,
    /// The Resource type.
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListOrgProjects200ResponseDataInnerMeta {
    /// The date that the project was last uploaded and monitored using cli.
    #[serde(rename = "cli_monitored_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cli_monitored_at: Option<Option<String>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListOrgProjects200ResponseMeta {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonIssueModel {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<CommonIssueModelAttributes>>,
    /// The Snyk ID of the vulnerability.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of the REST resource. Always ‘issue’.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

// #[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
// pub struct Links {
//     #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
//     pub first: Option<Box<LinkProperty>>,
//     #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
//     pub last: Option<Box<LinkProperty>>,
//     #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
//     pub next: Option<Box<LinkProperty>>,
//     #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
//     pub prev: Option<Box<LinkProperty>>,
//     #[serde(rename = "related", skip_serializing_if = "Option::is_none")]
//     pub related: Option<Box<LinkProperty>>,
//     #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
//     pub param_self: Option<Box<LinkProperty>>,
// }

// WARN: I had to change from the generated code above to Option<String> despite the spec.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(rename = "related", skip_serializing_if = "Option::is_none")]
    pub related: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonIssueModelAttributes {
    #[serde(rename = "coordinates")]
    pub coordinates: Option<Vec<Coordinate>>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// A description of the issue in Markdown format
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The type from enumeration of the issue’s severity level. This is usually set from the issue’s producer, but can be overridden by policies.
    #[serde(rename = "effective_severity_level")]
    pub effective_severity_level: Option<EffectiveSeverityLevel>,
    /// The Snyk vulnerability ID.
    #[serde(rename = "key")]
    pub key: Option<String>,
    #[serde(rename = "problems")]
    pub problems: Option<Vec<Problem>>,
    /// The severity level of the vulnerability: ‘low’, ‘medium’, ‘high’ or ‘critical’.
    #[serde(rename = "severities")]
    pub severities: Option<Vec<Severity>>,
    #[serde(rename = "slots")]
    pub slots: Option<Box<Slots>>,
    /// A human-readable title for this issue.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// The issue type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// When the vulnerability information was last modified.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

/// The type from enumeration of the issue’s severity level. This is usually set from the issue’s producer, but can be overridden by policies.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EffectiveSeverityLevel {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Coordinate {
    #[serde(rename = "remedies")]
    pub remedies: Option<Vec<Remedy>>,
    /// The affected versions of this vulnerability.
    #[serde(rename = "representation")]
    pub representation: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssuesMeta {
    #[serde(rename = "package")]
    pub package: Option<Box<PackageMeta>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssuesResponse {
    #[serde(rename = "data")]
    pub data: Option<Vec<CommonIssueModel>>,
    #[serde(rename = "jsonapi")]
    pub jsonapi: Option<Box<JsonApi>>,
    #[serde(rename = "links")]
    pub links: Option<Box<PaginatedLinks>>,
    #[serde(rename = "meta")]
    pub meta: Option<Box<IssuesMeta>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Org {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<OrgAttributes>>,
    /// The Snyk ID corresponding to this org
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Content type.
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrgAttributes {
    /// The ID of a Group.
    #[serde(rename = "group_id")]
    pub group_id: Option<uuid::Uuid>,
    /// Whether this organization belongs to an individual, rather than a Group.
    #[serde(rename = "is_personal")]
    pub is_personal: bool,
    /// Friendly name of the organization.
    #[serde(rename = "name")]
    pub name: String,
    /// Unique URL sanitized name of the organization for accessing it in Snyk.
    #[serde(rename = "slug")]
    pub slug: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PackageMeta {
    /// The package’s name
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A name prefix, such as a maven group id or docker image owner
    #[serde(rename = "namespace")]
    pub namespace: Option<String>,
    /// The package type or protocol
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The purl of the package
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// The version of the package
    #[serde(rename = "version")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Problem {
    /// When this problem was disclosed to the public.
    #[serde(rename = "disclosed_at")]
    pub disclosed_at: Option<String>,
    /// When this problem was first discovered.
    #[serde(rename = "discovered_at")]
    pub discovered_at: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "source")]
    pub source: String,
    /// When this problem was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// An optional URL for this problem.
    #[serde(rename = "url")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectAttributes {
    #[serde(rename = "business_criticality")]
    pub business_criticality: Option<Vec<BusinessCriticality>>,
    /// The date that the project was created on
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "environment")]
    pub environment: Option<Vec<Environment>>,
    #[serde(rename = "lifecycle")]
    pub lifecycle: Option<Vec<Lifecycle>>,
    /// Project name.
    #[serde(rename = "name")]
    pub name: String,
    /// The origin the project was added from.
    #[serde(rename = "origin")]
    pub origin: String,
    /// Describes if a project is currently monitored or it is de-activated.
    #[serde(rename = "status")]
    pub status: ProjectStatus,
    #[serde(rename = "tags")]
    pub tags: Option<Vec<ProjectAttributesTagsInner>>,
    /// Path within the target to identify a specific file/directory/image etc. when scanning just part  of the target, and not the entity.
    #[serde(rename = "target_file")]
    pub target_file: String,
    /// The additional information required to resolve which revision of the resource should be scanned.
    #[serde(rename = "target_reference")]
    pub target_reference: String,
    /// The package manager of the project.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BusinessCriticality {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "frontend")]
    Frontend,
    #[serde(rename = "backend")]
    Backend,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "saas")]
    Saas,
    #[serde(rename = "onprem")]
    Onprem,
    #[serde(rename = "hosted")]
    Hosted,
    #[serde(rename = "distributed")]
    Distributed,
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Lifecycle {
    #[serde(rename = "production")]
    Production,
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "sandbox")]
    Sandbox,
}

/// Describes if a project is currently monitored or it is de-activated.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

impl Default for ProjectStatus {
    fn default() -> ProjectStatus {
        Self::Active
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectAttributesTagsInner {
    #[serde(rename = "key")]
    pub key: Option<String>,
    #[serde(rename = "value")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectRelationships {
    #[serde(rename = "importer")]
    pub importer: Option<Box<Relationship>>,
    #[serde(rename = "organization")]
    pub organization: Box<Relationship>,
    #[serde(rename = "owner")]
    pub owner: Option<Box<Relationship>>,
    #[serde(rename = "target")]
    pub target: Box<ProjectRelationshipsTarget>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectRelationshipsTarget {
    #[serde(rename = "data")]
    pub data: Box<TargetData>,
    #[serde(rename = "links")]
    pub links: Box<RelatedLink>,
    /// Free-form object that may contain non-standard information.
    #[serde(rename = "meta")]
    pub meta: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelatedLink {
    #[serde(rename = "related", skip_serializing_if = "Option::is_none")]
    pub related: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Relationship {
    #[serde(rename = "data")]
    pub data: Box<RelationshipData>,
    #[serde(rename = "links")]
    pub links: Box<RelatedLink>,
    /// Free-form object that may contain non-standard information.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipData {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Type of the related resource
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Remedy {
    /// A markdown-formatted optional description of this remedy.
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "details")]
    pub details: Option<Box<RemedyDetails>>,
    /// The type of the remedy. Always ‘indeterminate’.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemedyDetails {
    /// A minimum version to upgrade to in order to remedy the issue.
    #[serde(rename = "upgrade_package")]
    pub upgrade_package: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SbomResource {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SbomResponse {
    #[serde(rename = "data")]
    pub data: Box<SbomResource>,
    #[serde(rename = "jsonapi")]
    pub jsonapi: Box<JsonApi>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScanAttributes {
    /// When the scan was created
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "deleted_at", default, with = "::serde_with::rust::double_option")]
    pub deleted_at: Option<Option<String>>,
    /// Environment ID
    #[serde(rename = "environment_id")]
    pub environment_id: Option<uuid::Uuid>,
    /// Error message if the scan failed
    #[serde(rename = "error", deserialize_with = "Option::deserialize")]
    pub error: Option<String>,
    /// When the scan finished
    #[serde(rename = "finished_at", default, with = "::serde_with::rust::double_option")]
    pub finished_at: Option<Option<String>>,
    /// Scan kind
    #[serde(rename = "kind", deserialize_with = "Option::deserialize")]
    pub kind: Option<Kind>,
    #[serde(rename = "options", default, with = "::serde_with::rust::double_option")]
    pub options: Option<Option<serde_json::Value>>,
    /// Organization ID
    #[serde(rename = "organization_id")]
    pub organization_id: Option<uuid::Uuid>,
    /// Errors that didn't fail the scan
    #[serde(rename = "partial_errors")]
    pub partial_errors: Option<String>,
    /// Increment for each change to a scan
    #[serde(rename = "revision")]
    pub revision: f32,
    /// Scan status
    #[serde(rename = "status", deserialize_with = "Option::deserialize")]
    pub status: Option<ScanStatus>,
    /// When the scan was last updated
    #[serde(rename = "updated_at", default, with = "::serde_with::rust::double_option")]
    pub updated_at: Option<Option<String>>,
}

/// Scan kind
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "user_initiated")]
    UserInitiated,
    #[serde(rename = "event_driven")]
    EventDriven,
    #[serde(rename = "null")]
    Null,
}

/// Scan status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScanStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "null")]
    Null,
}

impl Default for ScanStatus {
    fn default() -> ScanStatus {
        Self::Queued
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Severity {
    #[serde(rename = "level")]
    pub level: Option<String>,
    /// The CVSSv3 value of the vulnerability.
    #[serde(rename = "score", default, with = "::serde_with::rust::double_option")]
    pub score: Option<Option<f32>>,
    /// The source of this severity. The value must be the id of a referenced problem or class, in which case that problem or class is the source of this issue. If source is omitted, this severity is sourced internally in the Snyk application.
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// The CVSSv3 value of the vulnerability.
    #[serde(rename = "vector", default, with = "::serde_with::rust::double_option")]
    pub vector: Option<Option<String>>,
}


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Slots {
    /// The time at which this vulnerability was disclosed.
    #[serde(rename = "disclosure_time")]
    pub disclosure_time: Option<String>,
    /// The exploit maturity. Value of ‘No Data’, ‘Not Defined’, ‘Unproven’, ‘Proof of Concept’, ‘Functional’ or ‘High’.
    #[serde(rename = "exploit")]
    pub exploit: Option<String>,
    /// The time at which this vulnerability was published.
    #[serde(rename = "publication_time")]
    pub publication_time: Option<String>,
    #[serde(rename = "references")]
    pub references: Option<Vec<SlotsReferencesInner>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SlotsReferencesInner {
    /// Descriptor for an external reference to the issue
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// URL for an external reference to the issue
    #[serde(rename = "url")]
    pub url: Option<String>,
}


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

// #[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
// pub struct PaginatedLinks {
//     #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
//     pub first: Option<Box<LinkProperty>>,
//     #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
//     pub last: Option<Box<LinkProperty>>,
//     #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
//     pub next: Option<Box<LinkProperty>>,
//     #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
//     pub prev: Option<Box<LinkProperty>>,
//     #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
//     pub param_self: Option<Box<LinkProperty>>,
// }

// WARN: I had to change from the generated code above to Option<String> despite the spec.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaginatedLinks {
    #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TargetData {
    // WARN: Had to change this to an option despite the spec definition.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<TargetDataAttributes>>,
    /// The Resource ID.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The Resource type.
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TargetDataAttributes {
    /// The human readable name that represents this target. These are generated based on the provided properties, and the source. In the future we may support updating this value.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The URL for the resource. We do not use this as part of our representation of the identity of the target, as it can      be changed externally to Snyk We are reliant on individual integrations providing us with this value. Currently it is only provided by the CLI
    #[serde(rename = "url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url: Option<Option<String>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkProperty {
    /// A string containing the link’s URL.
    #[serde(rename = "href")]
    pub href: String,
    /// Free-form object that may contain non-standard information.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, serde_json::Value>>,
}
