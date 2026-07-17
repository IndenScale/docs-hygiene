use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::governance::ReferenceRelation;

#[derive(Debug, Deserialize, Serialize)]
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
    pub portable_snapshots: PortableSnapshotConfig,
    #[serde(default)]
    pub core_claims: Vec<CoreClaimConfig>,
    #[serde(default)]
    pub critical_dependencies: Vec<CriticalDependencyPolicyConfig>,
    #[serde(default = "default_pin_audit_log")]
    pub pin_audit_log: PathBuf,
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            manifests: Vec::new(),
            require_complete_vertical_derivation: false,
            topology: GovernanceTopologyConfig::default(),
            content_anchors: GovernanceContentAnchorConfig::default(),
            portable_snapshots: PortableSnapshotConfig::default(),
            core_claims: Vec::new(),
            critical_dependencies: Vec::new(),
            pin_audit_log: default_pin_audit_log(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PortableSnapshotConfig {
    #[serde(default)]
    pub manifests: Vec<PathBuf>,
    #[serde(default)]
    pub trusted_keys: BTreeMap<String, String>,
    #[serde(default)]
    pub require_signatures: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CriticalDependencyPolicyConfig {
    pub id: String,
    #[serde(rename = "match")]
    pub matcher: CriticalDependencyMatcherConfig,
    pub require: CriticalPinRequirementConfig,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CriticalDependencyMatcherConfig {
    #[serde(default)]
    pub source_kinds: Vec<ReferenceRelation>,
    #[serde(default)]
    pub target_kinds: Vec<ReferenceRelation>,
    #[serde(default)]
    pub relations: Vec<CriticalDependencyRelation>,
    #[serde(default)]
    pub source_paths: Vec<String>,
    #[serde(default)]
    pub target_paths: Vec<String>,
    #[serde(default)]
    pub source_ids: Vec<String>,
    #[serde(default)]
    pub target_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CriticalDependencyRelation {
    References,
    Formalizes,
    Realizes,
    Projects,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CriticalPinRequirementConfig {
    #[serde(default = "default_pin_algorithms")]
    pub algorithms: Vec<String>,
    #[serde(default)]
    pub minimum_scope: CriticalPinScope,
    #[serde(default)]
    pub forbid_whole_file: bool,
    pub max_age_days: Option<u64>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CriticalPinScope {
    #[default]
    File,
    Commit,
    Block,
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
    #[serde(default)]
    pub exceptions: Vec<SupernodeExceptionConfig>,
}

impl GovernanceTopologyConfig {
    pub fn configured_policy_count(&self) -> usize {
        usize::from(self.max_fan_in.is_some())
            + usize::from(self.max_fan_out.is_some())
            + usize::from(self.forbid_cycles)
            + self.exceptions.len()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SupernodeExceptionConfig {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub node: String,
    #[serde(default)]
    pub direction: Option<TopologyDirection>,
    #[serde(default)]
    pub budget: usize,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub owner: String,
    #[serde(default)]
    pub approved_by: String,
    #[serde(default)]
    pub expires: String,
    #[serde(default)]
    pub history: Vec<SupernodeDegreeObservationConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SupernodeDegreeObservationConfig {
    #[serde(default)]
    pub observed_at: String,
    pub degree: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TopologyDirection {
    FanIn,
    FanOut,
}

fn default_similarity_threshold() -> f64 {
    0.72
}

fn default_pin_algorithms() -> Vec<String> {
    vec!["sha256".to_owned()]
}

fn default_pin_audit_log() -> PathBuf {
    PathBuf::from(".docs-hygiene/pin-updates.jsonl")
}
