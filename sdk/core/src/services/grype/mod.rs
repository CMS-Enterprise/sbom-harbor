mod models;

use crate::entities::enrichments::Vulnerability;
use crate::Error;
use models::Match;
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Generates vulnerabilities for a repository SBOM by shelling out to the Syft CLI.
pub fn grype(source_path: &str) -> Result<Vec<Vulnerability>, Error> {
    let mut cmd = Command::new("grype");
    let cmd = cmd
        .arg(format!("sbom:{}", source_path))
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
        Ok(())
    }
}
