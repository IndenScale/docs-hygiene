use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceConfig {
    #[serde(default)]
    pub manifests: Vec<PathBuf>,
    #[serde(default)]
    pub require_complete_vertical_derivation: bool,
    #[serde(default)]
    pub topology: GovernanceTopologyConfig,
    #[serde(default)]
    pub content_anchors: GovernanceContentAnchorConfig,
    #[serde(default)]
    pub core_claims: Vec<CoreClaimConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CoreClaimConfig {
    pub id: String,
    pub authority: CoreClaimAuthorityConfig,
    #[serde(default)]
    pub candidate_paths: Vec<String>,
    #[serde(default = "default_similarity_threshold")]
    pub similarity_threshold: f64,
    #[serde(default)]
    pub occurrences: Vec<CoreClaimOccurrenceConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CoreClaimAuthorityConfig {
    pub id: String,
    pub selector: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CoreClaimOccurrenceConfig {
    pub path: PathBuf,
    pub selector: String,
    pub policy: CoreClaimOccurrencePolicy,
    pub migrate_by: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CoreClaimOccurrencePolicy {
    Forbidden,
    Migrate,
    ControlledExcerpt,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GovernanceContentAnchorConfig {
    #[serde(default)]
    pub verify_git_commits: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GovernanceTopologyConfig {
    pub max_fan_in: Option<usize>,
    pub max_fan_out: Option<usize>,
    #[serde(default)]
    pub forbid_cycles: bool,
}

impl GovernanceTopologyConfig {
    pub fn configured_policy_count(&self) -> usize {
        usize::from(self.max_fan_in.is_some())
            + usize::from(self.max_fan_out.is_some())
            + usize::from(self.forbid_cycles)
    }
}

fn default_similarity_threshold() -> f64 {
    0.72
}
