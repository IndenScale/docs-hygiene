use std::path::Path;

// Governance Library: [[SDK-001]]

use anyhow::Result;
use serde::Serialize;

use crate::activation::{
    CapabilityDimension, HygieneMaturity, ProjectFacts, RuleDecision, RuleState,
    evaluate_rule_activation,
};
use crate::checks::run_checks_with_activation;
use crate::config::{Config, DimensionApplicability, RuleMode};
use crate::governance::GovernanceGraph;
use crate::report::DocumentTemplateReport;

mod configuration;
mod output;
mod registry;

use configuration::{
    configured_dimension, dimension_is_inferred_applicable, legacy_structure_target,
    validate_profile_config,
};

pub use output::{print_json_profile, print_text_profile};
pub use registry::{INVARIANTS, InvariantApplicability, InvariantDelivery, InvariantSpec};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DimensionStatus {
    MeetsTarget,
    BelowTarget,
    Observed,
    Unverified,
    NotApplicable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InvariantOutcome {
    Passed,
    Failed,
    Unverified,
    NotApplicable,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvariantEvidence {
    pub invariant: &'static str,
    pub minimum_maturity: HygieneMaturity,
    pub delivery: InvariantDelivery,
    pub outcome: InvariantOutcome,
    pub diagnostic_codes: Vec<String>,
    pub paths: Vec<String>,
    pub suppression_reasons: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionResult {
    pub dimension: CapabilityDimension,
    pub applicable: bool,
    pub required: bool,
    pub target: Option<HygieneMaturity>,
    pub observed: Option<HygieneMaturity>,
    pub status: DimensionStatus,
    pub rationale: Option<String>,
    pub evidence: Vec<InvariantEvidence>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HygieneProfileReport {
    pub schema_version: &'static str,
    pub facts: ProjectFacts,
    pub decisions: Vec<RuleDecision>,
    pub document_templates: DocumentTemplateReport,
    pub governance_graph: GovernanceGraph,
    pub dimensions: Vec<DimensionResult>,
    pub overall_observed: Option<HygieneMaturity>,
    pub meets_targets: bool,
}

pub fn evaluate_hygiene_profile(root: &Path, config: &Config) -> Result<HygieneProfileReport> {
    validate_profile_config(config)?;
    let activation = evaluate_rule_activation(root, config)?;
    let checks = run_checks_with_activation(root, config, &activation)?;
    let dimensions = CapabilityDimension::ALL
        .iter()
        .copied()
        .map(|dimension| evaluate_dimension(dimension, config, &activation, &checks))
        .collect::<Vec<_>>();
    let required = dimensions
        .iter()
        .filter(|dimension| dimension.applicable && dimension.required)
        .collect::<Vec<_>>();
    let overall_observed = if required.is_empty()
        || required
            .iter()
            .any(|dimension| dimension.observed.is_none())
    {
        None
    } else {
        required
            .iter()
            .filter_map(|dimension| dimension.observed)
            .min()
    };
    let meets_targets = required.iter().all(|dimension| {
        dimension
            .target
            .zip(dimension.observed)
            .is_some_and(|(target, observed)| observed >= target)
    });

    Ok(HygieneProfileReport {
        schema_version: "docs-hygiene.profile.v1",
        facts: activation.facts,
        decisions: activation.decisions,
        document_templates: checks.document_templates,
        governance_graph: checks.governance_graph,
        dimensions,
        overall_observed,
        meets_targets,
    })
}

fn evaluate_dimension(
    dimension: CapabilityDimension,
    config: &Config,
    activation: &crate::activation::ActivationReport,
    checks: &crate::report::Report,
) -> DimensionResult {
    let configured = configured_dimension(&config.hygiene_profile.dimensions, dimension);
    let legacy_target = (dimension == CapabilityDimension::Structure)
        .then(|| legacy_structure_target(config.document_contracts.maturity.declared));
    let target = configured.and_then(|value| value.target).or(legacy_target);
    let explicitly_not_applicable = configured
        .is_some_and(|value| value.applicability == DimensionApplicability::NotApplicable);
    let inferred = dimension_is_inferred_applicable(dimension, activation);
    let applicable =
        !explicitly_not_applicable && (configured.is_some() || legacy_target.is_some() || inferred);
    let required = applicable
        && configured
            .map(|value| value.required)
            .unwrap_or(dimension == CapabilityDimension::Structure);
    let rationale = configured.and_then(|value| value.rationale.clone());

    if !applicable {
        return DimensionResult {
            dimension,
            applicable,
            required: false,
            target: None,
            observed: None,
            status: DimensionStatus::NotApplicable,
            rationale,
            evidence: Vec::new(),
        };
    }

    let evidence = INVARIANTS
        .iter()
        .filter(|invariant| invariant.dimension == dimension)
        .map(|invariant| evaluate_invariant(invariant, config, activation, checks))
        .collect::<Vec<_>>();
    let observed = observed_maturity(&evidence);
    let status = match target {
        Some(target) if observed.is_some_and(|observed| observed >= target) => {
            DimensionStatus::MeetsTarget
        }
        Some(_) => DimensionStatus::BelowTarget,
        None if observed.is_some() => DimensionStatus::Observed,
        None => DimensionStatus::Unverified,
    };

    DimensionResult {
        dimension,
        applicable,
        required,
        target,
        observed,
        status,
        rationale,
        evidence,
    }
}

fn evaluate_invariant(
    invariant: &'static InvariantSpec,
    config: &Config,
    activation: &crate::activation::ActivationReport,
    checks: &crate::report::Report,
) -> InvariantEvidence {
    let decisions = invariant
        .checkers
        .iter()
        .map(|checker| activation.decision_for(*checker))
        .collect::<Vec<_>>();
    let active = decisions
        .iter()
        .any(|decision| decision.state != RuleState::Inactive);
    let disabled = decisions
        .iter()
        .any(|decision| decision.mode == RuleMode::Disabled);
    let applies = match invariant.applicability() {
        InvariantApplicability::Checker => active,
        InvariantApplicability::Capability => true,
        InvariantApplicability::ContentPolicy => {
            config.docs.max_lines.is_some()
                || config.docs.forbid_ascii_art
                || config
                    .docs
                    .bases
                    .iter()
                    .any(|base| base.max_lines.is_some())
        }
        InvariantApplicability::DocumentTemplates => {
            !config.document_contracts.profiles.is_empty()
                || !config.document_contracts.templates.is_empty()
                || configured_dimension(
                    &config.hygiene_profile.dimensions,
                    CapabilityDimension::Structure,
                )
                .and_then(|dimension| dimension.target)
                .unwrap_or_else(|| {
                    legacy_structure_target(config.document_contracts.maturity.declared)
                }) >= HygieneMaturity::Controlled
        }
        InvariantApplicability::LocalizedRepresentation => {
            activation.facts.configured_localized_representations > 0
                || activation.facts.configured_localized_roots > 0
        }
        InvariantApplicability::AuthorityMigration => {
            !checks.governance_graph.authority_migrations.is_empty()
        }
        InvariantApplicability::ContentAnchor => checks.semantic_content_anchors_checked > 0,
        InvariantApplicability::ScopedContentAnchor => checks
            .governance_graph
            .edges
            .iter()
            .filter_map(|edge| edge.content_anchor.as_ref())
            .any(|anchor| anchor.scope != crate::ContentAnchorScope::File),
        InvariantApplicability::Selector => checks
            .governance_graph
            .edges
            .iter()
            .any(|edge| edge.selector.is_some()),
        InvariantApplicability::GovernanceGraph => {
            activation.facts.configured_governance_manifests > 0
                || checks.governance_graph.metrics.nodes > 0
        }
        InvariantApplicability::TopologyPolicy => {
            activation.facts.configured_governance_manifests > 0
                || checks.governance_graph.metrics.nodes > 0
        }
    };

    if !applies {
        return invariant_evidence(
            invariant,
            InvariantOutcome::NotApplicable,
            Vec::new(),
            Vec::new(),
            "No applicability evidence for this invariant.".to_owned(),
        );
    }
    if disabled {
        return invariant_evidence(
            invariant,
            InvariantOutcome::Unverified,
            Vec::new(),
            Vec::new(),
            "A required checker is explicitly disabled and cannot prove maturity.".to_owned(),
        );
    }
    if !decisions.is_empty() && !active {
        return invariant_evidence(
            invariant,
            InvariantOutcome::Unverified,
            Vec::new(),
            Vec::new(),
            "An applicable checker is inactive and cannot prove maturity.".to_owned(),
        );
    }
    if invariant.delivery != InvariantDelivery::Delivered {
        return invariant_evidence(
            invariant,
            InvariantOutcome::Unverified,
            Vec::new(),
            Vec::new(),
            format!(
                "Invariant implementation is {}.",
                match invariant.delivery {
                    InvariantDelivery::Delivered => unreachable!(),
                    InvariantDelivery::Partial => "partial",
                    InvariantDelivery::Missing => "missing",
                }
            ),
        );
    }

    let visible = checks
        .diagnostics
        .iter()
        .filter(|diagnostic| invariant.diagnostic_codes.contains(&diagnostic.code))
        .collect::<Vec<_>>();
    if !visible.is_empty() {
        return invariant_evidence(
            invariant,
            InvariantOutcome::Failed,
            visible
                .iter()
                .map(|diagnostic| diagnostic.code.to_owned())
                .collect(),
            visible
                .iter()
                .map(|diagnostic| diagnostic.path.clone())
                .collect(),
            "One or more checker diagnostics violate this invariant.".to_owned(),
        );
    }
    let suppressed = checks
        .suppressed_diagnostics
        .iter()
        .filter(|diagnostic| {
            invariant
                .diagnostic_codes
                .contains(&diagnostic.code.as_str())
        })
        .collect::<Vec<_>>();
    if !suppressed.is_empty() {
        let mut evidence = invariant_evidence(
            invariant,
            InvariantOutcome::Unverified,
            suppressed
                .iter()
                .map(|diagnostic| diagnostic.code.clone())
                .collect(),
            suppressed
                .iter()
                .map(|diagnostic| diagnostic.path.clone())
                .collect(),
            "Legacy suppression hides a failure and therefore cannot prove maturity.".to_owned(),
        );
        evidence.suppression_reasons = suppressed
            .iter()
            .filter_map(|diagnostic| diagnostic.reason.clone())
            .collect();
        return evidence;
    }
    if invariant.id == "structure.reusable-templates" && !checks.document_templates.proves_reuse() {
        return invariant_evidence(
            invariant,
            InvariantOutcome::Unverified,
            Vec::new(),
            Vec::new(),
            format!(
                "Reusable template coverage is incomplete: {} templates, {}/{} profiles bound, {} unused templates.",
                checks.document_templates.configured_templates,
                checks
                    .document_templates
                    .bindings
                    .values()
                    .map(Vec::len)
                    .sum::<usize>(),
                checks.document_templates.configured_profiles,
                checks.document_templates.unused_templates.len()
            ),
        );
    }
    if invariant.id == "structure.template-migration"
        && !checks.document_templates.proves_migration()
    {
        return invariant_evidence(
            invariant,
            InvariantOutcome::Unverified,
            Vec::new(),
            Vec::new(),
            format!(
                "Template migration coverage is incomplete: {} unrevisioned templates, {} unpinned profiles, {} outdated profiles, {} incompatible profiles.",
                checks.document_templates.unrevisioned_templates.len(),
                checks.document_templates.unpinned_profiles.len(),
                checks.document_templates.outdated_profiles.len(),
                checks.document_templates.incompatible_profiles.len(),
            ),
        );
    }

    invariant_evidence(
        invariant,
        InvariantOutcome::Passed,
        Vec::new(),
        Vec::new(),
        "Applicable checker evidence passed.".to_owned(),
    )
}

fn invariant_evidence(
    invariant: &'static InvariantSpec,
    outcome: InvariantOutcome,
    diagnostic_codes: Vec<String>,
    paths: Vec<String>,
    reason: String,
) -> InvariantEvidence {
    InvariantEvidence {
        invariant: invariant.id,
        minimum_maturity: invariant.minimum_maturity,
        delivery: invariant.delivery,
        outcome,
        diagnostic_codes,
        paths,
        suppression_reasons: Vec::new(),
        reason,
    }
}

fn observed_maturity(evidence: &[InvariantEvidence]) -> Option<HygieneMaturity> {
    let mut observed = None;
    for level in HygieneMaturity::ALL {
        let required = evidence
            .iter()
            .filter(|item| {
                item.minimum_maturity <= level && item.outcome != InvariantOutcome::NotApplicable
            })
            .collect::<Vec<_>>();
        if required.is_empty()
            || required
                .iter()
                .any(|item| item.outcome != InvariantOutcome::Passed)
        {
            break;
        }
        observed = Some(level);
    }
    observed
}

impl CapabilityDimension {
    pub const ALL: [Self; 4] = [
        Self::Structure,
        Self::Identity,
        Self::Dependency,
        Self::Topology,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Structure => "structure",
            Self::Identity => "identity",
            Self::Dependency => "dependency",
            Self::Topology => "topology",
        }
    }
}

impl HygieneMaturity {
    pub const ALL: [Self; 3] = [Self::Basic, Self::Controlled, Self::Governed];

    pub fn label(self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Controlled => "controlled",
            Self::Governed => "governed",
        }
    }
}

#[cfg(test)]
mod tests;
