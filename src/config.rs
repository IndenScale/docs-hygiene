use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub required_files: Vec<PathBuf>,
    #[serde(default)]
    pub entry_docs: EntryDocsConfig,
    #[serde(default)]
    pub docs: DocsConfig,
    #[serde(default)]
    pub i18n: I18nConfig,
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
}

#[derive(Debug, Deserialize, Serialize)]
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
    pub bases: Vec<DocsBaseConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocsBaseConfig {
    pub id: String,
    pub root: PathBuf,
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
#[serde(rename_all = "camelCase")]
pub struct FilenamePatternConfig {
    pub id: String,
    pub regex: String,
    #[serde(default = "default_pattern_role")]
    pub role: String,
    #[serde(default)]
    pub numbered: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nConfig {
    #[serde(default)]
    pub root_lang: Option<String>,
    #[serde(default)]
    pub languages: Vec<String>,
    #[serde(default)]
    pub require_docs_parity: bool,
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

#[derive(Debug, Deserialize, Serialize)]
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

impl Default for ToolAdapterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            command: None,
            args: Vec::new(),
        }
    }
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
            bases: Vec::new(),
        }
    }
}

impl Default for EntryDocsConfig {
    fn default() -> Self {
        Self {
            required: Vec::new(),
            optional: Vec::new(),
        }
    }
}

impl Default for I18nConfig {
    fn default() -> Self {
        Self {
            root_lang: None,
            languages: Vec::new(),
            require_docs_parity: false,
            require_number_parity: false,
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
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          role: numbered
          numbered: true
        - id: index
          regex: "^INDEX\\.md$"
          role: index
          numbered: false

i18n:
  rootLang: en
  languages: [zh]
  requireDocsParity: true
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

fn default_pattern_role() -> String {
    "numbered".to_string()
}

fn default_concepts_dir() -> PathBuf {
    PathBuf::from("concept")
}
