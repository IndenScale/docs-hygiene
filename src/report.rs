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
    pub info_count: usize,
    pub hint_count: usize,
}

impl Report {
    pub fn new(diagnostics: Vec<Diagnostic>, files_checked: usize, root: &Path) -> Self {
        let error_count = diagnostics
            .iter()
            .filter(|diag| matches!(diag.severity, Severity::Error))
            .count();
        let warning_count = diagnostics
            .iter()
            .filter(|diag| matches!(diag.severity, Severity::Warning))
            .count();
        let info_count = diagnostics
            .iter()
            .filter(|diag| matches!(diag.severity, Severity::Info))
            .count();
        let hint_count = diagnostics
            .iter()
            .filter(|diag| matches!(diag.severity, Severity::Hint))
            .count();
        Self {
            summary: Summary {
                files_checked,
                diagnostic_count: diagnostics.len(),
                error_count,
                warning_count,
                info_count,
                hint_count,
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
            "DH_ASCII_001" => Some("A document contains a forbidden ASCII art block."),
            "DH_REPRESENTATION_001" => {
                Some("A canonical document is missing a localized representation.")
            }
            "DH_REPRESENTATION_002" => {
                Some("A localized representation has no canonical document.")
            }
            "DH_LANG_001" => {
                Some("A document has less CJK content than its language policy expects.")
            }
            "DH_LANG_002" => {
                Some("A document has more CJK content than its language policy allows.")
            }
            "DH_CONTRACT_001" => {
                Some("A path-inferred document profile is missing a required section.")
            }
            "DH_CONTRACT_002" => {
                Some("A path-inferred document profile is missing a required field.")
            }
            "DH_CONTRACT_003" => {
                Some("A required document section contains a declared placeholder.")
            }
            "DH_CONTRACT_004" => {
                Some("Required document sections are not in the configured order.")
            }
            "DH_MATURITY_001" => {
                Some("Repository size signals recommend a higher document governance maturity.")
            }
            "DH_CONCEPT_001" => Some("A highlighted term is missing a concept definition file."),
            "DH_CONCEPT_002" => Some("A concept definition file is not referenced by docs."),
            "DH_GOVERNANCE_001" => {
                Some("A governance manifest is missing, malformed, duplicated, or invalid.")
            }
            "DH_REFERENCE_001" => {
                Some("A Body is missing a valid same-refinement-level reference to a Library.")
            }
            "DH_LIBRARY_001" => Some(
                "A Library directory has a missing, malformed, duplicate, or undeclared member term.",
            ),
            "DH_BODY_001" => Some(
                "A directory Body Package has a missing, malformed, duplicate, undeclared, or localized-mismatched member.",
            ),
            "DH_DERIVATION_001" => {
                Some("A Body has a missing or invalid adjacent-refinement-level derivation.")
            }
            "DH_DERIVATION_002" => {
                Some("A Library has a missing or invalid adjacent-refinement-level projection.")
            }
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
