use serde::Deserialize;
use serde_json;
use std::env;
#[cfg(test)]
use std::path::PathBuf;
use std::process::Command;
use std::str;

use crate::Error;

#[derive(Debug, PartialEq, Deserialize, Clone)]
/// Model that represents a value within the scorecard
pub struct ReportValue {
    #[serde(alias = "Ratio")]
    ratio: f32,
    #[serde(alias = "Reasoning")]
    reasoning: String,
    #[serde(alias = "MaxPoints")]
    max_points: u32,
}

#[derive(Debug, PartialEq, Deserialize)]
/// Model that represents the scorecard metadata
pub struct ReportMetadata {
    #[serde(alias = "TotalPackages")]
    total_packages: u32,
}

#[derive(Debug, PartialEq, Deserialize)]
/// Model that represents sbom scorecard results
pub struct Report {
    #[serde(alias = "Compliance")]
    compliance: ReportValue,
    #[serde(alias = "PackageIdentification")]
    package_identification: ReportValue,
    #[serde(alias = "PackageVersions")]
    package_versions: ReportValue,
    #[serde(alias = "PackageLicenses")]
    package_licenses: ReportValue,
    #[serde(alias = "CreationInfo")]
    creation_info: ReportValue,
    #[serde(alias = "Total")]
    total: ReportValue,
    #[serde(alias = "Metadata")]
    metadata: ReportMetadata,
}

/// Prints sbom scorecard results to stdout.
pub fn show(sbom_path: String) -> Result<String, Error> {
    println!("Generating scorecard from sbom file: {}", sbom_path);
    match env::var("SBOM_SCORECARD") {
        Ok(sbom_scorecard) => {
            match Command::new(sbom_scorecard)
                .arg("score")
                .arg(sbom_path)
                .output()
            {
                Ok(output) => match output.stderr.is_empty() {
                    false => Err(Error::SbomScorecard(format!(
                        "sbom-scorecard stderr: {}",
                        String::from_utf8_lossy(output.stderr.as_slice())
                    ))),
                    true => Ok(String::from_utf8_lossy(output.stdout.as_slice()).to_string()),
                },
                Err(error) => Err(Error::SbomScorecard(format!(
                    "sbom-scorecard failed with error: {}",
                    error
                ))),
            }
        }
        Err(e) => Err(Error::SbomScorecard(format!(
            "sbom-scorecard application not installed: {}",
            e
        ))),
    }
}

/// Uses the sbom-scorecard utility to create an SBOMScorecard Object
pub fn generate(sbom_path: String) -> Result<Report, Error> {
    println!("Generating scorecard from sbom file: {}", sbom_path);
    match env::var("SBOM_SCORECARD") {
        Ok(sbom_scorecard) => {
            let result = Command::new(sbom_scorecard)
                .arg("score")
                .arg(sbom_path)
                .arg("--outputFormat")
                .arg("json")
                .output()
                .expect("failed to execute process");
            let raw_report = str::from_utf8(&result.stdout).unwrap().to_string();

            let report: Report = serde_json::from_str(&raw_report).unwrap_or({
                let empty_row = ReportValue {
                    ratio: 0.0,
                    reasoning: String::new(),
                    max_points: 0,
                };
                Report {
                    compliance: empty_row.clone(),
                    package_identification: empty_row.clone(),
                    package_versions: empty_row.clone(),
                    package_licenses: empty_row.clone(),
                    creation_info: empty_row.clone(),
                    total: empty_row,
                    metadata: ReportMetadata { total_packages: 0 },
                }
            });

            Ok(report)
        }
        Err(e) => Err(Error::SbomScorecard(format!(
            "sbom-scorecard application not installed: {}",
            e
        ))),
    }
}

/// Compares two Sboms total scores, and returns a String with details about which is the higher score.
pub fn compare(sbom_1_path: String, sbom_2_path: String) -> Result<String, Error> {
    let scorecard_1_result = generate(sbom_1_path.clone());
    let scorecard_2_result = generate(sbom_2_path.clone());

    match (scorecard_1_result, scorecard_2_result) {
        (Ok(scorecard_1), Ok(scorecard_2)) => {
            let precision = 0;
            let scorecard_1_details = format!(
                "Scorecard 1:({})\n=> Has a total score of {:.2$}/100",
                sbom_1_path,
                100.0 * scorecard_1.total.ratio,
                precision
            );
            let scorecard_2_details = format!(
                "Scorecard 2:({})\n=> Has a total score of {:.2$}/100",
                sbom_2_path,
                100.0 * scorecard_2.total.ratio,
                precision
            );

            let mut compare_results =
                format!("{}\n\n{}\n\n", scorecard_1_details, scorecard_2_details);

            if scorecard_1.total.ratio > scorecard_2.total.ratio {
                compare_results.push_str("Scorecard 1 has a higher score!");
            } else if scorecard_1.total.ratio < scorecard_2.total.ratio {
                compare_results.push_str("Scorecard 2 has a higher score!");
            } else {
                compare_results.push_str("The two scorecards have a matching score!");
            }

            Ok(compare_results)
        }
        (Ok(_), Err(error)) => {
            let error_string = format!(
                "Unable due to compare scorecards due to error with second scorecard:\n{}",
                error
            );
            Err(Error::SbomScorecard(format!(
                "Failed to compare scorecards due to errors: {}",
                error_string
            )))
        }
        (Err(error), Ok(_)) => {
            let error_string = format!(
                "Unable due to compare scorecards due to error with first scorecard:\n{}",
                error
            );
            Err(Error::SbomScorecard(format!(
                "Failed to compare scorecards due to errors: {}",
                error_string
            )))
        }
        (Err(error_1), Err(error_2)) => {
            let error_string = format!("\n{}\n{}", error_1, error_2);
            Err(Error::SbomScorecard(format!(
                "Failed to compare scorecards due to errors: {}",
                error_string
            )))
        }
    }
}

#[test]
fn compare_matching_sboms() {
    let mut sbom_1_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    sbom_1_path.push("src/services/sboms/scorecard/test_files/dropwizard.json");

    let result = compare(
        sbom_1_path.display().to_string(),
        sbom_1_path.display().to_string(),
    );

    assert!(
        result.is_ok(),
        "Failed to get a result with error {:#?}",
        result.err()
    );
    assert!(
        result
            .unwrap()
            .contains("The two scorecards have a matching score!"),
        "The scorecards should be matching"
    );
}

#[test]
fn compare_not_matching_sboms() {
    let mut sbom_1_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    sbom_1_path.push("src/services/sboms/scorecard/test_files/dropwizard.json");

    let mut sbom_2_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    sbom_2_path.push("src/services/sboms/scorecard/test_files/keycloak.json");

    let result = compare(
        sbom_1_path.display().to_string(),
        sbom_2_path.display().to_string(),
    );

    assert!(
        result.is_ok(),
        "Failed to get a result with error {:#?}",
        result.err()
    );
    assert!(
        result.unwrap().contains("Scorecard 2 has a higher score!"),
        "The second scorecard should be more highly rated"
    );
}

#[test]
pub fn test_get_orgs() {
    let mut test_sbom = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_sbom.push("src/services/sboms/scorecard/test_files/dropwizard.json");

    let scorecard = generate(test_sbom.display().to_string());
    assert!(
        scorecard.is_ok(),
        "Failed to get a result with error {:#?}",
        scorecard.err()
    );
    let unwrapped_scorecard = scorecard.unwrap();
    assert!(
        unwrapped_scorecard.compliance.ratio == 1.0,
        "Compliance ratio should be 1"
    );
    assert!(
        unwrapped_scorecard.compliance.reasoning == "",
        "Compliance reasoning should be an empty string"
    );
    assert!(
        unwrapped_scorecard.compliance.max_points == 25,
        "Compliance max points should be 25"
    );

    assert!(
        unwrapped_scorecard.package_identification.ratio == 1.0,
        "PackageIdentification ratio should be 1"
    );
    assert!(
        unwrapped_scorecard.package_identification.reasoning
            == "100% have either a purl (100%) or CPE (0%)",
        "PackageIdentification reasoning should be 100% have either a purl (100%) or CPE (0%)"
    );
    assert!(
        unwrapped_scorecard.package_identification.max_points == 20,
        "PackageIdentification max points should be 20"
    );
}
