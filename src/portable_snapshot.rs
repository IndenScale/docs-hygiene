use serde::{Deserialize, Serialize};

use crate::ContentAnchorScope;

pub const PORTABLE_SNAPSHOT_SCHEMA_VERSION: &str = "docs-hygiene.snapshot.v1";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PortableSnapshotManifest {
    pub schema_version: String,
    pub id: String,
    pub repository: String,
    pub commit: String,
    pub status: PortableSnapshotStatus,
    pub replaced_by: Option<String>,
    pub retain_until: Option<String>,
    pub entries: Vec<PortableSnapshotEntry>,
    pub signature: Option<PortableSnapshotSignature>,
}

impl PortableSnapshotManifest {
    pub fn signing_bytes(&self) -> serde_json::Result<Vec<u8>> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct SigningPayload<'a> {
            schema_version: &'a str,
            id: &'a str,
            repository: &'a str,
            commit: &'a str,
            status: PortableSnapshotStatus,
            replaced_by: &'a Option<String>,
            retain_until: &'a Option<String>,
            entries: &'a [PortableSnapshotEntry],
        }

        serde_json::to_vec(&SigningPayload {
            schema_version: &self.schema_version,
            id: &self.id,
            repository: &self.repository,
            commit: &self.commit,
            status: self.status,
            replaced_by: &self.replaced_by,
            retain_until: &self.retain_until,
            entries: &self.entries,
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PortableSnapshotStatus {
    Active,
    Replaced,
    Revoked,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PortableSnapshotEntry {
    pub target: String,
    pub path: String,
    pub payload: String,
    pub scope: ContentAnchorScope,
    pub locator: Option<String>,
    pub digest: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PortableSnapshotSignature {
    pub algorithm: String,
    pub key_id: String,
    pub value: String,
}
