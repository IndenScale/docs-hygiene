use std::fs;

use tempfile::tempdir;

use super::*;

#[test]
fn legacy_maturity_maps_only_to_structure_target() {
    let temp = tempdir().unwrap();
    fs::create_dir(temp.path().join("docs")).unwrap();
    fs::write(temp.path().join("docs/01_overview.md"), "# Overview\n").unwrap();
    let config: Config = serde_yaml::from_str(
        "docs:\n  root: docs\ndocumentContracts:\n  maturity:\n    declared: maintained\n",
    )
    .unwrap();

    let report = evaluate_hygiene_profile(temp.path(), &config).unwrap();
    let structure = &report.dimensions[0];

    assert_eq!(structure.target, Some(HygieneMaturity::Controlled));
    assert!(structure.required);
    assert_eq!(report.dimensions[1].target, None);
}

#[test]
fn suppression_is_unverified_evidence_not_a_pass() {
    let temp = tempdir().unwrap();
    let config: Config = serde_yaml::from_str(
        r#"
entryDocs:
  required: [README.md]
suppressions:
  - code: DH_REQUIRED_001
    paths: [README.md]
    reason: migration
"#,
    )
    .unwrap();

    let report = evaluate_hygiene_profile(temp.path(), &config).unwrap();
    let evidence = report.dimensions[0]
        .evidence
        .iter()
        .find(|item| item.invariant == "structure.entry-docs")
        .unwrap();

    assert_eq!(evidence.outcome, InvariantOutcome::Unverified);
    assert_eq!(evidence.paths, vec!["README.md"]);
    assert_eq!(evidence.suppression_reasons, vec!["migration"]);
    assert_eq!(report.dimensions[0].observed, None);
}

#[test]
fn not_applicable_requires_a_rationale_and_is_excluded() {
    let temp = tempdir().unwrap();
    let invalid: Config = serde_yaml::from_str(
        "hygieneProfile:\n  dimensions:\n    topology:\n      applicability: notApplicable\n",
    )
    .unwrap();
    assert!(
        evaluate_hygiene_profile(temp.path(), &invalid)
            .unwrap_err()
            .to_string()
            .contains("requires a rationale")
    );

    let valid: Config = serde_yaml::from_str(
        "hygieneProfile:\n  dimensions:\n    topology:\n      applicability: notApplicable\n      rationale: No governed graph.\n",
    )
    .unwrap();
    let report = evaluate_hygiene_profile(temp.path(), &valid).unwrap();
    assert_eq!(report.dimensions[3].status, DimensionStatus::NotApplicable);
}

#[test]
fn conflicting_legacy_and_new_structure_targets_are_rejected() {
    let temp = tempdir().unwrap();
    let config: Config = serde_yaml::from_str(
        r#"
documentContracts:
  maturity:
    declared: maintained
hygieneProfile:
  dimensions:
    structure:
      target: governed
"#,
    )
    .unwrap();

    assert!(
        evaluate_hygiene_profile(temp.path(), &config)
            .unwrap_err()
            .to_string()
            .contains("conflicts with legacy")
    );
}

#[test]
fn topology_controlled_requires_explicit_policy_not_graph_presence_alone() {
    let temp = tempdir().unwrap();
    fs::create_dir(temp.path().join("prd")).unwrap();
    fs::write(
        temp.path().join("ul.yml"),
        "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [term.md]\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("term.md"),
        "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("prd/manifest.yml"),
        "id: PRD-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nmembers: [index.md]\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("prd/index.md"),
        "---\nid: PRD-1-INDEX\nstatus: proposed\n---\n\n# Body\n\n[[UL-1]]\n",
    )
    .unwrap();
    let without_policy: Config = serde_yaml::from_str(
        r#"
governance:
  manifests: [ul.yml, prd/manifest.yml]
hygieneProfile:
  dimensions:
    topology: { target: controlled, required: true }
"#,
    )
    .unwrap();

    let basic = evaluate_hygiene_profile(temp.path(), &without_policy).unwrap();
    assert_eq!(basic.dimensions[3].observed, Some(HygieneMaturity::Basic));
    assert_eq!(basic.dimensions[3].status, DimensionStatus::BelowTarget);

    let with_policy: Config = serde_yaml::from_str(
        r#"
governance:
  manifests: [ul.yml, prd/manifest.yml]
  topology: { maxFanIn: 8, forbidCycles: true }
hygieneProfile:
  dimensions:
    topology: { target: controlled, required: true }
"#,
    )
    .unwrap();
    let controlled = evaluate_hygiene_profile(temp.path(), &with_policy).unwrap();
    assert_eq!(
        controlled.dimensions[3].observed,
        Some(HygieneMaturity::Governed)
    );
    assert_eq!(
        controlled.dimensions[3].status,
        DimensionStatus::MeetsTarget
    );

    let suppressed_policy: Config = serde_yaml::from_str(
        r#"
governance:
  manifests: [ul.yml, prd/manifest.yml]
  topology: { maxFanIn: 0 }
hygieneProfile:
  dimensions:
    topology: { target: controlled, required: true }
suppressions:
  - code: DH_TOPOLOGY_001
    paths: [ul.yml]
    reason: temporary topology migration
"#,
    )
    .unwrap();
    let suppressed = evaluate_hygiene_profile(temp.path(), &suppressed_policy).unwrap();
    let threshold = suppressed.dimensions[3]
        .evidence
        .iter()
        .find(|evidence| evidence.invariant == "topology.thresholds")
        .unwrap();
    assert_eq!(threshold.outcome, InvariantOutcome::Unverified);
    assert_eq!(
        threshold.suppression_reasons,
        vec!["temporary topology migration"]
    );
    assert_eq!(
        suppressed.dimensions[3].observed,
        Some(HygieneMaturity::Basic)
    );

    let audited_policy: Config = serde_yaml::from_str(
        r#"
governance:
  manifests: [ul.yml, prd/manifest.yml]
  topology:
    maxFanIn: 0
    exceptions:
      - id: shared-library
        node: UL-1
        direction: fanIn
        budget: 2
        reason: intentional public Library
        owner: docs-platform
        approvedBy: architecture-council
        expires: 2099-12-31
        history: [{ observedAt: 2026-01-01, degree: 1 }]
hygieneProfile:
  dimensions:
    topology: { target: controlled, required: true }
"#,
    )
    .unwrap();
    let audited = evaluate_hygiene_profile(temp.path(), &audited_policy).unwrap();
    let threshold = audited.dimensions[3]
        .evidence
        .iter()
        .find(|evidence| evidence.invariant == "topology.thresholds")
        .unwrap();
    assert_eq!(threshold.outcome, InvariantOutcome::Excepted);
    assert_eq!(threshold.exception_ids, vec!["shared-library"]);
    assert_eq!(
        audited.topology_exceptions[0].status,
        crate::report::TopologyExceptionStatus::Applied
    );
    assert_eq!(audited.dimensions[3].observed, Some(HygieneMaturity::Basic));
    assert!(!audited.meets_targets);
}

#[test]
fn invariant_registry_ids_are_unique_and_ordered_by_dimension() {
    let ids = INVARIANTS
        .iter()
        .map(|invariant| invariant.id)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(ids.len(), INVARIANTS.len());
    assert_eq!(INVARIANTS[0].id, "structure.entry-docs");
    assert_eq!(INVARIANTS.last().unwrap().id, "topology.trends");
}
