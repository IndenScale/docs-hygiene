use serde::Serialize;

use crate::activation::{CapabilityDimension, HygieneMaturity, ProjectFacts, RuleDecision};
use crate::governance::GovernanceGraph;
use crate::report::{DocumentTemplateReport, OwnershipReport, TopologyExceptionEvidence};

use super::InvariantDelivery;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DimensionStatus {
    MeetsTarget,
    BelowTarget,
    Observed,
    Unverified,
    NotApplicable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InvariantOutcome {
    Passed,
    Excepted,
    Failed,
    Unverified,
    NotApplicable,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvariantEvidence {
    pub invariant: &'static str,
    pub minimum_maturity: HygieneMaturity,
    pub delivery: InvariantDelivery,
    pub outcome: InvariantOutcome,
    pub diagnostic_codes: Vec<String>,
    pub paths: Vec<String>,
    pub suppression_reasons: Vec<String>,
    pub exception_ids: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionResult {
    pub dimension: CapabilityDimension,
    pub applicable: bool,
    pub required: bool,
    pub target: Option<HygieneMaturity>,
    pub observed: Option<HygieneMaturity>,
    pub status: DimensionStatus,
    pub rationale: Option<String>,
    pub evidence: Vec<InvariantEvidence>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HygieneProfileReport {
    pub schema_version: &'static str,
    pub facts: ProjectFacts,
    pub decisions: Vec<RuleDecision>,
    pub document_templates: DocumentTemplateReport,
    pub governance_graph: GovernanceGraph,
    pub ownership: OwnershipReport,
    pub topology_exceptions: Vec<TopologyExceptionEvidence>,
    pub dimensions: Vec<DimensionResult>,
    pub overall_observed: Option<HygieneMaturity>,
    pub meets_targets: bool,
}
