use crate::entities::cyclonedx::Issue;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Identified security issue for a [Package].
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Finding {
    /// Unique identifier for the Finding.
    pub id: String,

    /// Indicates which system reported the vulnerability.
    pub source: Source,

    /// The Package URL for the finding.
    #[skip_serializing_none]
    pub purl: Option<String>,

    /// The source
    pub snyk_raw: Option<Issue>,
}

impl From<Issue> for Finding {
    fn from(value: Issue) -> Self {
        todo!()
    }
}

impl Finding {
    /// Compares the current finding with another to determine if they are functionally equal.
    /// Not an instance equality comparator.
    pub fn eq(&self, other: &Finding) -> bool {
        self.purl.eq(&other.purl)
            && self.source == other.source
            && self.snyk_raw.eq(&other.snyk_raw)
    }
}

/// Discriminator used to indicate what system identified a [Finding].
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Source {
    DependencyTrack,
    IonChannel,
    Snyk,
}
