use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use anyhow::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::activation::{
    ActivationReport, RuleApplicability, RuleChecker, RuleDecision, RuleSpec, RuleState,
    evaluate_rule_activation, rule_spec, rule_spec_for_diagnostic,
};
use crate::config::{
    Config, CoreClaimOccurrencePolicy, DocumentMatchConfig, DocumentProfileConfig,
    DocumentTemplateConfig, FilenamePatternConfig, MaturityLevel, RequiredFieldConfig,
    RequiredSectionConfig, SlugNormalization, SlugRenamePolicy, SlugSchemaConfig, SlugSourceConfig,
};
use crate::governance::{
    ContentAnchor, ContentAnchorScope, GovernanceEdge, GovernanceEdgeKind, GovernanceGraph,
    GovernanceLocation, GovernanceNode, LifecycleProvenance, ReferenceRelation, RefinementLevel,
};
use crate::reference::{
    CONTEXT_GOVERNED_ANCHOR, CONTEXT_GOVERNED_CONTENT, CONTEXT_IDENTITY_DECLARATION,
    CONTEXT_PROJECT_NAVIGATION, REFERENCE_POLICIES, ReferenceAnchorPayload, ReferenceDisposition,
    ReferenceOccurrence, ReferencePayload, ReferencePolicy, SYNTAX_FRONTMATTER,
    SYNTAX_MARKDOWN_LINK, SYNTAX_WIKI_LINK, reference_disposition,
};
use crate::report::TemplateRevisionReport;
use crate::report::{DocumentTemplateReport, Report, Severity, SuppressedDiagnostic};

#[derive(Debug)]
pub struct Diagnostic {
    pub source: &'static str,
    pub code: &'static str,
    pub severity: Severity,
    pub path: String,
    pub range: DiagnosticRange,
    pub message: String,
    pub related_information: Vec<RelatedInformation>,
    pub data: Option<DiagnosticData>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticData {
    pub original_value: String,
    pub normalized_value: String,
    pub document_kind: String,
    pub conflict_path: Option<String>,
    pub remediation: String,
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
            data: None,
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

    fn with_data(mut self, data: DiagnosticData) -> Self {
        self.data = Some(data);
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
    document_kind: String,
    pattern_id: String,
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
    run_checks_with_activation(root, config, &activation)
}

pub(crate) fn run_checks_with_activation(
    root: &Path,
    config: &Config,
    activation: &ActivationReport,
) -> Result<Report> {
    let ignore = build_ignore(root, config)?;
    let mut diagnostics = Vec::new();

    add_activation_guidance(config, activation, &mut diagnostics);

    let mut local = Vec::new();
    check_required_files(root, config, &mut local);
    append_rule_diagnostics(
        &mut diagnostics,
        local,
        activation.decision_for(RuleChecker::EntryDocs),
    );

    let mut structure = Vec::new();
    let docs = collect_docs(root, config, &ignore, &mut structure)?;
    check_numbering(config, &docs, &mut structure);
    check_max_lines(root, config, &docs, &mut structure)?;
    check_ascii_art(root, config, &docs, &mut structure)?;
    check_markdown_links(root, config, &docs, &mut structure)?;
    check_slug_identities(root, config, &docs, &mut structure)?;
    append_rule_diagnostics(
        &mut diagnostics,
        structure,
        activation.decision_for(RuleChecker::DocumentStructure),
    );

    let mut localization = Vec::new();
    check_language_representations(config, &docs, &mut localization);
    check_language(root, config, &docs, &mut localization)?;
    append_rule_diagnostics(
        &mut diagnostics,
        localization,
        activation.decision_for(RuleChecker::Localization),
    );

    let mut contracts = Vec::new();
    let document_templates =
        check_document_contracts(root, config, &ignore, &docs, &mut contracts)?;
    check_document_kinds(root, config, &docs, &mut contracts)?;
    append_rule_diagnostics(
        &mut diagnostics,
        contracts,
        activation.decision_for(RuleChecker::DocumentContracts),
    );

    let mut concepts = Vec::new();
    check_concepts(root, config, &docs, &ignore, &mut concepts)?;
    append_rule_diagnostics(
        &mut diagnostics,
        concepts,
        activation.decision_for(RuleChecker::Concepts),
    );

    let identity = activation.decision_for(RuleChecker::GovernanceIdentity);
    let traceability = activation.decision_for(RuleChecker::GovernanceTraceability);
    let topology = activation.decision_for(RuleChecker::GovernanceTopology);
    let mut semantic_content_anchors_checked = 0;
    let mut governance_graph = GovernanceGraph::default();
    if identity.state != RuleState::Inactive
        || traceability.state != RuleState::Inactive
        || topology.state != RuleState::Inactive
    {
        let mut governance = Vec::new();
        governance_graph = check_governance(root, config, &mut governance);
        check_topology_policy(config, &governance_graph, &mut governance);
        semantic_content_anchors_checked = governance_graph
            .metrics
            .relation_counts
            .get(&GovernanceEdgeKind::PinnedReference)
            .copied()
            .unwrap_or_default();
        for diagnostic in governance {
            let spec = rule_spec_for_diagnostic(diagnostic.code)
                .expect("native governance diagnostics must be owned by the rule registry");
            let decision = activation.decision_for(spec.checker);
            append_rule_diagnostics(&mut diagnostics, vec![diagnostic], decision);
        }
    }

    let adapters = activation.decision_for(RuleChecker::ExternalAdapters);
    if adapters.state != RuleState::Inactive {
        let mut adapter_diagnostics = Vec::new();
        check_adapters(root, config, &mut adapter_diagnostics)?;
        append_rule_diagnostics(&mut diagnostics, adapter_diagnostics, adapters);
    }
    let (diagnostics, suppressed) = apply_suppressions(config, diagnostics)?;

    Ok(Report::new(diagnostics, docs.len(), root)
        .with_suppressed(suppressed)
        .with_semantic_content_anchors_checked(semantic_content_anchors_checked)
        .with_governance_graph(governance_graph)
        .with_document_templates(document_templates))
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
            || has_explicit_feature_policy(
                rule_spec(&decision.rule)
                    .expect("activation decisions must be owned by the rule registry"),
                config,
            )
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

fn has_explicit_feature_policy(spec: &RuleSpec, config: &Config) -> bool {
    if config.rules.contains_key(spec.id) {
        return true;
    }
    match spec.applicability {
        RuleApplicability::EntryDocs => {
            !config.entry_docs.required.is_empty() || !config.required_files.is_empty()
        }
        RuleApplicability::DocumentStructure => {
            !config.docs.bases.is_empty()
                || config.docs.require_continuous_numbering
                || config.docs.max_lines.is_some()
                || config.docs.forbid_ascii_art
        }
        RuleApplicability::DocumentContracts => {
            !config.document_contracts.profiles.is_empty()
                || !config.document_kinds.is_empty()
                || !config
                    .document_contracts
                    .maturity
                    .recommendations
                    .is_empty()
        }
        RuleApplicability::Localization => {
            !config.language_representations.localized.is_empty()
                || config
                    .docs
                    .bases
                    .iter()
                    .any(|base| !base.localized_roots.is_empty())
                || !config.language.is_empty()
        }
        RuleApplicability::Concepts => {
            config.concepts.require_concept_file || config.concepts.fail_on_orphan_concept.is_some()
        }
        RuleApplicability::GovernanceIdentity | RuleApplicability::GovernanceTraceability => {
            !config.governance.manifests.is_empty()
        }
        RuleApplicability::GovernanceTopology => {
            config.governance.topology.configured_policy_count() > 0
        }
        RuleApplicability::ExternalAdapters => config.adapters.markdownlint.enabled,
    }
}

// Keep each policy surface independently reviewable. These implementation
// units are included into this module so the split does not widen internal APIs.
include!("checks/governance_models.rs");
include!("checks/lifecycle.rs");
include!("checks/library_claims.rs");
include!("checks/library_claim_scan.rs");
include!("checks/package_structure.rs");
include!("checks/package_localization.rs");
include!("checks/reference_collectors.rs");
include!("checks/reference_normalization.rs");
include!("checks/selectors.rs");
include!("checks/anchors.rs");
include!("checks/wiki_references.rs");
include!("checks/derivation.rs");
include!("checks/topology.rs");
include!("checks/document_templates.rs");
include!("checks/document_contracts.rs");
include!("checks/document_kinds.rs");
include!("checks/repository_structure.rs");
include!("checks/slug_identities.rs");
include!("checks/repository_content.rs");
include!("checks/support.rs");

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    include!("checks/tests/documents.rs");
    include!("checks/tests/slug_identities.rs");
    include!("checks/tests/policies.rs");
    include!("checks/tests/template_lifecycle.rs");
    include!("checks/tests/governance_packages.rs");
    include!("checks/tests/governance_graph.rs");
    include!("checks/tests/anchors.rs");
    include!("checks/tests/lifecycle.rs");
    include!("checks/tests/reference_ir.rs");
    include!("checks/tests/selectors.rs");
    include!("checks/tests/topology.rs");
    include!("checks/tests/library_claims.rs");
}
