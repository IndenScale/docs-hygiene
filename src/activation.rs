use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use anyhow::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::Serialize;
use walkdir::WalkDir;

use crate::config::{Config, RuleMode};

mod decisions;
mod output;

use decisions::{automatic_decision, validate_rule_ids};
pub use output::{print_json_activation, print_text_activation};

// Governance Library: [[GLOSSARY-RULE-ACTIVATION-DECISION]]

pub const RULE_IDS: [&str; 9] = [
    "project.entry-docs",
    "docs.structure",
    "documents.contracts",
    "localization.parity",
    "concepts.references",
    "governance.identity",
    "governance.domain-fanout",
    "governance.traceability",
    "adapters.external",
];

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleState {
    Inactive,
    Advisory,
    Warning,
    Error,
}

impl RuleState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Advisory => "advisory",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectFacts {
    pub markdown_documents: usize,
    pub markdown_lines: usize,
    pub code_lines: usize,
    pub localized_documents: usize,
    pub concept_documents: usize,
    pub manifest_files: usize,
    pub frontmatter_documents: usize,
    pub semantic_wiki_links: usize,
    pub configured_docs_bases: usize,
    pub configured_document_profiles: usize,
    pub configured_entry_docs: usize,
    pub configured_localized_representations: usize,
    pub configured_localized_roots: usize,
    pub configured_governance_manifests: usize,
    pub configured_refinement_levels: Vec<String>,
    pub detected_refinement_levels: Vec<String>,
    pub enabled_adapters: usize,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleDecision {
    pub rule: String,
    pub mode: RuleMode,
    pub state: RuleState,
    pub evidence: Vec<String>,
    pub rationale: String,
    pub remediation: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivationReport {
    pub schema_version: &'static str,
    pub facts: ProjectFacts,
    pub decisions: Vec<RuleDecision>,
}

impl ActivationReport {
    pub fn decision(&self, rule: &str) -> &RuleDecision {
        self.decisions
            .iter()
            .find(|decision| decision.rule == rule)
            .expect("built-in rule decision must exist")
    }
}

pub fn evaluate_rule_activation(root: &Path, config: &Config) -> Result<ActivationReport> {
    config.validate()?;
    validate_rule_ids(config)?;
    let facts = collect_project_facts(root, config)?;
    let decisions = RULE_IDS
        .iter()
        .map(|rule| automatic_decision(rule, config, &facts))
        .collect();
    Ok(ActivationReport {
        schema_version: "docs-hygiene.rule-activation.v1",
        facts,
        decisions,
    })
}

fn collect_project_facts(root: &Path, config: &Config) -> Result<ProjectFacts> {
    let ignore = build_ignore(&config.ignore.paths)?;
    let configured_localized_roots = config
        .docs
        .bases
        .iter()
        .flat_map(|base| base.localized_roots.values())
        .cloned()
        .collect::<Vec<_>>();
    let mut files = Vec::new();
    for entry in WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            if entry.depth() == 0 {
                return true;
            }
            let rel = entry.path().strip_prefix(root).unwrap_or(entry.path());
            entry.file_name() != ".git" && !ignore.is_match(rel)
        })
    {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        files.push(entry.path().strip_prefix(root)?.to_path_buf());
    }
    files.sort();

    let mut facts = ProjectFacts {
        configured_docs_bases: config.docs.bases.len(),
        configured_document_profiles: config.document_contracts.profiles.len(),
        configured_entry_docs: config.entry_docs.required.len() + config.required_files.len(),
        configured_localized_representations: config.language_representations.localized.len(),
        configured_localized_roots: configured_localized_roots.len(),
        configured_governance_manifests: config.governance.manifests.len(),
        enabled_adapters: usize::from(config.adapters.markdownlint.enabled),
        ..ProjectFacts::default()
    };
    let mut detected_levels = BTreeSet::new();
    for rel in &files {
        let extension = rel
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        let text = std::fs::read_to_string(root.join(rel)).unwrap_or_default();
        if extension == "md" {
            facts.markdown_documents += 1;
            facts.markdown_lines += text.lines().count();
            facts.semantic_wiki_links += text.matches("[[").count();
            facts.frontmatter_documents += usize::from(text.starts_with("---\n"));
            facts.localized_documents +=
                usize::from(is_localized_path(rel, &configured_localized_roots));
            facts.concept_documents += usize::from(rel.starts_with(&config.concepts.dir));
        } else if is_code_extension(extension) {
            facts.code_lines += text.lines().count();
        }
        if is_manifest_path(rel) {
            facts.manifest_files += 1;
            if let Some(level) = refinement_level(&text) {
                detected_levels.insert(level);
            }
        }
    }
    facts.detected_refinement_levels = detected_levels.into_iter().collect();
    facts.configured_refinement_levels = configured_refinement_levels(root, config);
    Ok(facts)
}

fn build_ignore(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(Glob::new(pattern)?);
    }
    Ok(builder.build()?)
}

fn configured_refinement_levels(root: &Path, config: &Config) -> Vec<String> {
    let mut levels = BTreeSet::new();
    for rel in &config.governance.manifests {
        let text = std::fs::read_to_string(root.join(rel)).unwrap_or_default();
        if let Some(level) = refinement_level(&text) {
            levels.insert(level);
        }
    }
    levels.into_iter().collect()
}

fn refinement_level(text: &str) -> Option<String> {
    let value = serde_yaml::from_str::<serde_yaml::Value>(text).ok()?;
    value
        .get("refinementLevel")
        .and_then(serde_yaml::Value::as_str)
        .map(str::to_owned)
}

fn is_manifest_path(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    matches!(name, "manifest.yml" | "manifest.yaml")
        || name.ends_with("-manifest.yml")
        || name.ends_with("-manifest.yaml")
}

fn is_localized_path(path: &Path, configured_roots: &[PathBuf]) -> bool {
    configured_roots.iter().any(|root| path.starts_with(root))
        || path
            .components()
            .any(|component| component.as_os_str() == "zh")
}

fn is_code_extension(extension: &str) -> bool {
    matches!(
        extension,
        "c" | "cc"
            | "cpp"
            | "cs"
            | "go"
            | "h"
            | "hpp"
            | "java"
            | "js"
            | "jsx"
            | "kt"
            | "kts"
            | "php"
            | "py"
            | "rb"
            | "rs"
            | "swift"
            | "ts"
            | "tsx"
    )
}

#[cfg(test)]
mod tests;
