use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentContractsConfig {
    #[serde(default)]
    pub maturity: MaturityConfig,
    #[serde(default)]
    pub templates: Vec<DocumentTemplateConfig>,
    #[serde(default)]
    pub profiles: Vec<DocumentProfileConfig>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MaturityLevel {
    #[default]
    Seed,
    Growing,
    Maintained,
    Governed,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaturityConfig {
    #[serde(default)]
    pub declared: MaturityLevel,
    #[serde(default)]
    pub recommendations: Vec<MaturityRecommendationConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MaturityRecommendationConfig {
    pub level: MaturityLevel,
    pub min_project_lines: Option<usize>,
    pub min_project_bytes: Option<u64>,
    pub min_managed_documents: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentTemplateConfig {
    pub id: String,
    pub revision: Option<u64>,
    pub compatible_from: Option<u64>,
    #[serde(flatten)]
    pub contract: DocumentContractFragmentConfig,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentProfileConfig {
    pub id: String,
    pub template: Option<String>,
    pub template_revision: Option<u64>,
    #[serde(rename = "match")]
    pub matcher: DocumentMatchConfig,
    #[serde(flatten)]
    pub contract: DocumentContractFragmentConfig,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentContractFragmentConfig {
    #[serde(default)]
    pub required_sections: Vec<RequiredSectionConfig>,
    #[serde(default)]
    pub required_fields: Vec<RequiredFieldConfig>,
    pub ordered_sections: Option<bool>,
    pub enforce_from: Option<MaturityLevel>,
    pub placeholders_allowed_until: Option<MaturityLevel>,
    #[serde(default)]
    pub placeholder_patterns: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentMatchConfig {
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    pub filenames: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequiredSectionConfig {
    pub id: String,
    pub headings: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequiredFieldConfig {
    pub id: String,
    pub pattern: String,
}

impl DocumentContractFragmentConfig {
    pub fn resolved_enforce_from(&self) -> MaturityLevel {
        self.enforce_from.unwrap_or(MaturityLevel::Maintained)
    }

    pub fn resolved_placeholders_allowed_until(&self) -> MaturityLevel {
        self.placeholders_allowed_until
            .unwrap_or(MaturityLevel::Growing)
    }
}
