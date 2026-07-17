use std::path::Path;

use serde::Serialize;

use crate::checks::{Diagnostic, DiagnosticData, DiagnosticRange, RelatedInformation};

use super::{OwnershipReport, Report, Severity, Summary, TopologyExceptionEvidence};

pub fn print_text_report(report: &Report) {
    if report.ownership.enabled {
        println!(
            "Ownership: responsibility {}/{}, review {}/{}, knowledge redundancy {}/{}; {} due soon, {} expired.",
            report.ownership.responsibility_coverage.covered,
            report.ownership.responsibility_coverage.total,
            report.ownership.review_coverage.covered,
            report.ownership.review_coverage.total,
            report.ownership.knowledge_redundancy_coverage.covered,
            report.ownership.knowledge_redundancy_coverage.total,
            report.ownership.reviews_due_soon,
            report.ownership.reviews_expired,
        );
    }
    for exception in &report.topology_exceptions {
        println!(
            "Topology exception {} [{}]: node {}, {:?}, degree {:?}, global {:?}, exception {}, remaining {:?}, trend {:?}, impact [{}].",
            exception.id,
            exception.status.label(),
            exception.node,
            exception.direction,
            exception.current_degree,
            exception.global_budget,
            exception.exception_budget,
            exception.remaining,
            exception.trend_delta,
            exception.transitive_impact.join(", "),
        );
    }
    if report.diagnostics.is_empty() {
        println!(
            "docs-hygiene passed: {} files checked.",
            report.summary.files_checked
        );
        return;
    }
    for diagnostic in &report.diagnostics {
        let line = format!(":{}", diagnostic.range.start.line + 1);
        println!(
            "{} {:?} {}{}: {}",
            diagnostic.code, diagnostic.severity, diagnostic.path, line, diagnostic.message
        );
    }
    println!(
        "\n{} diagnostics: {} errors, {} warnings, {} info, {} hints across {} docs files.",
        report.summary.diagnostic_count,
        report.summary.error_count,
        report.summary.warning_count,
        report.summary.info_count,
        report.summary.hint_count,
        report.summary.files_checked
    );
}

pub fn print_json_report(report: &Report) -> anyhow::Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(&JsonReport::from(report))?
    );
    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonReport<'a> {
    schema_version: &'static str,
    summary: &'a Summary,
    diagnostics: Vec<JsonDiagnostic<'a>>,
    ownership: &'a OwnershipReport,
    topology_exceptions: &'a [TopologyExceptionEvidence],
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonDiagnostic<'a> {
    source: &'a str,
    code: &'a str,
    severity: Severity,
    message: &'a str,
    path: &'a str,
    uri: String,
    range: &'a DiagnosticRange,
    related_information: Vec<JsonRelatedInformation<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<&'a DiagnosticData>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonRelatedInformation<'a> {
    path: &'a str,
    uri: String,
    range: &'a DiagnosticRange,
    message: &'a str,
}

impl<'a> From<&'a Report> for JsonReport<'a> {
    fn from(report: &'a Report) -> Self {
        Self {
            schema_version: "docs-hygiene.diagnostic.v1",
            summary: &report.summary,
            diagnostics: report
                .diagnostics
                .iter()
                .map(|diagnostic| JsonDiagnostic::new(report, diagnostic))
                .collect(),
            ownership: &report.ownership,
            topology_exceptions: &report.topology_exceptions,
        }
    }
}

impl<'a> JsonDiagnostic<'a> {
    fn new(report: &'a Report, diagnostic: &'a Diagnostic) -> Self {
        Self {
            source: diagnostic.source,
            code: diagnostic.code,
            severity: diagnostic.severity,
            message: &diagnostic.message,
            path: &diagnostic.path,
            uri: file_uri(&report.root, &diagnostic.path),
            range: &diagnostic.range,
            related_information: diagnostic
                .related_information
                .iter()
                .map(|related| JsonRelatedInformation::new(report, related))
                .collect(),
            data: diagnostic.data.as_ref(),
        }
    }
}

impl<'a> JsonRelatedInformation<'a> {
    fn new(report: &'a Report, related: &'a RelatedInformation) -> Self {
        Self {
            path: &related.path,
            uri: file_uri(&report.root, &related.path),
            range: &related.range,
            message: &related.message,
        }
    }
}

fn file_uri(root: &Path, path: &str) -> String {
    let absolute = if path == "." {
        root.to_path_buf()
    } else {
        root.join(path)
    };
    format!("file://{}", absolute.display())
}
