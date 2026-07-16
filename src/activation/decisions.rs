use anyhow::{Result, bail};

use crate::config::{Config, MaturityLevel, RuleMode};

use super::{ProjectFacts, RULE_IDS, RuleDecision, RuleState};

pub(super) fn validate_rule_ids(config: &Config) -> Result<()> {
    for rule in config.rules.keys() {
        if !RULE_IDS.contains(&rule.as_str()) {
            bail!("unknown rule id '{rule}' in rules policy");
        }
    }
    Ok(())
}

pub(super) fn automatic_decision(
    rule: &str,
    config: &Config,
    facts: &ProjectFacts,
) -> RuleDecision {
    let mode = config
        .rules
        .get(rule)
        .map(|policy| policy.mode)
        .unwrap_or_default();
    let (state, evidence, remediation) = infer_automatic_decision(rule, config, facts);
    match mode {
        RuleMode::Disabled => RuleDecision {
            rule: rule.to_owned(),
            mode,
            state: RuleState::Inactive,
            evidence: vec!["explicit mode disabled".to_owned()],
            rationale: rule_rationale(rule).to_owned(),
            remediation: format!("Set rules.{rule}.mode to auto or required to activate it."),
        },
        RuleMode::Required => RuleDecision {
            rule: rule.to_owned(),
            mode,
            state: RuleState::Error,
            evidence: vec!["explicit mode required".to_owned()],
            rationale: rule_rationale(rule).to_owned(),
            remediation,
        },
        RuleMode::Auto => RuleDecision {
            rule: rule.to_owned(),
            mode,
            state,
            evidence,
            rationale: rule_rationale(rule).to_owned(),
            remediation,
        },
    }
}

fn rule_rationale(rule: &str) -> &'static str {
    match rule {
        "project.entry-docs" => "Stable entry documents keep project intent discoverable.",
        "docs.structure" => {
            "Predictable structure keeps navigation and document growth reviewable."
        }
        "documents.contracts" => {
            "Semantic sections make each document type complete enough for its role."
        }
        "localization.parity" => {
            "Language representations must preserve one authoritative project identity."
        }
        "concepts.references" => {
            "Stable concept targets prevent highlighted terms from becoming implicit local jargon."
        }
        "governance.identity" => {
            "Stable identities and references make project knowledge resolvable."
        }
        "governance.traceability" => {
            "Typed adjacent-level edges show whether intent reaches definition and implementation."
        }
        "adapters.external" => "External surface checks remain visible in the same governance run.",
        _ => unreachable!("validated built-in rule id"),
    }
}

fn infer_automatic_decision(
    rule: &str,
    config: &Config,
    facts: &ProjectFacts,
) -> (RuleState, Vec<String>, String) {
    let result = match rule {
        "project.entry-docs" if facts.configured_entry_docs > 0 => (
            RuleState::Error,
            vec![format!(
                "{} required entry documents configured",
                facts.configured_entry_docs
            )],
        ),
        "project.entry-docs" => (
            RuleState::Inactive,
            vec!["no required entry documents".into()],
        ),
        "docs.structure" if facts.configured_docs_bases > 0 => (
            RuleState::Error,
            vec![format!(
                "{} documentation bases configured",
                facts.configured_docs_bases
            )],
        ),
        "docs.structure"
            if config.docs.require_continuous_numbering
                || config.docs.max_lines.is_some()
                || config.docs.forbid_ascii_art =>
        {
            (
                RuleState::Error,
                vec!["explicit document structure policy configured".into()],
            )
        }
        "docs.structure" if facts.markdown_documents >= 20 || facts.code_lines >= 20_000 => (
            RuleState::Advisory,
            vec![format!(
                "scale signal: {} Markdown documents and {} code lines",
                facts.markdown_documents, facts.code_lines
            )],
        ),
        "docs.structure" => (
            RuleState::Inactive,
            vec!["no structure or scale signal".into()],
        ),
        "documents.contracts" if facts.configured_document_profiles > 0 => {
            let state = match config.document_contracts.maturity.declared {
                MaturityLevel::Seed | MaturityLevel::Growing => RuleState::Warning,
                MaturityLevel::Maintained | MaturityLevel::Governed => RuleState::Error,
            };
            (
                state,
                vec![format!(
                    "{} document profiles configured at {:?} maturity",
                    facts.configured_document_profiles, config.document_contracts.maturity.declared
                )],
            )
        }
        "documents.contracts"
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
        "documents.contracts" if facts.markdown_documents >= 20 || facts.code_lines >= 20_000 => (
            RuleState::Advisory,
            vec![format!(
                "scale signal: {} Markdown documents and {} code lines",
                facts.markdown_documents, facts.code_lines
            )],
        ),
        "documents.contracts" => (
            RuleState::Inactive,
            vec!["no profile or scale signal".into()],
        ),
        "localization.parity"
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
        "localization.parity" if !config.language.is_empty() => (
            RuleState::Error,
            vec![format!(
                "{} language thresholds configured",
                config.language.len()
            )],
        ),
        "localization.parity" if facts.localized_documents > 0 => (
            RuleState::Warning,
            vec![format!(
                "{} localized documents detected",
                facts.localized_documents
            )],
        ),
        "localization.parity" => (
            RuleState::Inactive,
            vec!["no localized representation detected".into()],
        ),
        "concepts.references"
            if config.concepts.require_concept_file
                || config.concepts.fail_on_orphan_concept.is_some() =>
        {
            (
                RuleState::Error,
                vec!["explicit concept policy configured".into()],
            )
        }
        "concepts.references" if facts.concept_documents > 0 => (
            RuleState::Warning,
            vec![format!(
                "{} concept documents detected",
                facts.concept_documents
            )],
        ),
        "concepts.references" => (
            RuleState::Inactive,
            vec!["no concept policy or documents".into()],
        ),
        "governance.identity" if facts.configured_governance_manifests > 0 => (
            RuleState::Error,
            vec![format!(
                "{} governance Manifests configured",
                facts.configured_governance_manifests
            )],
        ),
        "governance.identity"
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
        "governance.identity" => (
            RuleState::Inactive,
            vec!["no governance identity signal".into()],
        ),
        "governance.traceability" if facts.configured_refinement_levels.len() >= 2 => (
            RuleState::Error,
            vec![format!(
                "configured assets span {} refinement levels: {}",
                facts.configured_refinement_levels.len(),
                facts.configured_refinement_levels.join(", ")
            )],
        ),
        "governance.traceability" if facts.detected_refinement_levels.len() >= 2 => (
            RuleState::Warning,
            vec![format!(
                "detected Manifests span {} refinement levels: {}",
                facts.detected_refinement_levels.len(),
                facts.detected_refinement_levels.join(", ")
            )],
        ),
        "governance.traceability" => (
            RuleState::Inactive,
            vec!["fewer than two refinement levels detected".into()],
        ),
        "adapters.external" if facts.enabled_adapters > 0 => (
            RuleState::Error,
            vec![format!(
                "{} external Adapters enabled",
                facts.enabled_adapters
            )],
        ),
        "adapters.external" => (
            RuleState::Inactive,
            vec!["no external Adapter enabled".into()],
        ),
        _ => unreachable!("validated built-in rule id"),
    };
    (
        result.0,
        result.1,
        format!("Configure rules.{rule}.mode as auto, required, or disabled."),
    )
}
