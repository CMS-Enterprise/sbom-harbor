use platform::mongodb::MongoDocument;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use platform::mongodb::mongo_doc;
use crate::entities::tasks::{Task, TaskRef};
use crate::Error;

use std::result::Result;

/// The Manifest struct is the root of the report
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    /// The id of the report...?
    pub id: String,
    /// This value is the “name” of the package.  We should have
    /// this value because it will help us identify the product it
    /// references. The actual value will often be a shortened,
    /// single name of the component. Examples: commons-lang3
    /// and jquery
    pub name: String,
    /// Package manager of the Package
    pub package_manager: String,
    /// The purl of the Package
    pub purl: String,
    /// What Provider extracted the package
    pub provider: String,
    /// This value is the component version. We should have this
    /// value because it is necessary to identify the version of
    /// the component. The version should ideally comply with semantic
    /// versioning but is not enforced
    pub version: String,
    /// This value is the date/time the Report was created.  We should
    /// have this value because it is useful for tracking when reports
    /// are created. This field is an ISO 8601 timestamp representing
    /// the time the report was finished being generated.
    pub created: String,
    /// This value is where the actual report data is located. It is
    /// a list of Report objects.
    pub report: Vec<Report>,
    /// Reference to each [Task] that was performed against this [Package].
    pub task_refs: Vec<TaskRef>,
}

impl Manifest {

    /// Sets up a reference between the [Manifest] and a [Task].
    pub fn join_task(&mut self, task: &Task) -> Result<TaskRef, Error> {
        if task.id.is_empty() {
            return Err(
                Error::Entity(
                    "task_id_required".to_string()
                )
            );
        }

        let task_ref = TaskRef::new(task, self.purl.clone());

        let result = task_ref.clone();
        self.task_refs.push(task_ref);

        Ok(result)
    }

    /// Add a [TaskRef] to the [Manifest].
    pub fn task_refs(&mut self, task_ref: &TaskRef) {
        if !self.task_refs.iter().any(|s| s.task_id == task_ref.task_id) {
            self.task_refs.push(task_ref.clone());
        }
    }
}

mongo_doc!(Manifest);

/// The report section maps to the 'Report` portion of the specification
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    /// Name of the dependant Package
    pub name: String,
    /// Version of the dependant Package
    pub version: String,
    /// The CPE of the Package.
    pub cpe: String,
    /// The purl of the dependant Package
    pub purl: String,
    /// Any enrichments the dependent Package has
    pub enrichments: Vec<Enrichment>,
}

/// The 'Enrichment' section of the specification
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enrichment {
    /// Indicates which enrichment source reported the vulnerability.
    /// This value is the name of the enrichment source.  We should
    /// have the enrichment source name to specify which enrichment
    /// source the “enrichment” data came from.
    pub provider: String,
    /// This is the list of actual results that came from the
    /// enrichment source.  It is a list of enrichment result objects
    pub results: Vec<Vulnerability>,
}

/// The 'Result' portion is a generic Vulnerability Result
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vulnerability {
    /// (CVSS v3) Indicates the severity of the Vulnerability.
    pub severity: String,
    /// The CVE ID of the Vulnerability.  CVE, short for Common
    /// Vulnerabilities and Exposures, is a list of publicly
    /// disclosed computer security flaws.
    pub cve: String,
    /// The CVE description of the Vulnerability.  Descriptions serve
    /// as a summary of the vulnerability and can include information
    /// such as the vulnerable product, impacts, attack vector, weakness
    /// or other relevant technical information. At times, CVEs may display
    /// a Current Description and Analysis Description
    pub description: String,
    /// Optional CVSS Detail from the enrichment provider.  Common
    /// Vulnerability Scoring System (CVSS) is a method used to supply a
    /// qualitative measure of severity. CVSS is not a measure of risk.
    pub cvss: String,
    /// Optional list of identified CWEs for the Vulnerability. Common
    /// Weakness Enumeration (CWE), which was created to identify common
    /// software security weaknesses.
    pub cwes: Vec<Cwe>,
    /// Optional advice from the enrichment provider on how to mitigate
    /// the Vulnerability. So far, only Snyk has this type of data.
    pub remediation: String,
}

/// Represents the portion of the report that contains the
/// Common Weakness Enumeration (CWE). The CWE is a community-developed
/// list of software and hardware weakness types. It serves
/// as a common language, a measuring stick for security tools,
/// and as a baseline for weakness identification, mitigation,
/// and prevention efforts.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cwe {
    /// the id of the CWE
    pub id: i64,
}


