use crate::activation::{CapabilityDimension, HygieneMaturity, RuleChecker};

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InvariantDelivery {
    Delivered,
    Partial,
    Missing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvariantApplicability {
    Checker,
    Capability,
    ContentPolicy,
    DocumentTemplates,
    DocumentKinds,
    CoreClaims,
    CriticalDependencies,
    PortableSnapshots,
    LocalizedRepresentation,
    SlugSchema,
    ContentAnchor,
    ScopedContentAnchor,
    Selector,
    GovernanceGraph,
    AuthorityMigration,
    TopologyPolicy,
    TopologyExceptions,
}

#[derive(Clone, Copy, Debug)]
pub struct InvariantSpec {
    pub id: &'static str,
    pub dimension: CapabilityDimension,
    pub minimum_maturity: HygieneMaturity,
    pub delivery: InvariantDelivery,
    pub checkers: &'static [RuleChecker],
    pub diagnostic_codes: &'static [&'static str],
}

impl InvariantSpec {
    pub fn applicability(self) -> InvariantApplicability {
        match self.id {
            "structure.content-policy" => InvariantApplicability::ContentPolicy,
            "structure.reusable-templates" | "structure.template-migration" => {
                InvariantApplicability::DocumentTemplates
            }
            "structure.kind-schema" => InvariantApplicability::DocumentKinds,
            "identity.library-claims" => InvariantApplicability::CoreClaims,
            "dependency.critical-pins" => InvariantApplicability::CriticalDependencies,
            "dependency.portable-snapshot" => InvariantApplicability::PortableSnapshots,
            "identity.canonical-source" => InvariantApplicability::LocalizedRepresentation,
            "identity.slug-schema" => InvariantApplicability::SlugSchema,
            "identity.authority-migration" => InvariantApplicability::AuthorityMigration,
            "dependency.content-anchor" | "dependency.target-staleness" => {
                InvariantApplicability::ContentAnchor
            }
            "dependency.scoped-anchor" => InvariantApplicability::ScopedContentAnchor,
            "dependency.selector" => InvariantApplicability::Selector,
            "dependency.typed-edges" | "dependency.transitive-impact" | "topology.metrics" => {
                InvariantApplicability::GovernanceGraph
            }
            "topology.fan-and-cycles" => InvariantApplicability::GovernanceGraph,
            "topology.thresholds" => InvariantApplicability::TopologyPolicy,
            "topology.budgets" => InvariantApplicability::TopologyPolicy,
            "topology.public-exceptions" | "topology.trends" => {
                InvariantApplicability::TopologyExceptions
            }
            _ if self.delivery != InvariantDelivery::Delivered => {
                InvariantApplicability::Capability
            }
            _ => InvariantApplicability::Checker,
        }
    }
}

use CapabilityDimension::{Dependency, Identity, Structure, Topology};
use HygieneMaturity::{Basic, Controlled, Governed};
use InvariantDelivery::Delivered;
use RuleChecker::{
    Concepts, DocumentContracts, DocumentStructure, EntryDocs, GovernanceIdentity,
    GovernanceTopology, GovernanceTraceability, Localization,
};

pub const INVARIANTS: &[InvariantSpec] = &[
    InvariantSpec {
        id: "structure.entry-docs",
        dimension: Structure,
        minimum_maturity: Basic,
        delivery: Delivered,
        checkers: &[EntryDocs],
        diagnostic_codes: &["DH_REQUIRED_001"],
    },
    InvariantSpec {
        id: "structure.naming-sequence",
        dimension: Structure,
        minimum_maturity: Basic,
        delivery: Delivered,
        checkers: &[DocumentStructure],
        diagnostic_codes: &["DH_NAME_001", "DH_SEQ_001", "DH_SEQ_002"],
    },
    InvariantSpec {
        id: "structure.local-links",
        dimension: Structure,
        minimum_maturity: Basic,
        delivery: Delivered,
        checkers: &[DocumentStructure],
        diagnostic_codes: &["DH_LINK_001"],
    },
    InvariantSpec {
        id: "structure.contracts",
        dimension: Structure,
        minimum_maturity: Controlled,
        delivery: Delivered,
        checkers: &[DocumentContracts],
        diagnostic_codes: &[
            "DH_CONTRACT_001",
            "DH_CONTRACT_002",
            "DH_CONTRACT_003",
            "DH_CONTRACT_004",
        ],
    },
    InvariantSpec {
        id: "structure.content-policy",
        dimension: Structure,
        minimum_maturity: Controlled,
        delivery: Delivered,
        checkers: &[DocumentStructure],
        diagnostic_codes: &["DH_SIZE_001", "DH_ASCII_001"],
    },
    InvariantSpec {
        id: "structure.reusable-templates",
        dimension: Structure,
        minimum_maturity: Controlled,
        delivery: Delivered,
        checkers: &[DocumentContracts],
        diagnostic_codes: &["DH_TEMPLATE_001", "DH_TEMPLATE_002"],
    },
    InvariantSpec {
        id: "structure.template-migration",
        dimension: Structure,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[DocumentContracts],
        diagnostic_codes: &["DH_TEMPLATE_001", "DH_TEMPLATE_003", "DH_TEMPLATE_004"],
    },
    InvariantSpec {
        id: "structure.kind-schema",
        dimension: Structure,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[DocumentContracts],
        diagnostic_codes: &["DH_KIND_001", "DH_KIND_002", "DH_FRONTMATTER_001"],
    },
    InvariantSpec {
        id: "identity.stable-ids",
        dimension: Identity,
        minimum_maturity: Basic,
        delivery: Delivered,
        checkers: &[GovernanceIdentity],
        diagnostic_codes: &["DH_GOVERNANCE_001", "DH_LIBRARY_001", "DH_BODY_001"],
    },
    InvariantSpec {
        id: "identity.duplicates",
        dimension: Identity,
        minimum_maturity: Basic,
        delivery: Delivered,
        checkers: &[GovernanceIdentity],
        diagnostic_codes: &["DH_GOVERNANCE_001", "DH_LIBRARY_001", "DH_BODY_001"],
    },
    InvariantSpec {
        id: "identity.library-claims",
        dimension: Identity,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[GovernanceIdentity],
        diagnostic_codes: &["DH_CLAIM_001", "DH_REFERENCE_001"],
    },
    InvariantSpec {
        id: "identity.canonical-source",
        dimension: Identity,
        minimum_maturity: Controlled,
        delivery: Delivered,
        checkers: &[Localization],
        diagnostic_codes: &["DH_REPRESENTATION_001", "DH_REPRESENTATION_002"],
    },
    InvariantSpec {
        id: "identity.slug-schema",
        dimension: Identity,
        minimum_maturity: Controlled,
        delivery: Delivered,
        checkers: &[DocumentStructure],
        diagnostic_codes: &["DH_SLUG_001"],
    },
    InvariantSpec {
        id: "identity.semantic-reference",
        dimension: Identity,
        minimum_maturity: Controlled,
        delivery: Delivered,
        checkers: &[Concepts, GovernanceIdentity],
        diagnostic_codes: &["DH_CONCEPT_001", "DH_CONCEPT_002", "DH_REFERENCE_001"],
    },
    InvariantSpec {
        id: "identity.lifecycle",
        dimension: Identity,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[GovernanceIdentity],
        diagnostic_codes: &["DH_GOVERNANCE_001", "DH_LIBRARY_001", "DH_BODY_001"],
    },
    InvariantSpec {
        id: "identity.authority-migration",
        dimension: Identity,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[GovernanceIdentity],
        diagnostic_codes: &["DH_GOVERNANCE_001"],
    },
    InvariantSpec {
        id: "dependency.resolve",
        dimension: Dependency,
        minimum_maturity: Basic,
        delivery: Delivered,
        checkers: &[GovernanceIdentity, GovernanceTraceability],
        diagnostic_codes: &["DH_REFERENCE_001", "DH_DERIVATION_001", "DH_DERIVATION_002"],
    },
    InvariantSpec {
        id: "dependency.typed-edges",
        dimension: Dependency,
        minimum_maturity: Controlled,
        delivery: Delivered,
        checkers: &[GovernanceIdentity, GovernanceTraceability],
        diagnostic_codes: &["DH_REFERENCE_001", "DH_DERIVATION_001", "DH_DERIVATION_002"],
    },
    InvariantSpec {
        id: "dependency.content-anchor",
        dimension: Dependency,
        minimum_maturity: Controlled,
        delivery: Delivered,
        checkers: &[GovernanceIdentity],
        diagnostic_codes: &["DH_REFERENCE_001"],
    },
    InvariantSpec {
        id: "dependency.target-staleness",
        dimension: Dependency,
        minimum_maturity: Controlled,
        delivery: Delivered,
        checkers: &[GovernanceIdentity],
        diagnostic_codes: &["DH_REFERENCE_001"],
    },
    InvariantSpec {
        id: "dependency.selector",
        dimension: Dependency,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[GovernanceIdentity],
        diagnostic_codes: &["DH_SELECTOR_001"],
    },
    InvariantSpec {
        id: "dependency.scoped-anchor",
        dimension: Dependency,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[GovernanceIdentity],
        diagnostic_codes: &["DH_REFERENCE_001", "DH_SELECTOR_001"],
    },
    InvariantSpec {
        id: "dependency.critical-pins",
        dimension: Dependency,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[GovernanceTraceability],
        diagnostic_codes: &[
            "DH_PIN_001",
            "DH_PIN_002",
            "DH_PIN_003",
            "DH_PIN_004",
            "DH_PIN_005",
            "DH_PIN_006",
        ],
    },
    InvariantSpec {
        id: "dependency.portable-snapshot",
        dimension: Dependency,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[GovernanceTraceability],
        diagnostic_codes: &[
            "DH_SNAPSHOT_001",
            "DH_SNAPSHOT_002",
            "DH_SNAPSHOT_003",
            "DH_SNAPSHOT_004",
            "DH_SNAPSHOT_005",
            "DH_SNAPSHOT_006",
            "DH_SNAPSHOT_007",
        ],
    },
    InvariantSpec {
        id: "dependency.transitive-impact",
        dimension: Dependency,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[GovernanceTraceability],
        diagnostic_codes: &["DH_DERIVATION_001", "DH_DERIVATION_002"],
    },
    InvariantSpec {
        id: "topology.metrics",
        dimension: Topology,
        minimum_maturity: Basic,
        delivery: Delivered,
        checkers: &[GovernanceIdentity, GovernanceTraceability],
        diagnostic_codes: &[
            "DH_GOVERNANCE_001",
            "DH_REFERENCE_001",
            "DH_DERIVATION_001",
            "DH_DERIVATION_002",
        ],
    },
    InvariantSpec {
        id: "topology.fan-and-cycles",
        dimension: Topology,
        minimum_maturity: Controlled,
        delivery: Delivered,
        checkers: &[GovernanceIdentity, GovernanceTraceability],
        diagnostic_codes: &[
            "DH_GOVERNANCE_001",
            "DH_REFERENCE_001",
            "DH_DERIVATION_001",
            "DH_DERIVATION_002",
        ],
    },
    InvariantSpec {
        id: "topology.thresholds",
        dimension: Topology,
        minimum_maturity: Controlled,
        delivery: Delivered,
        checkers: &[GovernanceTopology],
        diagnostic_codes: &["DH_TOPOLOGY_001", "DH_TOPOLOGY_002"],
    },
    InvariantSpec {
        id: "topology.budgets",
        dimension: Topology,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[GovernanceTopology],
        diagnostic_codes: &["DH_TOPOLOGY_001", "DH_TOPOLOGY_003"],
    },
    InvariantSpec {
        id: "topology.public-exceptions",
        dimension: Topology,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[GovernanceTopology],
        diagnostic_codes: &["DH_TOPOLOGY_003", "DH_TOPOLOGY_004"],
    },
    InvariantSpec {
        id: "topology.trends",
        dimension: Topology,
        minimum_maturity: Governed,
        delivery: Delivered,
        checkers: &[GovernanceTopology],
        diagnostic_codes: &["DH_TOPOLOGY_005"],
    },
];
