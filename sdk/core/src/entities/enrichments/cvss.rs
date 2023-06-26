use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Summary of relevant CVSS data points.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Cvss {
    /// Indicates the [CVSS 3 maturity](https://www.first.org/cvss/specification-document) of the
    /// [Vulnerability].
    pub maturity: Option<Maturity>,

    /// Mean score.
    pub mean_score: Option<f32>,

    /// Median score.
    pub median_score: Option<f32>,

    /// Mode score.
    pub mode_score: Option<f32>,

    /// Optional set of associated scores from the enrichment provider.
    pub scores: Option<Vec<Score>>,
}

impl Cvss {
    /// Calculates mean, mode, and median for CVSS scores in the summary.
    pub fn calculate_scores(&mut self) {
        match &self.scores {
            None => {}
            Some(_scores) => {
                // let raw_scores = scores.iter().map(|score| score.score).collect();
                self.mean_score = None; //Some(mean(raw_scores.clone()));
                self.mode_score = None; //Some(mode(raw_scores.clone()));
                self.median_score = None; //Some(median(raw_scores.clone()));
            }
        }
    }
}

/// Discriminator used to indicate which CVSS version was used to render a score.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Version {
    /// CVSS Version 1.0.
    V1,
    /// CVSS Version 2.0.
    V2,
    /// CVSS Version 3.0.
    V3,
    /// CVSS Version 3.1.
    V3_1,
    /// CVSS Version 4.0.
    V4,
    /// Unknown CVSS version.
    Unknown,
}

impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.0" => Ok(Version::V1),
            "2.0" => Ok(Version::V2),
            "3.0" => Ok(Version::V3),
            "3.1" => Ok(Version::V3_1),
            "4.0" => Ok(Version::V4),
            _ => Err(()),
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::V1 => write!(f, "1.0"),
            Version::V2 => write!(f, "2.0"),
            Version::V3 => write!(f, "3.0"),
            Version::V3_1 => write!(f, "3.1"),
            Version::V4 => write!(f, "4.0"),
            Version::Unknown => write!(f, "unknown"),
        }
    }
}

/// A CVSS Score for a [Vulnerability] returned by an enrichment provider.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Score {
    /// The calculated CVSS base score.
    pub score: f32,
    /// Optional source of the score.
    pub source: Option<String>,
    /// Version of CVSS used to calculate the score.
    pub version: Option<Version>,
    /// A compressed textual representation of the values used to derive the score ([NIST](https://nvd.nist.gov/vuln-metrics/cvss#)).
    pub vector: Option<String>,
}

/// Indicates the [maturity](https://www.first.org/cvss/specification-document) of the [Vulnerability].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Maturity {
    /// Insufficient information to choose one of the other values. Has the same effect on scoring
    /// as assigning High.
    NotDefined,
    /// Functional autonomous code exists, or no exploit is required (manual trigger) and details
    /// are widely available.
    High,
    /// Functional exploit code is available.
    Functional,
    /// Proof-of-concept exploit code is available, or an attack demonstration is not practical for
    /// most systems.
    ProofOfConcept,
    /// No exploit code is available, or an exploit is theoretical.
    Unproven,
}

impl Display for Maturity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Maturity::NotDefined => write!(f, "notDefined"),
            Maturity::High => write!(f, "high"),
            Maturity::Functional => write!(f, "functional"),
            Maturity::ProofOfConcept => write!(f, "proofOfConcept"),
            Maturity::Unproven => write!(f, "unproven"),
        }
    }
}
