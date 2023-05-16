use regex::Regex;
use std::process::Command;
use std::str;

#[derive(Debug, PartialEq)]
struct SbomScorecardRow {
    index: String,
    field_name: String,
    points: String,
    reasoning: String,
}

#[derive(Debug, PartialEq)]
struct SbomScorecard {
    rows: Vec<SbomScorecardRow>,
    summary: String,
}

// Converts a raw string into an SbomScorecard object
fn get_scorecard_from_string(raw_scorecard: String) -> SbomScorecard {

    let t = raw_scorecard.chars().filter(|c| c.is_ascii()).collect::<String>();

    let t = t.replace("\n\n", "\n");
    let collection: Vec<&str> = t.split("\n").collect();

    let re = Regex::new(r"\s{2,}").unwrap();

    let mut scorecard_rows: Vec<SbomScorecardRow> = Vec::new();
    let mut summary: String = format!("");

    for row in collection {

        // Do nothing if we have an empty row or with the header
        if !row.is_empty() && !row.starts_with(" #"){
            let record = re.replace_all(&row, "||");
            let record: Vec<&str> = record.split("||").collect();

            if record.len() >= 4 {
                let sbom_scorecard_row = SbomScorecardRow {
                    index: record.get(0).unwrap_or(&"").to_string(),
                    field_name: record.get(1).unwrap_or(&"").to_string(),
                    points: record.get(2).unwrap_or(&"").to_string(),
                    reasoning: record.get(3).unwrap_or(&"").to_string(),
                };
                scorecard_rows.push(sbom_scorecard_row);
            } else {
                for field in record {
                    summary.push_str(field);
                }
            }
        }
    }
    let sbom_scorecard = SbomScorecard {
        rows: scorecard_rows,
        summary,
    };
    return sbom_scorecard;
}

// Uses the sbom-scorecard utility to get a raw string representation of an sbom scorecard from stdout
fn retrieve_sbom_scorecard(sbom_path: String) -> String {
    let result = Command::new("/home/jshattjr/sbom-scorecard")
    .arg("score")
    .arg(sbom_path)
    .output()
    .expect("failed to execute process");

    return str::from_utf8(&result.stdout).unwrap().to_string();
}

// Compares two Sboms, and returns true if they match. We may wish to do a more in-depth comparison in the future
fn is_matching_sbom(sbom_1_path: String, sbom_2_path: String) -> bool {
    let raw_scorecard_1 = retrieve_sbom_scorecard(sbom_1_path);
    let scorecard_1 = get_scorecard_from_string(raw_scorecard_1);

    let raw_scorecard_2 = retrieve_sbom_scorecard(sbom_2_path);
    let scorecard_2 = get_scorecard_from_string(raw_scorecard_2);

    if scorecard_1 == scorecard_2 {
        return true;
    } else {
        return false;
    }
}

#[test]
fn compare_matching_sboms() {
    let sbom_1_path = format!("/home/jshattjr/SBOM/dropwizard-1.3.15/bom.json");
    let sbom_2_path = format!("/home/jshattjr/SBOM/dropwizard-1.3.15/bom.json");

    let result = is_matching_sbom(sbom_1_path, sbom_2_path);

    assert!(result == true, "Sboms should be matching");
}

#[test]
fn compare_not_matching_sboms() {
    let sbom_1_path = format!("/home/jshattjr/SBOM/dropwizard-1.3.15/bom.json");
    let sbom_2_path = format!("/home/jshattjr/SBOM/keycloak-10.0.2/bom.json");

    let result = is_matching_sbom(sbom_1_path, sbom_2_path);

    assert!(result == false, "Sboms should be matching");
}

#[test]
pub fn test_get_orgs() {
    let path = format!("/home/jshattjr/SBOM/dropwizard-1.3.15/bom.json");
    let raw_scorecard = retrieve_sbom_scorecard(path);
    let scorecard = get_scorecard_from_string(raw_scorecard);

    println!("{:#?}", scorecard);
}