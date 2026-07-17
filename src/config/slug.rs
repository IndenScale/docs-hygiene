use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SlugSchemaConfig {
    pub document_kind: String,
    pub source: SlugSourceConfig,
    pub pattern: String,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    #[serde(default)]
    pub reserved: Vec<String>,
    #[serde(default)]
    pub normalization: SlugNormalization,
    #[serde(default = "default_identity_field")]
    pub identity_field: String,
    #[serde(default = "default_aliases_field")]
    pub aliases_field: String,
    #[serde(default)]
    pub rename_policy: SlugRenamePolicy,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum SlugSourceConfig {
    Filename { capture: String },
    Frontmatter { field: String },
    StableId { field: String },
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SlugNormalization {
    None,
    Lowercase,
    #[default]
    LowercaseKebab,
}

impl SlugNormalization {
    pub(crate) fn normalize(self, value: &str) -> String {
        match self {
            Self::None => value.to_owned(),
            Self::Lowercase => value.to_lowercase(),
            Self::LowercaseKebab => {
                let mut output = String::new();
                let mut separator = false;
                for character in value.chars().flat_map(char::to_lowercase) {
                    if character.is_alphanumeric() {
                        if separator && !output.is_empty() {
                            output.push('-');
                        }
                        separator = false;
                        output.push(character);
                    } else {
                        separator = true;
                    }
                }
                output
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SlugRenamePolicy {
    #[default]
    StableIdentity,
    RequireAlias,
    AllowPathBreak,
}

fn default_identity_field() -> String {
    "id".to_owned()
}

fn default_aliases_field() -> String {
    "aliases".to_owned()
}
