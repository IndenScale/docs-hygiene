use anyhow::{Result, bail};

use crate::config::{Config, MaturityLevel, RuleMode};

use super::{ProjectFacts, RuleApplicability, RuleDecision, RuleSpec, RuleState, rule_spec};

pub(super) fn validate_rule_ids(config: &Config) -> Result<()> {
    for rule in config.rules.keys() {
        if rule_spec(rule).is_none() {
            bail!("unknown rule id '{rule}' in rules policy");
        }
    }
    Ok(())
}

pub(super) fn automatic_decision(
    spec: &RuleSpec,
    config: &Config,
    facts: &ProjectFacts,
) -> RuleDecision {
    let mode = config
        .rules
        .get(spec.id)
        .map(|policy| policy.mode)
        .unwrap_or(spec.default_mode);
    let (state, evidence) = infer_automatic_decision(spec, config, facts);
    match mode {
        RuleMode::Disabled => RuleDecision {
            rule: spec.id.to_owned(),
            mode,
            state: RuleState::Inactive,
            evidence: vec!["explicit mode disabled".to_owned()],
            rationale: spec.rationale.to_owned(),
            remediation: format!(
                "Set rules.{}.mode to auto or required to activate it.",
                spec.id
            ),
        },
        RuleMode::Required => RuleDecision {
            rule: spec.id.to_owned(),
            mode,
            state: RuleState::Error,
            evidence: vec!["explicit mode required".to_owned()],
            rationale: spec.rationale.to_owned(),
            remediation: spec.remediation.to_owned(),
        },
        RuleMode::Auto => RuleDecision {
            rule: spec.id.to_owned(),
            mode,
            state,
            evidence,
            rationale: spec.rationale.to_owned(),
            remediation: spec.remediation.to_owned(),
        },
    }
}

fn infer_automatic_decision(
    spec: &RuleSpec,
    config: &Config,
    facts: &ProjectFacts,
) -> (RuleState, Vec<String>) {
    match spec.applicability {
        RuleApplicability::EntryDocs if facts.configured_entry_docs > 0 => (
            RuleState::Error,
            vec![format!(
                "{} required entry documents configured",
                facts.configured_entry_docs
            )],
        ),
        RuleApplicability::EntryDocs => (
            RuleState::Inactive,
            vec!["no required entry documents".into()],
        ),
        RuleApplicability::DocumentStructure if facts.configured_docs_bases > 0 => (
            RuleState::Error,
            vec![format!(
                "{} documentation bases configured",
                facts.configured_docs_bases
            )],
        ),
        RuleApplicability::DocumentStructure
            if config.docs.require_continuous_numbering
                || config.docs.max_lines.is_some()
                || config.docs.forbid_ascii_art =>
        {
            (
                RuleState::Error,
                vec!["explicit document structure policy configured".into()],
            )
        }
        RuleApplicability::DocumentStructure
            if facts.markdown_documents >= 20 || facts.code_lines >= 20_000 =>
        {
            (
                RuleState::Advisory,
                vec![format!(
                    "scale signal: {} Markdown documents and {} code lines",
                    facts.markdown_documents, facts.code_lines
                )],
            )
        }
        RuleApplicability::DocumentStructure => (
            RuleState::Inactive,
            vec!["no structure or scale signal".into()],
        ),
        RuleApplicability::DocumentContracts
            if facts.configured_document_profiles > 0
                || facts.configured_document_templates > 0
                || facts.configured_document_kinds > 0 =>
        {
            let state = match config.document_contracts.maturity.declared {
                MaturityLevel::Seed | MaturityLevel::Growing => RuleState::Warning,
                MaturityLevel::Maintained | MaturityLevel::Governed => RuleState::Error,
            };
            (
                state,
                vec![format!(
                    "document contracts configured: {} profiles, {} templates, {} kinds; {:?} maturity",
                    facts.configured_document_profiles,
                    facts.configured_document_templates,
                    facts.configured_document_kinds,
                    config.document_contracts.maturity.declared
                )],
            )
        }
        RuleApplicability::DocumentContracts
            if !config
                .document_contracts
                .maturity
                .recommendations
                .is_empty() =>
        {
            (
                RuleState::Advisory,
                vec![format!(
                    "{} maturity recommendations configured",
                    config.document_contracts.maturity.recommendations.len()
                )],
            )
        }
        RuleApplicability::DocumentContracts
            if facts.markdown_documents >= 20 || facts.code_lines >= 20_000 =>
        {
            (
                RuleState::Advisory,
                vec![format!(
                    "scale signal: {} Markdown documents and {} code lines",
                    facts.markdown_documents, facts.code_lines
                )],
            )
        }
        RuleApplicability::DocumentContracts => (
            RuleState::Inactive,
            vec!["no profile, Kind, or scale signal".into()],
        ),
        RuleApplicability::Localization
            if facts.configured_localized_representations > 0
                || facts.configured_localized_roots > 0 =>
        {
            (
                RuleState::Error,
                vec![format!(
                    "{} localized representations and {} localized roots configured",
                    facts.configured_localized_representations, facts.configured_localized_roots
                )],
            )
        }
        RuleApplicability::Localization if !config.language.is_empty() => (
            RuleState::Error,
            vec![format!(
                "{} language thresholds configured",
                config.language.len()
            )],
        ),
        RuleApplicability::Localization if facts.localized_documents > 0 => (
            RuleState::Warning,
            vec![format!(
                "{} localized documents detected",
                facts.localized_documents
            )],
        ),
        RuleApplicability::Localization => (
            RuleState::Inactive,
            vec!["no localized representation detected".into()],
        ),
        RuleApplicability::Concepts
            if config.concepts.require_concept_file
                || config.concepts.fail_on_orphan_concept.is_some() =>
        {
            (
                RuleState::Error,
                vec!["explicit concept policy configured".into()],
            )
        }
        RuleApplicability::Concepts if facts.concept_documents > 0 => (
            RuleState::Warning,
            vec![format!(
                "{} concept documents detected",
                facts.concept_documents
            )],
        ),
        RuleApplicability::Concepts => (
            RuleState::Inactive,
            vec!["no concept policy or documents".into()],
        ),
        RuleApplicability::GovernanceIdentity if facts.configured_governance_manifests > 0 => (
            RuleState::Error,
            vec![format!(
                "{} governance Manifests configured",
                facts.configured_governance_manifests
            )],
        ),
        RuleApplicability::GovernanceIdentity
            if facts.manifest_files > 0
                || facts.frontmatter_documents > 0
                || facts.semantic_wiki_links > 0 =>
        {
            (
                RuleState::Warning,
                vec![format!(
                    "structural signals: {} Manifests, {} frontmatter documents, {} Wiki Links",
                    facts.manifest_files, facts.frontmatter_documents, facts.semantic_wiki_links
                )],
            )
        }
        RuleApplicability::GovernanceIdentity => (
            RuleState::Inactive,
            vec!["no governance identity signal".into()],
        ),
        RuleApplicability::GovernanceTraceability
            if facts.configured_critical_dependencies > 0
                || facts.configured_portable_snapshots > 0 =>
        {
            (
                RuleState::Error,
                vec![format!(
                    "{} critical dependency policies and {} portable snapshots configured",
                    facts.configured_critical_dependencies, facts.configured_portable_snapshots
                )],
            )
        }
        RuleApplicability::GovernanceTraceability
            if facts.configured_refinement_levels.len() >= 2 =>
        {
            (
                RuleState::Error,
                vec![format!(
                    "configured assets span {} refinement levels: {}",
                    facts.configured_refinement_levels.len(),
                    facts.configured_refinement_levels.join(", ")
                )],
            )
        }
        RuleApplicability::GovernanceTraceability
            if facts.detected_refinement_levels.len() >= 2 =>
        {
            (
                RuleState::Warning,
                vec![format!(
                    "detected Manifests span {} refinement levels: {}",
                    facts.detected_refinement_levels.len(),
                    facts.detected_refinement_levels.join(", ")
                )],
            )
        }
        RuleApplicability::GovernanceTraceability => (
            RuleState::Inactive,
            vec!["fewer than two refinement levels detected".into()],
        ),
        RuleApplicability::GovernanceTopology if facts.configured_topology_policies > 0 => (
            RuleState::Error,
            vec![format!(
                "{} explicit topology policies configured",
                facts.configured_topology_policies
            )],
        ),
        RuleApplicability::GovernanceTopology => (
            RuleState::Inactive,
            vec!["no explicit topology policy configured".into()],
        ),
        RuleApplicability::ExternalAdapters if facts.enabled_adapters > 0 => (
            RuleState::Error,
            vec![format!(
                "{} external Adapters enabled",
                facts.enabled_adapters
            )],
        ),
        RuleApplicability::ExternalAdapters => (
            RuleState::Inactive,
            vec!["no external Adapter enabled".into()],
        ),
    }
}
