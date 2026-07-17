use serde::{Deserialize, Serialize};

use crate::config::RuleMode;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CapabilityDimension {
    Structure,
    Identity,
    Dependency,
    Topology,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HygieneMaturity {
    Basic,
    Controlled,
    Governed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleCapability {
    pub dimension: CapabilityDimension,
    pub minimum_maturity: HygieneMaturity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleChecker {
    EntryDocs,
    DocumentStructure,
    DocumentContracts,
    Localization,
    Concepts,
    GovernanceIdentity,
    GovernanceTraceability,
    GovernanceTopology,
    ExternalAdapters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleApplicability {
    EntryDocs,
    DocumentStructure,
    DocumentContracts,
    Localization,
    Concepts,
    GovernanceIdentity,
    GovernanceTraceability,
    GovernanceTopology,
    ExternalAdapters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExceptionBehavior {
    LegacySuppression,
}

#[derive(Clone, Copy, Debug)]
pub struct RuleSpec {
    pub id: &'static str,
    pub capabilities: &'static [RuleCapability],
    pub default_mode: RuleMode,
    pub applicability: RuleApplicability,
    pub checker: RuleChecker,
    pub diagnostic_codes: &'static [&'static str],
    pub rationale: &'static str,
    pub remediation: &'static str,
    pub exception_behavior: ExceptionBehavior,
}

const STRUCTURE_BASIC: &[RuleCapability] = &[RuleCapability {
    dimension: CapabilityDimension::Structure,
    minimum_maturity: HygieneMaturity::Basic,
}];
const STRUCTURE_CONTROLLED: &[RuleCapability] = &[RuleCapability {
    dimension: CapabilityDimension::Structure,
    minimum_maturity: HygieneMaturity::Controlled,
}];
const IDENTITY_BASIC: &[RuleCapability] = &[RuleCapability {
    dimension: CapabilityDimension::Identity,
    minimum_maturity: HygieneMaturity::Basic,
}];
const IDENTITY_CONTROLLED: &[RuleCapability] = &[RuleCapability {
    dimension: CapabilityDimension::Identity,
    minimum_maturity: HygieneMaturity::Controlled,
}];
const IDENTITY_AND_DEPENDENCY: &[RuleCapability] = &[
    RuleCapability {
        dimension: CapabilityDimension::Identity,
        minimum_maturity: HygieneMaturity::Basic,
    },
    RuleCapability {
        dimension: CapabilityDimension::Dependency,
        minimum_maturity: HygieneMaturity::Basic,
    },
    RuleCapability {
        dimension: CapabilityDimension::Dependency,
        minimum_maturity: HygieneMaturity::Governed,
    },
];
const DEPENDENCY_CONTROLLED: &[RuleCapability] = &[RuleCapability {
    dimension: CapabilityDimension::Dependency,
    minimum_maturity: HygieneMaturity::Controlled,
}];
const TOPOLOGY_CONTROLLED: &[RuleCapability] = &[RuleCapability {
    dimension: CapabilityDimension::Topology,
    minimum_maturity: HygieneMaturity::Controlled,
}];
const ENABLING_TOOL: &[RuleCapability] = &[];

pub const RULE_SPECS: [RuleSpec; 9] = [
    RuleSpec {
        id: "project.entry-docs",
        capabilities: STRUCTURE_BASIC,
        default_mode: RuleMode::Auto,
        applicability: RuleApplicability::EntryDocs,
        checker: RuleChecker::EntryDocs,
        diagnostic_codes: &["DH_REQUIRED_001"],
        rationale: "Stable entry documents keep project intent discoverable.",
        remediation: "Configure rules.project.entry-docs.mode as auto, required, or disabled.",
        exception_behavior: ExceptionBehavior::LegacySuppression,
    },
    RuleSpec {
        id: "docs.structure",
        capabilities: STRUCTURE_BASIC,
        default_mode: RuleMode::Auto,
        applicability: RuleApplicability::DocumentStructure,
        checker: RuleChecker::DocumentStructure,
        diagnostic_codes: &[
            "DH_NAME_001",
            "DH_SEQ_001",
            "DH_SEQ_002",
            "DH_SIZE_001",
            "DH_ASCII_001",
            "DH_LINK_001",
            "DH_SLUG_001",
        ],
        rationale: "Predictable structure keeps navigation and document growth reviewable.",
        remediation: "Configure rules.docs.structure.mode as auto, required, or disabled.",
        exception_behavior: ExceptionBehavior::LegacySuppression,
    },
    RuleSpec {
        id: "documents.contracts",
        capabilities: STRUCTURE_CONTROLLED,
        default_mode: RuleMode::Auto,
        applicability: RuleApplicability::DocumentContracts,
        checker: RuleChecker::DocumentContracts,
        diagnostic_codes: &[
            "DH_CONTRACT_001",
            "DH_CONTRACT_002",
            "DH_CONTRACT_003",
            "DH_CONTRACT_004",
            "DH_TEMPLATE_001",
            "DH_TEMPLATE_002",
            "DH_TEMPLATE_003",
            "DH_TEMPLATE_004",
            "DH_MATURITY_001",
            "DH_KIND_001",
            "DH_KIND_002",
            "DH_FRONTMATTER_001",
        ],
        rationale: "Semantic sections make each document type complete enough for its role.",
        remediation: "Configure rules.documents.contracts.mode as auto, required, or disabled.",
        exception_behavior: ExceptionBehavior::LegacySuppression,
    },
    RuleSpec {
        id: "localization.parity",
        capabilities: IDENTITY_CONTROLLED,
        default_mode: RuleMode::Auto,
        applicability: RuleApplicability::Localization,
        checker: RuleChecker::Localization,
        diagnostic_codes: &[
            "DH_REPRESENTATION_001",
            "DH_REPRESENTATION_002",
            "DH_LANG_001",
            "DH_LANG_002",
        ],
        rationale: "Language representations must preserve one authoritative project identity.",
        remediation: "Configure rules.localization.parity.mode as auto, required, or disabled.",
        exception_behavior: ExceptionBehavior::LegacySuppression,
    },
    RuleSpec {
        id: "concepts.references",
        capabilities: IDENTITY_BASIC,
        default_mode: RuleMode::Auto,
        applicability: RuleApplicability::Concepts,
        checker: RuleChecker::Concepts,
        diagnostic_codes: &["DH_CONCEPT_001", "DH_CONCEPT_002"],
        rationale: "Stable concept targets prevent highlighted terms from becoming implicit local jargon.",
        remediation: "Configure rules.concepts.references.mode as auto, required, or disabled.",
        exception_behavior: ExceptionBehavior::LegacySuppression,
    },
    RuleSpec {
        id: "governance.identity",
        capabilities: IDENTITY_AND_DEPENDENCY,
        default_mode: RuleMode::Auto,
        applicability: RuleApplicability::GovernanceIdentity,
        checker: RuleChecker::GovernanceIdentity,
        diagnostic_codes: &[
            "DH_GOVERNANCE_001",
            "DH_REFERENCE_001",
            "DH_SELECTOR_001",
            "DH_LIBRARY_001",
            "DH_BODY_001",
        ],
        rationale: "Stable identities and references make project knowledge resolvable.",
        remediation: "Configure rules.governance.identity.mode as auto, required, or disabled.",
        exception_behavior: ExceptionBehavior::LegacySuppression,
    },
    RuleSpec {
        id: "governance.traceability",
        capabilities: DEPENDENCY_CONTROLLED,
        default_mode: RuleMode::Auto,
        applicability: RuleApplicability::GovernanceTraceability,
        checker: RuleChecker::GovernanceTraceability,
        diagnostic_codes: &["DH_DERIVATION_001", "DH_DERIVATION_002"],
        rationale: "Typed adjacent-level edges show whether intent reaches definition and implementation.",
        remediation: "Configure rules.governance.traceability.mode as auto, required, or disabled.",
        exception_behavior: ExceptionBehavior::LegacySuppression,
    },
    RuleSpec {
        id: "governance.topology",
        capabilities: TOPOLOGY_CONTROLLED,
        default_mode: RuleMode::Auto,
        applicability: RuleApplicability::GovernanceTopology,
        checker: RuleChecker::GovernanceTopology,
        diagnostic_codes: &["DH_TOPOLOGY_001", "DH_TOPOLOGY_002"],
        rationale: "Explicit graph thresholds keep dependency concentration and cycles reviewable.",
        remediation: "Configure governance.topology thresholds or set rules.governance.topology.mode explicitly.",
        exception_behavior: ExceptionBehavior::LegacySuppression,
    },
    RuleSpec {
        id: "adapters.external",
        capabilities: ENABLING_TOOL,
        default_mode: RuleMode::Auto,
        applicability: RuleApplicability::ExternalAdapters,
        checker: RuleChecker::ExternalAdapters,
        diagnostic_codes: &["DH_ADAPTER_001"],
        rationale: "External surface checks remain visible in the same governance run.",
        remediation: "Configure rules.adapters.external.mode as auto, required, or disabled.",
        exception_behavior: ExceptionBehavior::LegacySuppression,
    },
];

pub fn rule_spec(id: &str) -> Option<&'static RuleSpec> {
    RULE_SPECS.iter().find(|spec| spec.id == id)
}

pub fn rule_spec_for_checker(checker: RuleChecker) -> &'static RuleSpec {
    RULE_SPECS
        .iter()
        .find(|spec| spec.checker == checker)
        .expect("every built-in checker must have one compatibility-family rule")
}

pub fn rule_spec_for_diagnostic(code: &str) -> Option<&'static RuleSpec> {
    RULE_SPECS
        .iter()
        .find(|spec| spec.diagnostic_codes.contains(&code))
}
