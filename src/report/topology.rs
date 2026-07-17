use serde::Serialize;

use crate::TopologyDirection;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TopologyExceptionStatus {
    Applied,
    Idle,
    Invalid,
    Expired,
    Exceeded,
}

impl TopologyExceptionStatus {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::Idle => "idle",
            Self::Invalid => "invalid",
            Self::Expired => "expired",
            Self::Exceeded => "exceeded",
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyExceptionEvidence {
    pub id: String,
    pub node: String,
    pub direction: Option<TopologyDirection>,
    pub current_degree: Option<usize>,
    pub global_budget: Option<usize>,
    pub exception_budget: usize,
    pub remaining: Option<i64>,
    pub reason: String,
    pub owner: String,
    pub approved_by: String,
    pub expires: String,
    pub latest_observed_at: Option<String>,
    pub latest_observed_degree: Option<usize>,
    pub trend_delta: Option<i64>,
    pub transitive_impact: Vec<String>,
    pub status: TopologyExceptionStatus,
}
