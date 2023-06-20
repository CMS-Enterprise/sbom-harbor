mod models;

use crate::entities::enrichments::Vulnerability;
use crate::Error;
use models::Match;
use serde::{Deserialize, Serialize};
use std::process::Command;

/// The kind of scan Grype should perform.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ScanKind {
    /// Directory
    Directory,
    /// Sbom file
    Sbom,
    /// Container image archive
    Image,
}

impl ScanKind {
    /// Returns a directory, file, or image name formatted for use as an arg to the Grype CLI.
    pub fn to_grype_arg(&self, target_path: &str) -> String {
        match self {
            ScanKind::Directory => format!("dir:{}", target_path),
            ScanKind::Sbom => format!("sbom:{}", target_path),
            ScanKind::Image => target_path.to_string(),
        }
    }
}

/// Scans for vulnerabilities by shelling out to the Syft CLI.
pub fn scan(target_path: &str, kind: ScanKind) -> Result<Vec<Vulnerability>, Error> {
    let mut cmd = Command::new("grype");
    let cmd = cmd
        .arg(kind.to_grype_arg(target_path))
        .arg("-o")
        .arg("json");

    let stdout = platform::process::execute(cmd, "grype")?;
    let output: GrypeOutput =
        serde_json::from_str(stdout.as_str()).map_err(|e| Error::Serde(format!("{}", e)))?;

    let mut vulns = vec![];
    for m in output.matches.iter() {
        let vuln = m.to_vulnerability()?;
        vulns.push(vuln);
    }

    Ok(vulns)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GrypeOutput {
    matches: Vec<Match>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_results_from_grype() -> Result<(), Error> {
        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("cannot access CARGO_MANIFEST_DIR");

        let manifest_dir = manifest_dir.replace("/sdk/core", "");
        let _ = scan(manifest_dir.as_str(), ScanKind::Directory)?;

        Ok(())
    }
}
