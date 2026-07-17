use std::path::PathBuf;

use docs_hygiene::{
    Config, HygieneMaturity, InvariantOutcome, evaluate_hygiene_profile, run_checks,
};

#[test]
fn repository_policy_and_governance_graph_pass() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut config = Config::load(&root.join("docs-hygiene.yml")).unwrap();

    // External adapters have their own environment dependencies. This test
    // exercises the repository's complete native policy and governance graph.
    config.adapters.markdownlint.enabled = false;

    let report = run_checks(&root, &config).unwrap();
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
}

#[test]
fn repository_profile_meets_configured_targets() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut config = Config::load(&root.join("docs-hygiene.yml")).unwrap();
    config.adapters.markdownlint.enabled = false;

    let report = evaluate_hygiene_profile(&root, &config).unwrap();

    assert_eq!(report.dimensions[0].target, Some(HygieneMaturity::Governed));
    assert_eq!(
        report.dimensions[0].observed,
        Some(HygieneMaturity::Governed)
    );
    assert_eq!(
        report.dimensions[1].observed,
        Some(HygieneMaturity::Governed)
    );
    assert_eq!(
        report.dimensions[2].observed,
        Some(HygieneMaturity::Governed)
    );
    assert_eq!(
        report.dimensions[3].target,
        Some(HygieneMaturity::Controlled)
    );
    assert_eq!(
        report.dimensions[3].observed,
        Some(HygieneMaturity::Governed)
    );
    assert!(report.governance_graph.metrics.nodes > 0);
    assert!(report.governance_graph.metrics.edges > 0);
    assert!(report.governance_graph.metrics.cycle_groups.is_empty());
    assert!(report.governance_graph.edges.iter().any(|edge| {
        edge.source == "PRD-004"
            && edge.target == "DH-HYGIENE-PROFILE"
            && edge.selector.as_deref() == Some("documentation-hygiene-profile")
    }));
    let selector = report.dimensions[2]
        .evidence
        .iter()
        .find(|evidence| evidence.invariant == "dependency.selector")
        .unwrap();
    assert_eq!(selector.outcome, InvariantOutcome::Passed);
    let transitive_impact = report.dimensions[2]
        .evidence
        .iter()
        .find(|evidence| evidence.invariant == "dependency.transitive-impact")
        .unwrap();
    assert_eq!(transitive_impact.outcome, InvariantOutcome::Passed);
    assert_eq!(
        report.governance_graph.transitive_impact["DH-HYGIENE-PROFILE"],
        ["IMPL-002", "IMPL-003", "PRD-004", "SPEC-003"]
    );
    let lifecycle = report.dimensions[1]
        .evidence
        .iter()
        .find(|evidence| evidence.invariant == "identity.lifecycle")
        .unwrap();
    assert_eq!(lifecycle.outcome, InvariantOutcome::Passed);
    let authority_migration = report.dimensions[1]
        .evidence
        .iter()
        .find(|evidence| evidence.invariant == "identity.authority-migration")
        .unwrap();
    assert_eq!(authority_migration.outcome, InvariantOutcome::NotApplicable);
    assert!(report.governance_graph.authority_migrations.is_empty());
    assert!(report.document_templates.proves_reuse());
    assert!(report.document_templates.proves_migration());
    assert_eq!(report.document_templates.configured_templates, 1);
    assert_eq!(report.document_templates.configured_profiles, 5);
    assert_eq!(
        report.document_templates.bindings["maintained-open-contract"].len(),
        5
    );
    assert!(
        report
            .governance_graph
            .metrics
            .fan_in
            .values()
            .all(|degree| *degree <= 8)
    );
    assert!(
        report
            .governance_graph
            .metrics
            .fan_out
            .values()
            .all(|degree| *degree <= 12)
    );
    assert!(report.meets_targets);
}
