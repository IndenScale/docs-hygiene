use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use anyhow::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::config::{Config, DocumentProfileConfig, FilenamePatternConfig, MaturityLevel};
use crate::report::{Report, Severity};

#[derive(Debug)]
pub struct Diagnostic {
    pub source: &'static str,
    pub code: &'static str,
    pub severity: Severity,
    pub path: String,
    pub range: DiagnosticRange,
    pub message: String,
    pub related_information: Vec<RelatedInformation>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticRange {
    pub start: DiagnosticPosition,
    pub end: DiagnosticPosition,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticPosition {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug)]
pub struct RelatedInformation {
    pub path: String,
    pub range: DiagnosticRange,
    pub message: String,
}

impl Diagnostic {
    fn new(
        code: &'static str,
        severity: Severity,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            source: "docs-hygiene",
            code,
            severity,
            path: path.into(),
            range: DiagnosticRange::origin(),
            message: message.into(),
            related_information: Vec::new(),
        }
    }

    fn with_source(mut self, source: &'static str) -> Self {
        self.source = source;
        self
    }

    fn at_line(mut self, line: usize) -> Self {
        self.range = DiagnosticRange::line(line);
        self
    }

    fn with_related(mut self, related: RelatedInformation) -> Self {
        self.related_information.push(related);
        self
    }
}

impl DiagnosticRange {
    fn origin() -> Self {
        Self {
            start: DiagnosticPosition {
                line: 0,
                character: 0,
            },
            end: DiagnosticPosition {
                line: 0,
                character: 0,
            },
        }
    }

    fn line(line: usize) -> Self {
        let zero_based = line.saturating_sub(1);
        Self {
            start: DiagnosticPosition {
                line: zero_based,
                character: 0,
            },
            end: DiagnosticPosition {
                line: zero_based,
                character: 0,
            },
        }
    }
}

impl RelatedInformation {
    fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            range: DiagnosticRange::origin(),
            message: message.into(),
        }
    }
}

#[derive(Debug)]
struct DocFile {
    base_id: String,
    rel: PathBuf,
    lang: Option<String>,
    number: Option<u32>,
    stem: String,
    numbered: bool,
}

struct NormalizedBase {
    id: String,
    root: PathBuf,
    localized_roots: BTreeMap<String, PathBuf>,
    patterns: Vec<FilenamePatternConfig>,
    require_continuous_numbering: bool,
    max_lines: Option<usize>,
    ignore: Vec<String>,
}

pub fn run_checks(root: &Path, config: &Config) -> Result<Report> {
    let ignore = build_ignore(root, config)?;
    let mut diagnostics = Vec::new();

    check_required_files(root, config, &mut diagnostics);
    let docs = collect_docs(root, config, &ignore, &mut diagnostics)?;
    check_numbering(config, &docs, &mut diagnostics);
    check_language_representations(config, &docs, &mut diagnostics);
    check_max_lines(root, config, &docs, &mut diagnostics)?;
    check_ascii_art(root, config, &docs, &mut diagnostics)?;
    check_language(root, config, &docs, &mut diagnostics)?;
    check_markdown_links(root, config, &docs, &mut diagnostics)?;
    check_document_contracts(root, config, &ignore, &docs, &mut diagnostics)?;
    check_concepts(root, config, &docs, &ignore, &mut diagnostics)?;
    check_governance(root, config, &mut diagnostics);
    check_adapters(root, config, &mut diagnostics)?;
    let diagnostics = apply_suppressions(config, diagnostics)?;

    Ok(Report::new(diagnostics, docs.len(), root))
}

// Keep each policy surface independently reviewable. These implementation
// units are included into this module so the split does not widen internal APIs.
include!("checks/governance_models.rs");
include!("checks/package_structure.rs");
include!("checks/package_localization.rs");
include!("checks/wiki_references.rs");
include!("checks/derivation.rs");
include!("checks/document_contracts.rs");
include!("checks/repository_structure.rs");
include!("checks/repository_content.rs");
include!("checks/support.rs");

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    include!("checks/tests/documents.rs");
    include!("checks/tests/policies.rs");
    include!("checks/tests/governance_packages.rs");
    include!("checks/tests/governance_graph.rs");
}
