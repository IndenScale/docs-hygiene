use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::checks::{Diagnostic, DiagnosticRange, RelatedInformation};

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Debug)]
pub struct Report {
    pub diagnostics: Vec<Diagnostic>,
    pub summary: Summary,
    pub root: PathBuf,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub files_checked: usize,
    pub diagnostic_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
}

impl Report {
    pub fn new(diagnostics: Vec<Diagnostic>, files_checked: usize, root: &Path) -> Self {
        let error_count = diagnostics
            .iter()
            .filter(|diag| matches!(diag.severity, Severity::Error))
            .count();
        let warning_count = diagnostics.len() - error_count;
        Self {
            summary: Summary {
                files_checked,
                diagnostic_count: diagnostics.len(),
                error_count,
                warning_count,
            },
            diagnostics,
            root: root.to_path_buf(),
        }
    }

    pub fn explain(code: &str) -> Option<&'static str> {
        match code {
            "DH_REQUIRED_001" => Some("A required repository documentation file is missing."),
            "DH_NAME_001" => Some("A docs file name does not match the configured pattern."),
            "DH_SEQ_001" => Some("A numbered docs directory has a gap in its sequence."),
            "DH_SEQ_002" => Some("A numbered docs directory uses the same number more than once."),
            "DH_SIZE_001" => Some("A docs file exceeds the configured line budget."),
            "DH_I18N_001" => Some("A root docs file is missing a localized counterpart."),
            "DH_I18N_002" => Some("A localized docs file has no root counterpart."),
            "DH_LANG_001" => {
                Some("A document has less CJK content than its language policy expects.")
            }
            "DH_LANG_002" => {
                Some("A document has more CJK content than its language policy allows.")
            }
            "DH_CONCEPT_001" => Some("A highlighted term is missing a concept definition file."),
            "DH_CONCEPT_002" => Some("A concept definition file is not referenced by docs."),
            "DH_ADAPTER_001" => Some("An external documentation adapter reported a failure."),
            "DH_SUPPRESSION_001" => Some("A diagnostic was suppressed by configuration."),
            _ => None,
        }
    }
}

pub fn print_text_report(report: &Report) {
    if report.diagnostics.is_empty() {
        println!(
            "docs-hygiene passed: {} files checked.",
            report.summary.files_checked
        );
        return;
    }

    for diag in &report.diagnostics {
        let line = format!(":{}", diag.range.start.line + 1);
        println!(
            "{} {:?} {}{}: {}",
            diag.code, diag.severity, diag.path, line, diag.message
        );
    }
    println!(
        "\n{} diagnostics: {} errors, {} warnings across {} docs files.",
        report.summary.diagnostic_count,
        report.summary.error_count,
        report.summary.warning_count,
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
