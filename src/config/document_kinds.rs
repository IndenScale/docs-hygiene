use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentKindConfig {
    pub id: String,
    pub base: String,
    pub pattern: String,
    pub profile: String,
    pub scaffold: KindScaffoldConfig,
    pub frontmatter: FrontmatterSchemaConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct KindScaffoldConfig {
    pub filename: String,
    #[serde(default = "default_title_template")]
    pub title: String,
    #[serde(default)]
    pub section_headings: BTreeMap<String, BTreeMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FrontmatterSchemaConfig {
    pub revision: u64,
    pub compatible_from: Option<u64>,
    #[serde(default = "default_revision_field")]
    pub revision_field: String,
    #[serde(default = "default_allow_unknown_fields")]
    pub allow_unknown_fields: bool,
    #[serde(default)]
    pub fields: Vec<FrontmatterFieldConfig>,
    #[serde(default)]
    pub conditions: Vec<FrontmatterConditionConfig>,
    #[serde(default)]
    pub invariants: Vec<FrontmatterInvariantConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FrontmatterFieldConfig {
    pub id: String,
    #[serde(rename = "type")]
    pub field_type: FrontmatterFieldType,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub source: FrontmatterFieldSource,
    #[serde(default)]
    pub values: Vec<serde_yaml::Value>,
    pub format: Option<String>,
    pub default: Option<serde_yaml::Value>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FrontmatterFieldType {
    String,
    Integer,
    Number,
    Boolean,
    StringList,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FrontmatterFieldSource {
    #[default]
    Input,
    Identity,
    Slug,
    Locale,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FrontmatterConditionConfig {
    pub when: FrontmatterPredicateConfig,
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(default)]
    pub forbidden: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FrontmatterPredicateConfig {
    pub field: String,
    pub equals: serde_yaml::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FrontmatterInvariantConfig {
    pub left: String,
    pub operator: FrontmatterInvariantOperator,
    pub right: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FrontmatterInvariantOperator {
    Equals,
    NotEquals,
}

fn default_title_template() -> String {
    "{identity}".to_owned()
}

fn default_revision_field() -> String {
    "schemaRevision".to_owned()
}

fn default_allow_unknown_fields() -> bool {
    true
}
