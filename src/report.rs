use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::checks::Diagnostic;
use crate::governance::GovernanceGraph;

mod output;
mod ownership;
mod topology;

pub use output::{print_json_report, print_text_report};
pub use ownership::{Coverage, OwnershipIdentityEvidence, OwnershipReport, ReviewState};
pub use topology::{TopologyExceptionEvidence, TopologyExceptionStatus};

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Debug)]
pub struct Report {
    pub diagnostics: Vec<Diagnostic>,
    pub suppressed_diagnostics: Vec<SuppressedDiagnostic>,
    pub semantic_content_anchors_checked: usize,
    pub governance_graph: GovernanceGraph,
    pub ownership: OwnershipReport,
    pub topology_exceptions: Vec<TopologyExceptionEvidence>,
    pub document_templates: DocumentTemplateReport,
    pub summary: Summary,
    pub root: PathBuf,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuppressedDiagnostic {
    pub code: String,
    pub path: String,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentTemplateReport {
    pub configured_templates: usize,
    pub configured_profiles: usize,
    pub bindings: BTreeMap<String, Vec<String>>,
    pub template_revisions: BTreeMap<String, TemplateRevisionReport>,
    pub profile_revision_pins: BTreeMap<String, u64>,
    pub untemplated_profiles: Vec<String>,
    pub unused_templates: Vec<String>,
    pub unrevisioned_templates: Vec<String>,
    pub unpinned_profiles: Vec<String>,
    pub outdated_profiles: Vec<String>,
    pub incompatible_profiles: Vec<String>,
    pub registry_valid: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateRevisionReport {
    pub revision: u64,
    pub compatible_from: u64,
}

impl DocumentTemplateReport {
    pub fn proves_reuse(&self) -> bool {
        self.registry_valid
            && self.configured_templates > 0
            && self.configured_profiles > 0
            && self.untemplated_profiles.is_empty()
            && self.unused_templates.is_empty()
            && self.bindings.values().map(Vec::len).sum::<usize>() == self.configured_profiles
    }

    pub fn proves_migration(&self) -> bool {
        self.proves_reuse()
            && self.template_revisions.len() == self.configured_templates
            && self.profile_revision_pins.len() == self.configured_profiles
            && self.unrevisioned_templates.is_empty()
            && self.unpinned_profiles.is_empty()
            && self.outdated_profiles.is_empty()
            && self.incompatible_profiles.is_empty()
    }
}

impl Default for DocumentTemplateReport {
    fn default() -> Self {
        Self {
            configured_templates: 0,
            configured_profiles: 0,
            bindings: BTreeMap::new(),
            template_revisions: BTreeMap::new(),
            profile_revision_pins: BTreeMap::new(),
            untemplated_profiles: Vec::new(),
            unused_templates: Vec::new(),
            unrevisioned_templates: Vec::new(),
            unpinned_profiles: Vec::new(),
            outdated_profiles: Vec::new(),
            incompatible_profiles: Vec::new(),
            registry_valid: true,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub files_checked: usize,
    pub diagnostic_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub hint_count: usize,
}

impl Report {
    pub fn new(diagnostics: Vec<Diagnostic>, files_checked: usize, root: &Path) -> Self {
        let error_count = diagnostics
            .iter()
            .filter(|diag| matches!(diag.severity, Severity::Error))
            .count();
        let warning_count = diagnostics
            .iter()
            .filter(|diag| matches!(diag.severity, Severity::Warning))
            .count();
        let info_count = diagnostics
            .iter()
            .filter(|diag| matches!(diag.severity, Severity::Info))
            .count();
        let hint_count = diagnostics
            .iter()
            .filter(|diag| matches!(diag.severity, Severity::Hint))
            .count();
        Self {
            summary: Summary {
                files_checked,
                diagnostic_count: diagnostics.len(),
                error_count,
                warning_count,
                info_count,
                hint_count,
            },
            diagnostics,
            suppressed_diagnostics: Vec::new(),
            semantic_content_anchors_checked: 0,
            governance_graph: GovernanceGraph::default(),
            ownership: OwnershipReport::default(),
            topology_exceptions: Vec::new(),
            document_templates: DocumentTemplateReport::default(),
            root: root.to_path_buf(),
        }
    }

    pub(crate) fn with_suppressed(
        mut self,
        suppressed_diagnostics: Vec<SuppressedDiagnostic>,
    ) -> Self {
        self.suppressed_diagnostics = suppressed_diagnostics;
        self
    }

    pub(crate) fn with_semantic_content_anchors_checked(mut self, count: usize) -> Self {
        self.semantic_content_anchors_checked = count;
        self
    }

    pub(crate) fn with_governance_graph(mut self, graph: GovernanceGraph) -> Self {
        self.governance_graph = graph;
        self
    }

    pub(crate) fn with_ownership(mut self, ownership: OwnershipReport) -> Self {
        self.ownership = ownership;
        self
    }

    pub(crate) fn with_topology_exceptions(
        mut self,
        exceptions: Vec<TopologyExceptionEvidence>,
    ) -> Self {
        self.topology_exceptions = exceptions;
        self
    }

    pub(crate) fn with_document_templates(mut self, templates: DocumentTemplateReport) -> Self {
        self.document_templates = templates;
        self
    }

    pub fn explain(code: &str) -> Option<&'static str> {
        match code {
            "DH_REQUIRED_001" => Some("A required project documentation file is missing."),
            "DH_NAME_001" => Some("A docs file name does not match the configured pattern."),
            "DH_SEQ_001" => Some("A numbered docs directory has a gap in its sequence."),
            "DH_SEQ_002" => Some("A numbered docs directory uses the same number more than once."),
            "DH_SIZE_001" => Some("A docs file exceeds the configured line budget."),
            "DH_ASCII_001" => Some("A document contains a forbidden ASCII art block."),
            "DH_REPRESENTATION_001" => {
                Some("A canonical document is missing a localized representation.")
            }
            "DH_REPRESENTATION_002" => {
                Some("A localized representation has no canonical document.")
            }
            "DH_LANG_001" => {
                Some("A document has less CJK content than its language policy expects.")
            }
            "DH_LANG_002" => {
                Some("A document has more CJK content than its language policy allows.")
            }
            "DH_CONTRACT_001" => {
                Some("A path-inferred document profile is missing a required section.")
            }
            "DH_CONTRACT_002" => {
                Some("A path-inferred document profile is missing a required field.")
            }
            "DH_CONTRACT_003" => {
                Some("A required document section contains a declared placeholder.")
            }
            "DH_CONTRACT_004" => {
                Some("Required document sections are not in the configured order.")
            }
            "DH_TEMPLATE_001" => Some(
                "A document template or profile has a duplicate identity, invalid binding, or conflicting semantic member.",
            ),
            "DH_TEMPLATE_002" => Some("A configured document template has no profile binding."),
            "DH_TEMPLATE_003" => {
                Some("A document template or profile needs a compatible revision pin migration.")
            }
            "DH_TEMPLATE_004" => Some(
                "A document profile revision pin is outside its template compatibility window.",
            ),
            "DH_MATURITY_001" => {
                Some("Project scale signals recommend stronger document governance.")
            }
            "DH_KIND_001" => Some(
                "A Document Kind registry entry does not consistently bind its base, filename pattern, profile, or scaffold contract.",
            ),
            "DH_KIND_002" => Some(
                "A typed Document Kind uses a missing, stale, or incompatible frontmatter Schema revision.",
            ),
            "DH_FRONTMATTER_001" => Some(
                "A typed Document Kind violates its frontmatter field, enum, format, condition, invariant, or unknown-field policy.",
            ),
            "DH_ACTIVATION_001" => Some(
                "Project facts activated a governance rule without an explicit feature policy; the diagnostic records the evidence and override path.",
            ),
            "DH_CONCEPT_001" => Some("A highlighted term is missing a concept definition file."),
            "DH_CONCEPT_002" => Some("A concept definition file is not referenced by docs."),
            "DH_LINK_001" => Some(
                "A project-root-local Markdown Link or image target does not resolve to an existing path; external URL reachability is outside this rule.",
            ),
            "DH_SLUG_001" => Some(
                "A configured Document Kind has an invalid, reserved, conflicting, representation-drifted, or migration-incomplete slug identity.",
            ),
            "DH_GOVERNANCE_001" => {
                Some("A governance manifest is missing, malformed, duplicated, or invalid.")
            }
            "DH_REFERENCE_001" => Some(
                "A governed asset has a missing, invalid, or content-hash-stale semantic Wiki Link to a Library identity.",
            ),
            "DH_SELECTOR_001" => {
                Some("A semantic Wiki Link selector does not resolve to a target Markdown heading.")
            }
            "DH_LIBRARY_001" => Some(
                "A Library directory has a missing, malformed, duplicate, or undeclared member term.",
            ),
            "DH_BODY_001" => Some(
                "A Body Package has a missing, malformed, duplicate, unsafe, undeclared, or localized-mismatched member.",
            ),
            "DH_CLAIM_001" => Some(
                "An explicitly governed core claim has an invalid authority, forbidden duplicate, expired migration, or unpinned/stale controlled excerpt.",
            ),
            "DH_PIN_001" => Some("A critical dependency is missing a required content anchor."),
            "DH_PIN_002" => Some(
                "A critical dependency anchor has insufficient scope or violates whole-file policy.",
            ),
            "DH_PIN_003" => {
                Some("A critical dependency anchor uses an algorithm not allowed by policy.")
            }
            "DH_PIN_004" => {
                Some("A critical dependency target changed after its digest was accepted.")
            }
            "DH_PIN_005" => {
                Some("A critical dependency pin is missing audit time or exceeds its maximum age.")
            }
            "DH_PIN_006" => Some(
                "A critical dependency policy or anchor declaration is invalid or unverifiable.",
            ),
            "DH_SNAPSHOT_001" => Some(
                "A portable snapshot manifest or registration is missing, malformed, or duplicated.",
            ),
            "DH_SNAPSHOT_002" => {
                Some("A portable snapshot repository identity is invalid or inconsistent.")
            }
            "DH_SNAPSHOT_003" => Some("A portable snapshot commit OID is invalid or inconsistent."),
            "DH_SNAPSHOT_004" => Some(
                "A portable snapshot source or payload path is unsafe, missing, or inconsistent.",
            ),
            "DH_SNAPSHOT_005" => Some(
                "A portable snapshot digest, scope, locator, or local payload is inconsistent.",
            ),
            "DH_SNAPSHOT_006" => {
                Some("A portable snapshot signature is required, untrusted, or invalid.")
            }
            "DH_SNAPSHOT_007" => {
                Some("A portable snapshot lifecycle, replacement, or retention policy is invalid.")
            }
            "DH_DERIVATION_001" => {
                Some("A Body has a missing or invalid adjacent-refinement-level derivation.")
            }
            "DH_DERIVATION_002" => {
                Some("A Library has a missing or invalid adjacent-refinement-level projection.")
            }
            "DH_TOPOLOGY_001" => {
                Some("A governed identity exceeds an explicit Fan-In or Fan-Out threshold.")
            }
            "DH_TOPOLOGY_002" => {
                Some("The normalized governance graph contains a forbidden directed cycle.")
            }
            "DH_TOPOLOGY_003" => Some(
                "A supernode exception has invalid identity, target, direction, budget, audit metadata, or expiry.",
            ),
            "DH_TOPOLOGY_004" => Some("A supernode exception is idle and should be removed."),
            "DH_TOPOLOGY_005" => Some(
                "A supernode exception has missing, invalid, unordered, or future degree history.",
            ),
            "DH_OWNERSHIP_001" => Some(
                "A governed identity has missing or invalid ownership, or its principal directory is invalid.",
            ),
            "DH_REVIEW_001" => {
                Some("A governed identity has missing, invalid, or expired review evidence.")
            }
            "DH_REVIEW_002" => Some(
                "A governed identity review is approaching its deadline and needs an explicit reset.",
            ),
            "DH_KNOWLEDGE_001" => {
                Some("A governed identity lacks current confirmations from two active people.")
            }
            "DH_ADAPTER_001" => Some("An external documentation adapter reported a failure."),
            "DH_SUPPRESSION_001" => Some("A diagnostic was suppressed by configuration."),
            _ => None,
        }
    }
}
