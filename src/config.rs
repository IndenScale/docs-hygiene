use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::activation::HygieneMaturity;

mod document_contracts;
mod document_kinds;
mod governance;
mod slug;

pub use document_contracts::{
    DocumentContractFragmentConfig, DocumentContractsConfig, DocumentMatchConfig,
    DocumentProfileConfig, DocumentTemplateConfig, MaturityConfig, MaturityLevel,
    MaturityRecommendationConfig, RequiredFieldConfig, RequiredSectionConfig,
};
pub use document_kinds::{
    DocumentKindConfig, FrontmatterConditionConfig, FrontmatterFieldConfig, FrontmatterFieldSource,
    FrontmatterFieldType, FrontmatterInvariantConfig, FrontmatterInvariantOperator,
    FrontmatterPredicateConfig, FrontmatterSchemaConfig, KindScaffoldConfig,
};
pub use governance::{
    CoreClaimAuthorityConfig, CoreClaimConfig, CoreClaimOccurrenceConfig,
    CoreClaimOccurrencePolicy, GovernanceConfig, GovernanceContentAnchorConfig,
    GovernanceTopologyConfig,
};
pub use slug::{SlugNormalization, SlugRenamePolicy, SlugSchemaConfig, SlugSourceConfig};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub required_files: Vec<PathBuf>,
    #[serde(default)]
    pub entry_docs: EntryDocsConfig,
    #[serde(default)]
    pub docs: DocsConfig,
    #[serde(default)]
    pub language_representations: LanguageRepresentationsConfig,
    #[serde(default)]
    pub concepts: ConceptsConfig,
    #[serde(default)]
    pub language: BTreeMap<String, LanguageConfig>,
    #[serde(default)]
    pub adapters: AdaptersConfig,
    #[serde(default)]
    pub suppressions: Vec<SuppressionConfig>,
    #[serde(default)]
    pub ignore: IgnoreConfig,
    #[serde(default)]
    pub document_contracts: DocumentContractsConfig,
    #[serde(default)]
    pub document_kinds: Vec<DocumentKindConfig>,
    #[serde(default)]
    pub governance: GovernanceConfig,
    #[serde(default)]
    pub rules: BTreeMap<String, RulePolicyConfig>,
    #[serde(default)]
    pub hygiene_profile: HygieneProfileConfig,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DimensionApplicability {
    #[default]
    Applicable,
    NotApplicable,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HygieneProfileConfig {
    #[serde(default)]
    pub dimensions: HygieneProfileDimensionsConfig,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HygieneProfileDimensionsConfig {
    pub structure: Option<DimensionProfileConfig>,
    pub identity: Option<DimensionProfileConfig>,
    pub dependency: Option<DimensionProfileConfig>,
    pub topology: Option<DimensionProfileConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DimensionProfileConfig {
    #[serde(default)]
    pub applicability: DimensionApplicability,
    pub target: Option<HygieneMaturity>,
    #[serde(default = "default_required_dimension")]
    pub required: bool,
    pub rationale: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleMode {
    #[default]
    Auto,
    Required,
    Disabled,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RulePolicyConfig {
    #[serde(default)]
    pub mode: RuleMode,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDocsConfig {
    #[serde(default)]
    pub required: Vec<PathBuf>,
    #[serde(default)]
    pub optional: Vec<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocsConfig {
    #[serde(default = "default_docs_root")]
    pub root: PathBuf,
    #[serde(default = "default_filename_pattern")]
    pub filename_pattern: String,
    #[serde(default)]
    pub require_continuous_numbering: bool,
    pub max_lines: Option<usize>,
    #[serde(default)]
    pub forbid_ascii_art: bool,
    #[serde(default)]
    pub bases: Vec<DocsBaseConfig>,
    #[serde(default)]
    pub slug_schemas: Vec<SlugSchemaConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocsBaseConfig {
    pub id: String,
    pub root: PathBuf,
    #[serde(default)]
    pub localized_roots: BTreeMap<String, PathBuf>,
    #[serde(default)]
    pub patterns: Vec<FilenamePatternConfig>,
    #[serde(default)]
    pub require_continuous_numbering: Option<bool>,
    #[serde(default)]
    pub max_lines: Option<usize>,
    #[serde(default)]
    pub ignore: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FilenamePatternConfig {
    pub id: String,
    pub regex: String,
    #[serde(default = "default_document_kind")]
    pub document_kind: String,
    #[serde(default)]
    pub numbered: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LanguageRepresentationsConfig {
    #[serde(default)]
    pub canonical: Option<String>,
    #[serde(default)]
    pub localized: Vec<String>,
    #[serde(default)]
    pub require_document_parity: bool,
    #[serde(default)]
    pub require_number_parity: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptsConfig {
    #[serde(default = "default_concepts_dir")]
    pub dir: PathBuf,
    #[serde(default)]
    pub require_concept_file: bool,
    #[serde(default)]
    pub fail_on_orphan_concept: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_cjk_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cjk_ratio: Option<f64>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdaptersConfig {
    #[serde(default)]
    pub markdownlint: ToolAdapterConfig,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolAdapterConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuppressionConfig {
    pub code: String,
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IgnoreConfig {
    #[serde(default)]
    pub paths: Vec<String>,
}

impl Default for DocsConfig {
    fn default() -> Self {
        Self {
            root: default_docs_root(),
            filename_pattern: default_filename_pattern(),
            require_continuous_numbering: false,
            max_lines: None,
            forbid_ascii_art: false,
            bases: Vec::new(),
            slug_schemas: Vec::new(),
        }
    }
}

impl Default for ConceptsConfig {
    fn default() -> Self {
        Self {
            dir: default_concepts_dir(),
            require_concept_file: false,
            fail_on_orphan_concept: None,
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        serde_yaml::from_str(&content)
            .with_context(|| format!("failed to parse {}", path.display()))
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_yaml::to_string(self)
            .with_context(|| format!("failed to serialize {}", path.display()))?;
        std::fs::write(path, content).with_context(|| format!("failed to write {}", path.display()))
    }

    pub fn starter_yaml() -> &'static str {
        r#"entryDocs:
  required:
    - README.md
    - README_ZH.md
    - CHANGELOG.md
    - LICENSE
  optional:
    - AGENTS.md
    - CLAUDE.md
    - GEMINI.md
    - CONTRIBUTING.md
    - SECURITY.md

docs:
  bases:
    - id: main
      root: docs
      requireContinuousNumbering: true
      maxLines: 500
      forbidAsciiArt: true
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          documentKind: numbered
          numbered: true
        - id: index
          regex: "^INDEX\\.md$"
          documentKind: index
          numbered: false

languageRepresentations:
  canonical: en
  localized: [zh]
  requireDocumentParity: true
  requireNumberParity: true

concepts:
  dir: concept
  requireConceptFile: true
  failOnOrphanConcept: warn

language:
  en:
    maxCjkRatio: 0.05
  zh:
    minCjkRatio: 0.15

rules:
  project.entry-docs:
    mode: auto
  docs.structure:
    mode: auto
  documents.contracts:
    mode: auto
  localization.parity:
    mode: auto
  concepts.references:
    mode: auto
  governance.identity:
    mode: auto
  governance.traceability:
    mode: auto
  governance.topology:
    mode: auto
  adapters.external:
    mode: auto

hygieneProfile:
  dimensions:
    structure:
      target: controlled
      required: true
    identity:
      target: controlled
      required: true
    dependency:
      applicability: notApplicable
      rationale: No semantic dependency graph is governed yet.
    topology:
      applicability: notApplicable
      rationale: No semantic dependency graph is governed yet.

documentContracts:
  maturity:
    declared: growing
    recommendations:
      - level: maintained
        minProjectLines: 10000
        minManagedDocuments: 20
  templates:
    - id: maintained-open-contract
      revision: 1
      compatibleFrom: 1
      enforceFrom: maintained
      placeholdersAllowedUntil: growing
      placeholderPatterns: ["(?i)\\b(?:TODO|TBD)\\b", "待补充"]
      orderedSections: true
  profiles:
    - id: project-readme
      template: maintained-open-contract
      templateRevision: 1
      match:
        paths: [README.md, README_ZH.md]
        filenames: ["^README(?:_ZH)?\\.md$"]
      requiredSections:
        - id: overview
          headings: [Overview, 概览]
        - id: quick-start
          headings: [Quick Start, 快速开始]

adapters:
  markdownlint:
    enabled: true
    command: markdownlint-cli2
    args:
      - README.md
      - README_ZH.md
      - CHANGELOG.md
      - "docs/**/*.md"

suppressions:
  - code: DH_LANG_002
    paths:
      - docs/fixtures/**
    reason: Fixtures may intentionally contain mixed-language examples.

ignore:
  paths:
    - target/**
"#
    }
}

fn default_docs_root() -> PathBuf {
    PathBuf::from("docs")
}

fn default_filename_pattern() -> String {
    r"^\d{2}_[a-z0-9_-]+\.md$".to_string()
}

fn default_document_kind() -> String {
    "numbered".to_string()
}

fn default_concepts_dir() -> PathBuf {
    PathBuf::from("concept")
}

fn default_required_dimension() -> bool {
    true
}
