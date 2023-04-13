use crate::entities::packages::SnykXRef;
use crate::Error;
use tracing::debug;

/// Purl is a derived type that facilitates analysis of a Package across the entire enterprise.
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Purl {
    /// Unique identifier for the Package URL.
    pub id: String,

    /// The raw PackageURL.
    pub raw: String,

    /// The package name extracted from the package URL.
    pub name: String,

    /// The package version extracted from the package URL.
    pub version: Option<String>,

    /// Source of the Purl.
    pub source: Source,

    /// A map of projects that are associated with the Purl.
    pub(crate) snyk_refs: Vec<SnykXRef>,
}

impl Purl {
    pub(crate) fn parse(raw: String) -> Result<Purl, Error> {
        if !raw.contains("@") {
            return (raw, "".to_string());
        }

        if raw.matches("@").count() > 1 {
            return (raw, "unknown".to_string());
        }

        let parts: Vec<&str> = raw.split("@").collect();
        if parts.len() != 2 {
            debug!("should not be possible")
        }

        (parts[0].to_string(), parts[1].to_string())
    }
}

/// The Source of the Purl.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Source {
    /// The Purl was extracted from the Component Metadata of a Package in the Registry.
    Package,
    /// The Purl was extracted from a Dependency of a Package in the Registry.
    Dependency,
}

impl ToString for Source {
    fn to_string(&self) -> String {
        match self {
            Source::Package => "package".to_string(),
            Source::Dependency => "dependency".to_string(),
        }
    }
}

impl Purl {
    pub(crate) fn merge_snyk_refs(&mut self, refs: Vec<SnykXRef>) {
        for r in refs {
            if self.snyk_refs.iter().any(|existing| r.eq(existing)) {
                continue;
            }

            self.snyk_refs.push(r);
        }
    }
}
