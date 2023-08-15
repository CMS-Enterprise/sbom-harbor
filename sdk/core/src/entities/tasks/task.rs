use crate::entities::analytics::AnalyticProviderKind;
use crate::entities::enrichments::VulnerabilityProviderKind;
use crate::entities::sboms::SbomProviderKind;
use crate::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// A [Task] is a value type that allows tracking and correlating operations performed by Harbor.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Task {
    /// The unique identifier for the [Task] batch.
    pub id: String,

    /// Discriminator indicating the kind of task performed.
    pub kind: TaskKind,

    /// The total number of items to be processed by the [Task].
    pub count: u64,

    /// The unix timestamp for when the [Task] was created.
    pub timestamp: u64,

    /// Human readable start time.
    pub start: DateTime<Utc>,

    /// Human readable end time.
    pub finish: DateTime<Utc>,

    /// The duration of the completed [Task] in seconds.
    pub duration_seconds: i64,

    /// Result of the [Task].
    pub status: TaskStatus,

    /// Optional error message if the [Task] failed.
    pub err: Option<String>,

    /// Map of recoverable errors that occurred during the [Task]. Used to track recoverable
    /// errors and the target that produced the error.
    pub ref_errs: Option<HashMap<String, String>>,

    /// The total count of errors encountered during the [Task].
    pub err_total: u64,
}

impl Task {
    /// Factory method to create new instance of type.
    pub fn new(kind: TaskKind) -> Result<Task, Error> {
        let timestamp = platform::time::timestamp().map_err(|e| Error::Runtime(e.to_string()))?;
        let now = Utc::now();

        Ok(Task {
            id: "".to_string(),
            kind,
            count: 0,
            timestamp,
            start: now,
            finish: now,
            duration_seconds: 0,
            status: TaskStatus::Started,
            err: None,
            ref_errs: None,
            err_total: 0,
        })
    }

    /// Add an error string for a specific target.
    pub fn ref_errs(&mut self, target_id: String, err: String) {
        let target_id = platform::persistence::s3::to_safe_object_key(target_id.as_str())
            .unwrap_or(format!("invalid-target-id-{}", uuid::Uuid::new_v4()));
        match self.ref_errs.clone() {
            None => {
                self.ref_errs = Some(HashMap::from([(target_id, err)]));
            }
            Some(mut ref_errs) => {
                ref_errs.insert(target_id, err);
                self.ref_errs = Some(ref_errs);
            }
        }
    }
}

/// Discriminator indicating the type of operation performed and which provider performed the
/// [Task].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskKind {
    /// [Task] was performed to assess Vulnerabilities.
    Vulnerabilities(VulnerabilityProviderKind),
    /// [Task] was performed to assess Sboms.
    Sbom(SbomProviderKind),
    /// [Task] was performed by a custom extension.
    Extension(String),
    /// [Task] was performed to execute an Analytic.
    Analytics(AnalyticProviderKind),
}

impl Display for TaskKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskKind::Vulnerabilities(kind) => write!(f, "vulnerabilities::{}", kind),
            TaskKind::Sbom(kind) => write!(f, "sbom::{}", kind),
            TaskKind::Extension(name) => write!(f, "extension::{}", name),
            TaskKind::Analytics(kind) => write!(f, "analytic::{}", kind),
        }
    }
}

/// Reference to an instance of a [Task]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskRef {
    /// The unique identifier for the [Task] batch.
    pub task_id: String,

    /// The unique identifier for the [Task] target.
    pub target_id: String,

    /// Optional error message if the [Task] failed for this target.
    pub err: Option<String>,
}

impl TaskRef {
    /// Factory method for creating new instance of type.
    pub fn new(task: &Task, target_id: String) -> Self {
        Self {
            task_id: task.id.clone(),
            target_id,
            err: None,
        }
    }
}

/// Used to track [Task] results and errors.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub enum TaskStatus {
    /// Task started.
    Started,
    /// Task completed successfully.
    Complete,
    /// Task completed with recoverable errors.
    CompleteWithErrors,
    /// Task completed with unrecoverable errors.
    Failed,
}
