use serde::Serialize;

use crate::{ContentAnchorScope, GovernanceLocation, SnapshotProvenance};

pub const REFERENCE_OCCURRENCE_SCHEMA_VERSION: &str = "docs-hygiene.reference-occurrence.v1";
pub const SYNTAX_WIKI_LINK: &str = "wikiLink";
pub const SYNTAX_MARKDOWN_LINK: &str = "markdownLink";
pub const SYNTAX_FRONTMATTER: &str = "frontmatter";
pub const CONTEXT_GOVERNED_CONTENT: &str = "governedContent";
pub const CONTEXT_PROJECT_NAVIGATION: &str = "projectNavigation";
pub const CONTEXT_IDENTITY_DECLARATION: &str = "identityDeclaration";
pub const CONTEXT_GOVERNED_ANCHOR: &str = "governedAnchor";

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferencePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<ReferenceAnchorPayload>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceAnchorPayload {
    pub algorithm: String,
    pub digest: String,
    #[serde(skip_serializing_if = "ContentAnchorScope::is_file")]
    pub scope: ContentAnchorScope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_document_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<SnapshotProvenance>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceOccurrence {
    pub schema_version: &'static str,
    pub raw_target: String,
    pub syntax: String,
    pub context: String,
    pub location: GovernanceLocation,
    pub payload: ReferencePayload,
}

impl ReferenceOccurrence {
    pub fn new(
        raw_target: impl Into<String>,
        syntax: impl Into<String>,
        context: impl Into<String>,
        location: GovernanceLocation,
        payload: ReferencePayload,
    ) -> Self {
        Self {
            schema_version: REFERENCE_OCCURRENCE_SCHEMA_VERSION,
            raw_target: raw_target.into(),
            syntax: syntax.into(),
            context: context.into(),
            location,
            payload,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReferenceDisposition {
    SemanticDependency,
    NavigationOnly,
    IdentityDeclaration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferencePolicy {
    pub syntax: &'static str,
    pub context: &'static str,
    pub disposition: ReferenceDisposition,
}

pub const REFERENCE_POLICIES: &[ReferencePolicy] = &[
    ReferencePolicy {
        syntax: SYNTAX_WIKI_LINK,
        context: CONTEXT_GOVERNED_CONTENT,
        disposition: ReferenceDisposition::SemanticDependency,
    },
    ReferencePolicy {
        syntax: SYNTAX_MARKDOWN_LINK,
        context: CONTEXT_PROJECT_NAVIGATION,
        disposition: ReferenceDisposition::NavigationOnly,
    },
    ReferencePolicy {
        syntax: SYNTAX_FRONTMATTER,
        context: CONTEXT_IDENTITY_DECLARATION,
        disposition: ReferenceDisposition::IdentityDeclaration,
    },
    ReferencePolicy {
        syntax: SYNTAX_FRONTMATTER,
        context: CONTEXT_GOVERNED_ANCHOR,
        disposition: ReferenceDisposition::SemanticDependency,
    },
];

pub fn reference_disposition(
    occurrence: &ReferenceOccurrence,
    policies: &[ReferencePolicy],
) -> Option<ReferenceDisposition> {
    policies
        .iter()
        .find(|policy| policy.syntax == occurrence.syntax && policy.context == occurrence.context)
        .map(|policy| policy.disposition)
}
