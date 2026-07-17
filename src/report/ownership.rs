use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReviewState {
    Current,
    DueSoon,
    Expired,
    Invalid,
    #[default]
    Missing,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Coverage {
    pub covered: usize,
    pub total: usize,
    pub percentage: u8,
}

impl Coverage {
    pub(crate) fn new(covered: usize, total: usize) -> Self {
        let percentage = if total == 0 {
            0
        } else {
            u8::try_from(covered.saturating_mul(100) / total).unwrap_or(100)
        };
        Self {
            covered,
            total,
            percentage,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipIdentityEvidence {
    pub identity: String,
    pub path: String,
    pub owner: Option<String>,
    pub owner_valid: bool,
    pub review_by: Option<String>,
    pub review_state: ReviewState,
    pub valid_understanders: Vec<String>,
    pub knowledge_bus_factor: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipReport {
    pub enabled: bool,
    pub responsibility_coverage: Coverage,
    pub review_coverage: Coverage,
    pub knowledge_redundancy_coverage: Coverage,
    pub reviews_due_soon: usize,
    pub reviews_expired: usize,
    pub identities: Vec<OwnershipIdentityEvidence>,
}
