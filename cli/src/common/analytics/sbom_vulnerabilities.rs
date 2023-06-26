use crate::Error;
use crate::OutputFormat;
use harbcore::entities::enrichments::cvss::{Cvss, Maturity, Version};
use harbcore::entities::enrichments::{Cwe, Severity};
use harbcore::services::analytics::models::{PackageSummary, SbomSummary, VulnerabilitySummary};

/// Formats the Output of the `sbom-vulnerabilities` command using the specified formatter.
pub fn format_sbom_summaries(
    summaries: Vec<SbomSummary>,
    format: &OutputFormat,
) -> Result<String, Error> {
    let output = match format {
        OutputFormat::Json => {
            serde_json::to_string(&summaries).map_err(|e| Error::Runtime(e.to_string()))?
        }
        OutputFormat::Csv => to_csv(summaries)?,
        OutputFormat::Text => format!("{:#?}", summaries),
    };

    Ok(output)
}

fn to_csv(summaries: Vec<SbomSummary>) -> Result<String, Error> {
    let headers = csv_headers();
    let mut rows: Vec<Vec<String>> = vec![];

    for summary in summaries.iter() {
        let next = csv_rows(summary);
        rows.push(next);
    }

    for row in rows.iter() {
        for inner in row.iter() {
            println!("{}", inner);
        }
    }

    platform::persistence::csv::to_csv(headers, rows).map_err(|e| Error::Runtime(e.to_string()))
}

fn csv_headers() -> Vec<&'static str> {
    vec![
        "id",
        "name",
        "version",
        "package_manager",
        "sbom_purl",
        "author",
        "provider",
        //"supplier_name",
        "dependency_id",
        "dependency_purl",
        "dependency_version",
        //"dependency_cpe",
        "vulnerability_id",
        "vulnerability_provider",
        "severity",
        "cve",
        "epss_score",
        "cwes",
        "cvss",
        //"description",
    ]
}

/// Generate a cartesian product of sbom to dependency to vulnerability for inclusion in a
/// flattened csv.
fn csv_rows(summary: &SbomSummary) -> Vec<String> {
    let mut rows = vec![];

    if summary.dependencies.is_empty() {
        return vec![format_row(summary, None, None)];
    }

    for dependency in summary.dependencies.iter() {
        if dependency.vulnerabilities.is_empty() {
            rows.push(format_row(summary, Some(dependency), None))
        } else {
            for vuln in dependency.vulnerabilities.iter() {
                rows.push(format_row(summary, Some(dependency), Some(vuln)));
            }
        }
    }

    rows
}

fn format_row(
    sbom: &SbomSummary,
    package: Option<&PackageSummary>,
    vuln: Option<&VulnerabilitySummary>,
) -> String {
    let sbom_provider = match &sbom.provider {
        None => "".to_string(),
        Some(p) => p.to_string(),
    };

    let binding = PackageSummary::default();
    let package = match package {
        None => &binding,
        Some(p) => p,
    };

    let binding = VulnerabilitySummary::default();
    let vuln = match vuln {
        None => &binding,
        Some(v) => v,
    };

    vec![
        sbom.id.clone(),
        sbom.name.clone().unwrap_or("NULL".to_string()),
        sbom.version.clone().unwrap_or("NULL".to_string()),
        sbom.package_manager.clone().unwrap_or("NULL".to_string()),
        sbom.purl.clone().unwrap_or("NULL".to_string()),
        sbom.author.clone().to_string(),
        sbom_provider,
        // sbom.supplier_name.clone().unwrap_or("NULL".to_string()),
        package.id.clone(),
        package.purl.clone().unwrap_or("NULL".to_string()),
        package.version.clone().unwrap_or("NULL".to_string()),
        // package.cpe.clone().unwrap_or("NULL".to_string()),
        vuln.id.clone(),
        format_vulnerability_provider(vuln),
        vuln.severity.unwrap_or(Severity::None).to_string(),
        vuln.cve.clone().unwrap_or("NULL".to_string()),
        vuln.epss_score.unwrap_or(0.0_f32).to_string(),
        vuln.cwes
            .clone()
            .unwrap_or(vec![Cwe {
                id: "NULL".to_string(),
                name: None,
                description: None,
            }])
            .iter()
            .map(|cwe| cwe.id.clone())
            .collect::<Vec<String>>()
            .join("|"),
        format_cvss(&vuln.cvss),
        // vuln.description.clone().unwrap_or("NULL".to_string()),
    ]
    .join(",")
}

fn format_cvss(cvss: &Option<Cvss>) -> String {
    match cvss {
        None => "".to_string(),
        Some(cvss) => {
            let scores = match cvss.scores.clone() {
                None => "".to_string(),
                Some(scores) => scores
                    .iter()
                    .map(|score| {
                        format!(
                            "{}|{}|{}|{}",
                            score.source.clone().unwrap_or("".to_string()),
                            score.score,
                            score.version.clone().unwrap_or(Version::Unknown),
                            score.vector.clone().unwrap_or("".to_string())
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("||"),
            };

            format!(
                "{}|{}",
                cvss.maturity.clone().unwrap_or(Maturity::NotDefined),
                scores
            )
        }
    }
}

fn format_vulnerability_provider(vuln: &VulnerabilitySummary) -> String {
    match vuln.id.is_empty() {
        true => "NULL".to_string(),
        false => vuln.provider.clone().to_string(),
    }
}
