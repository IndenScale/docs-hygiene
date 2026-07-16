use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use anyhow::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::activation::{ActivationReport, RuleDecision, RuleState, evaluate_rule_activation};
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
    let activation = evaluate_rule_activation(root, config)?;
    let ignore = build_ignore(root, config)?;
    let mut diagnostics = Vec::new();

    add_activation_guidance(config, &activation, &mut diagnostics);

    let mut local = Vec::new();
    check_required_files(root, config, &mut local);
    append_rule_diagnostics(
        &mut diagnostics,
        local,
        activation.decision("project.entry-docs"),
    );

    let mut structure = Vec::new();
    let docs = collect_docs(root, config, &ignore, &mut structure)?;
    check_numbering(config, &docs, &mut structure);
    check_max_lines(root, config, &docs, &mut structure)?;
    check_ascii_art(root, config, &docs, &mut structure)?;
    check_markdown_links(root, config, &docs, &mut structure)?;
    append_rule_diagnostics(
        &mut diagnostics,
        structure,
        activation.decision("docs.structure"),
    );

    let mut localization = Vec::new();
    check_language_representations(config, &docs, &mut localization);
    check_language(root, config, &docs, &mut localization)?;
    append_rule_diagnostics(
        &mut diagnostics,
        localization,
        activation.decision("localization.parity"),
    );

    let mut contracts = Vec::new();
    check_document_contracts(root, config, &ignore, &docs, &mut contracts)?;
    append_rule_diagnostics(
        &mut diagnostics,
        contracts,
        activation.decision("documents.contracts"),
    );

    let mut concepts = Vec::new();
    check_concepts(root, config, &docs, &ignore, &mut concepts)?;
    append_rule_diagnostics(
        &mut diagnostics,
        concepts,
        activation.decision("concepts.references"),
    );

    let identity = activation.decision("governance.identity");
    let domain_fanout = activation.decision("governance.domain-fanout");
    let traceability = activation.decision("governance.traceability");
    if identity.state != RuleState::Inactive
        || domain_fanout.state != RuleState::Inactive
        || traceability.state != RuleState::Inactive
    {
        let mut governance = Vec::new();
        check_governance(root, config, &mut governance);
        for diagnostic in governance {
            let decision = if matches!(diagnostic.code, "DH_DERIVATION_001" | "DH_DERIVATION_002") {
                traceability
            } else if diagnostic.code == "DH_DOMAIN_001" {
                domain_fanout
            } else {
                identity
            };
            append_rule_diagnostics(&mut diagnostics, vec![diagnostic], decision);
        }
    }

    let adapters = activation.decision("adapters.external");
    if adapters.state != RuleState::Inactive {
        let mut adapter_diagnostics = Vec::new();
        check_adapters(root, config, &mut adapter_diagnostics)?;
        append_rule_diagnostics(&mut diagnostics, adapter_diagnostics, adapters);
    }
    let diagnostics = apply_suppressions(config, diagnostics)?;

    Ok(Report::new(diagnostics, docs.len(), root))
}

fn append_rule_diagnostics(
    target: &mut Vec<Diagnostic>,
    diagnostics: Vec<Diagnostic>,
    decision: &RuleDecision,
) {
    if decision.state == RuleState::Inactive {
        return;
    }
    target.extend(diagnostics.into_iter().map(|mut diagnostic| {
        diagnostic.severity = match decision.state {
            RuleState::Inactive => unreachable!(),
            RuleState::Advisory => Severity::Info,
            RuleState::Warning => Severity::Warning,
            RuleState::Error => diagnostic.severity,
        };
        diagnostic
    }));
}

fn add_activation_guidance(
    config: &Config,
    activation: &ActivationReport,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for decision in &activation.decisions {
        if !matches!(decision.state, RuleState::Advisory | RuleState::Warning)
            || has_explicit_feature_policy(&decision.rule, config)
        {
            continue;
        }
        let severity = if decision.state == RuleState::Warning {
            Severity::Warning
        } else {
            Severity::Info
        };
        diagnostics.push(Diagnostic::new(
            "DH_ACTIVATION_001",
            severity,
            ".",
            format!(
                "Rule '{}' activated as {}. Activated because: {}. Why this matters: {} How to fix: {}",
                decision.rule,
                decision.state.label(),
                decision.evidence.join("; "),
                decision.rationale,
                decision.remediation
            ),
        ));
    }
}

fn has_explicit_feature_policy(rule: &str, config: &Config) -> bool {
    if config.rules.contains_key(rule) {
        return true;
    }
    match rule {
        "project.entry-docs" => {
            !config.entry_docs.required.is_empty() || !config.required_files.is_empty()
        }
        "docs.structure" => {
            !config.docs.bases.is_empty()
                || config.docs.require_continuous_numbering
                || config.docs.max_lines.is_some()
                || config.docs.forbid_ascii_art
        }
        "documents.contracts" => {
            !config.document_contracts.profiles.is_empty()
                || !config
                    .document_contracts
                    .maturity
                    .recommendations
                    .is_empty()
        }
        "localization.parity" => {
            !config.language_representations.localized.is_empty()
                || config
                    .docs
                    .bases
                    .iter()
                    .any(|base| !base.localized_roots.is_empty())
                || !config.language.is_empty()
        }
        "concepts.references" => {
            config.concepts.require_concept_file || config.concepts.fail_on_orphan_concept.is_some()
        }
        "governance.identity" | "governance.domain-fanout" | "governance.traceability" => {
            !config.governance.manifests.is_empty()
        }
        "adapters.external" => config.adapters.markdownlint.enabled,
        _ => false,
    }
}

// Keep each policy surface independently reviewable. These implementation
// units are included into this module so the split does not widen internal APIs.
include!("checks/governance_models.rs");
include!("checks/domain_fanout.rs");
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
