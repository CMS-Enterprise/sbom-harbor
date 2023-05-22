use serde::Deserialize;
use std::env;
#[cfg(test)]
use std::path::PathBuf;
use std::process::Command;
use std::str;
use serde_json;


#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct RowData {
    #[serde(alias = "Ratio")]
    ratio: f32,
    #[serde(alias = "Reasoning")]
    reasoning: String,
    #[serde(alias = "MaxPoints")]
    max_points: u32
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SbomScorecardMetadata {
    #[serde(alias = "TotalPackages")]
    total_packages: u32
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SbomScorecard {
    #[serde(alias = "Compliance")]
    compliance: RowData,
    #[serde(alias = "PackageIdentification")]
    package_identification: RowData,
    #[serde(alias = "PackageVersions")]
    package_versions: RowData,
    #[serde(alias = "PackageLicenses")]
    package_licenses: RowData,
    #[serde(alias = "CreationInfo")]
    creation_info: RowData,
    #[serde(alias = "Total")]
    total: RowData,
    #[serde(alias = "Metadata")]
    metadata: SbomScorecardMetadata
}

pub fn show_sbom_scorecard(sbom_path: String) -> String {
    println!("Generating scorecard from sbom file: {}", sbom_path);
    match env::var(format!("SBOM_SCORECARD")) {
        Ok(sbom_scorecard) => {
            let result = Command::new(sbom_scorecard)
                .arg("score")
                .arg(sbom_path)
                .output()
                .expect("failed to execute process");

            if !result.stderr.is_empty() {
                return String::from_utf8_lossy(&result.stderr).to_string();
            }
            else {
                return String::from_utf8_lossy(&result.stdout).to_string();
            }
        }
        Err(e) => panic!("sbom-scorecard application not installed: {}", e),
    }
}

/// Uses the sbom-scorecard utility to create an SBOMScorecard Object
pub fn generate_sbom_scorecard(sbom_path: String) -> SbomScorecard {
    println!("Generating scorecard from sbom file: {}", sbom_path);
    match env::var(format!("SBOM_SCORECARD")) {
        Ok(sbom_scorecard) => {
            let result = Command::new(sbom_scorecard)
                .arg("score")
                .arg(sbom_path)
                .arg("--outputFormat")
                .arg("json")
                .output()
                .expect("failed to execute process");
            let raw_scorecard = str::from_utf8(&result.stdout).unwrap().to_string();

            let scorecard_obj: SbomScorecard = serde_json::from_str(&raw_scorecard).unwrap_or({
                let empty_row = RowData{ ratio: 0.0, reasoning: format!(""), max_points: 0 };
                SbomScorecard { compliance: empty_row.clone(), package_identification: empty_row.clone(), package_versions: empty_row.clone(), package_licenses: empty_row.clone(), creation_info: empty_row.clone(), total: empty_row.clone(), metadata: SbomScorecardMetadata{total_packages:0} }
            });

            return scorecard_obj;

        }
        Err(e) => panic!("sbom-scorecard application not installed: {}", e),
    }
}

/// Compares two Sboms total scores, and returns a String with details about which is the higher score.
pub fn compare_sbom_scorecards(sbom_1_path: String, sbom_2_path: String) -> String {

    let scorecard_1 = generate_sbom_scorecard(sbom_1_path.clone());
    let scorecard_2 = generate_sbom_scorecard(sbom_2_path.clone());

    let precision = 0;
    let scorecard_1_details = format!("Scorecard 1:({})\n=> Has a total score of {:.2$}/100", sbom_1_path.clone(), 100.0 * scorecard_1.total.ratio, precision);
    let scorecard_2_details = format!("Scorecard 2:({})\n=> Has a total score of {:.2$}/100", sbom_2_path.clone(), 100.0 * scorecard_2.total.ratio, precision);
   
    let mut compare_results = format!("{}\n\n{}\n\n", scorecard_1_details, scorecard_2_details);
 
    if scorecard_1.total.ratio > scorecard_2.total.ratio {
        compare_results.push_str(&format!("Scorecard 1 has a higher score!"));
    }
    else if scorecard_1.total.ratio < scorecard_2.total.ratio {
        compare_results.push_str(&format!("Scorecard 2 has a higher score!"));
    }
    else {
        compare_results.push_str(&format!("The two scorecards have a matching score!"));
    }

    return compare_results;
}

#[test]
fn compare_matching_sboms() {
    let mut sbom_1_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    sbom_1_path.push("src/services/sboms/sbom_scorecard/test_files/dropwizard.json");

    let result = compare_sbom_scorecards(
        sbom_1_path.display().to_string(),
        sbom_1_path.display().to_string(),
    );

}

#[test]
fn compare_not_matching_sboms() {
    let mut sbom_1_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    sbom_1_path.push("src/services/sboms/sbom_scorecard/test_files/dropwizard.json");

    let mut sbom_2_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    sbom_2_path.push("src/services/sboms/sbom_scorecard/test_files/keycloak.json");

    let result = compare_sbom_scorecards(
        sbom_1_path.display().to_string(),
        sbom_2_path.display().to_string(),
    );

}

#[test]
pub fn test_get_orgs() {
    let mut test_sbom = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_sbom.push("src/services/sboms/sbom_scorecard/test_files/dropwizard.json");

    let scorecard = generate_sbom_scorecard(test_sbom.display().to_string());
    assert!(scorecard.compliance.ratio == 1.0, "Compliance ratio should be 1");
    assert!(scorecard.compliance.reasoning == "", "Compliance reasoning should be an empty string");
    assert!(scorecard.compliance.max_points == 25, "Compliance max points should be 25");

    assert!(scorecard.package_identification.ratio == 1.0, "PackageIdentification ratio should be 1");
    assert!(scorecard.package_identification.reasoning == "100% have either a purl (100%) or CPE (0%)", "PackageIdentification reasoning should be 100% have either a purl (100%) or CPE (0%)");
    assert!(scorecard.package_identification.max_points == 20, "PackageIdentification max points should be 20");
}

#[test]
pub fn test_show_score() {
    let mut test_sbom = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_sbom.push("src/services/sboms/sbom_scorecard/test_files/dropwizard.json");

    let result = show_sbom_scorecard(test_sbom.display().to_string());

    println!("{}", result);
}
