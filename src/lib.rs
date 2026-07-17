//! Public implementation reference for Docs Hygiene policies.
//!
//! This crate is governed as `SDK-001` by `sdk-manifest.yml`. The manifest declares
//! its Glossary projection. The CLI is one consumer of this API.
//! Semantic source: [[GLOSSARY-001]].

pub mod activation;
pub mod checks;
pub mod config;
pub mod document_kinds;
pub mod governance;
pub mod kind_migration;
pub mod pin_update;
pub mod profile;
pub mod reference;
pub mod report;
pub mod template_migration;

pub use activation::{
    ActivationReport, CapabilityDimension, ExceptionBehavior, HygieneMaturity, ProjectFacts,
    RULE_SPECS, RuleApplicability, RuleCapability, RuleChecker, RuleDecision, RuleSpec, RuleState,
    evaluate_rule_activation, print_json_activation, print_text_activation, rule_spec,
    rule_spec_for_checker, rule_spec_for_diagnostic,
};
pub use checks::run_checks;
pub use checks::{
    LibraryClaimCandidate, LibraryClaimScanReport, print_json_library_claim_scan,
    print_text_library_claim_scan, scan_library_claim_candidates,
};
pub use config::{
    Config, CoreClaimAuthorityConfig, CoreClaimConfig, CoreClaimOccurrenceConfig,
    CoreClaimOccurrencePolicy, DimensionApplicability, DimensionProfileConfig, DocumentKindConfig,
    FrontmatterConditionConfig, FrontmatterFieldConfig, FrontmatterFieldSource,
    FrontmatterFieldType, FrontmatterInvariantConfig, FrontmatterInvariantOperator,
    FrontmatterPredicateConfig, FrontmatterSchemaConfig, GovernanceContentAnchorConfig,
    GovernanceTopologyConfig, HygieneProfileConfig, HygieneProfileDimensionsConfig,
    KindScaffoldConfig, RuleMode, RulePolicyConfig,
};
pub use document_kinds::{
    KindIssue, KindIssueCategory, ScaffoldDocumentPlan, ScaffoldDocumentRequest,
    parse_frontmatter_mapping, plan_scaffold_document, validate_document_kind_registry,
    validate_kind_frontmatter,
};
pub use governance::{
    ContentAnchor, ContentAnchorScope, GovernanceEdge, GovernanceEdgeKind, GovernanceGraph,
    GovernanceGraphMetrics, GovernanceLocation, GovernanceNode, LifecycleProvenance,
    ReferenceRelation, RefinementLevel,
};
pub use kind_migration::{
    KindMigrationBlock, KindMigrationChange, KindMigrationReport, KindTemplateMigrationChange,
    migrate_document_kinds, print_json_kind_migration, print_text_kind_migration,
};
pub use pin_update::{
    PinUpdateBlock, PinUpdateChange, PinUpdateReport, PinUpdateRequest, print_json_pin_update,
    print_text_pin_update, update_critical_pins,
};
pub use profile::{
    DimensionResult, DimensionStatus, HygieneProfileReport, INVARIANTS, InvariantApplicability,
    InvariantDelivery, InvariantEvidence, InvariantOutcome, InvariantSpec,
    evaluate_hygiene_profile, print_json_profile, print_text_profile,
};
pub use reference::{
    CONTEXT_GOVERNED_ANCHOR, CONTEXT_GOVERNED_CONTENT, CONTEXT_IDENTITY_DECLARATION,
    CONTEXT_PROJECT_NAVIGATION, REFERENCE_OCCURRENCE_SCHEMA_VERSION, REFERENCE_POLICIES,
    ReferenceAnchorPayload, ReferenceDisposition, ReferenceOccurrence, ReferencePayload,
    ReferencePolicy, SYNTAX_FRONTMATTER, SYNTAX_MARKDOWN_LINK, SYNTAX_WIKI_LINK,
    reference_disposition,
};
pub use report::{
    DocumentTemplateReport, Report, SuppressedDiagnostic, TemplateRevisionReport,
    print_json_report, print_text_report,
};
pub use template_migration::{
    TemplateMigrationBlock, TemplateMigrationChange, TemplateMigrationReport,
    migrate_document_template_bindings, print_json_template_migration,
    print_text_template_migration,
};
