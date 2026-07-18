use std::fs;

use tempfile::tempdir;

use super::*;

#[test]
fn rule_registry_is_ordered_unique_and_complete_for_compatibility_families() {
    let ids = RULE_SPECS.iter().map(|spec| spec.id).collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "project.entry-docs",
            "docs.structure",
            "documents.contracts",
            "localization.parity",
            "concepts.references",
            "governance.identity",
            "governance.traceability",
            "governance.topology",
            "adapters.external",
        ]
    );
    let unique = ids
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(unique.len(), RULE_SPECS.len());
    let diagnostic_codes = RULE_SPECS
        .iter()
        .flat_map(|spec| spec.diagnostic_codes.iter().copied())
        .collect::<Vec<_>>();
    let unique_diagnostic_codes = diagnostic_codes
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(unique_diagnostic_codes.len(), diagnostic_codes.len());
    assert!(RULE_SPECS.iter().all(|spec| {
        !spec.diagnostic_codes.is_empty()
            && !spec.rationale.is_empty()
            && !spec.remediation.is_empty()
            && spec.default_mode == RuleMode::Auto
    }));
    assert!(RULE_SPECS.iter().any(|spec| spec.capabilities.len() > 1));
    assert_eq!(
        rule_spec_for_diagnostic("DH_PIN_001").unwrap().checker,
        RuleChecker::GovernanceTraceability
    );
}

#[test]
fn scale_signals_are_advisory_and_deterministic() {
    let temp = tempdir().unwrap();
    for index in 0..20 {
        fs::write(
            temp.path().join(format!("doc-{index:02}.md")),
            format!("# Document {index}\n"),
        )
        .unwrap();
    }
    let config = Config::default();

    let first = evaluate_rule_activation(temp.path(), &config).unwrap();
    let second = evaluate_rule_activation(temp.path(), &config).unwrap();

    assert_eq!(
        serde_json::to_value(&first).unwrap(),
        serde_json::to_value(&second).unwrap()
    );
    assert_eq!(first.decision("docs.structure").state, RuleState::Advisory);
    assert_eq!(
        first.decision("documents.contracts").state,
        RuleState::Advisory
    );
    assert!(first.decisions.iter().all(|decision| {
        !decision.evidence.is_empty()
            && !decision.rationale.is_empty()
            && !decision.remediation.is_empty()
            && (decision.state != RuleState::Error || decision.mode == RuleMode::Required)
    }));
}

#[test]
fn structural_signals_strengthen_decisions() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("docs/zh")).unwrap();
    fs::write(
        temp.path().join("docs/zh/overview.md"),
        "---\nid: OVERVIEW\nstatus: draft\n---\n\n# 概览\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("docs/manifest.yml"),
        "id: PRD-1\nreferenceRelation: body\nstatus: draft\n",
    )
    .unwrap();
    let report = evaluate_rule_activation(temp.path(), &Config::default()).unwrap();

    assert_eq!(
        report.decision("localization.parity").state,
        RuleState::Warning
    );
    assert_eq!(
        report.decision("governance.identity").state,
        RuleState::Warning
    );
    assert_eq!(
        report.decision("governance.traceability").state,
        RuleState::Inactive
    );
}

#[test]
fn automatic_states_are_monotonic_across_stronger_evidence() {
    let temp = tempdir().unwrap();
    let empty = evaluate_rule_activation(temp.path(), &Config::default()).unwrap();
    let initial = empty.decision("docs.structure").state;

    for index in 0..20 {
        fs::write(
            temp.path().join(format!("doc-{index:02}.md")),
            format!("# Document {index}\n"),
        )
        .unwrap();
    }
    let scale = evaluate_rule_activation(temp.path(), &Config::default()).unwrap();
    let advisory = scale.decision("docs.structure").state;

    let configured: Config = serde_yaml::from_str(
        r#"
docs:
  bases:
    - id: main
      root: docs
"#,
    )
    .unwrap();
    let policy = evaluate_rule_activation(temp.path(), &configured).unwrap();
    let enforced = policy.decision("docs.structure").state;

    assert_eq!(initial, RuleState::Inactive);
    assert_eq!(advisory, RuleState::Advisory);
    assert_eq!(enforced, RuleState::Error);
    assert!(initial < advisory && advisory < enforced);
}

#[test]
fn explicit_modes_override_automatic_inference() {
    let temp = tempdir().unwrap();
    let config: Config = serde_yaml::from_str(
        r#"
rules:
  docs.structure:
    mode: required
  project.entry-docs:
    mode: disabled
entryDocs:
  required: [README.md]
"#,
    )
    .unwrap();

    let report = evaluate_rule_activation(temp.path(), &config).unwrap();

    assert_eq!(report.decision("docs.structure").state, RuleState::Error);
    assert_eq!(
        report.decision("project.entry-docs").state,
        RuleState::Inactive
    );
}

#[test]
fn unknown_rule_ids_are_rejected() {
    let temp = tempdir().unwrap();
    let config: Config = serde_yaml::from_str("rules:\n  unknown.rule:\n    mode: auto\n").unwrap();

    let error = evaluate_rule_activation(temp.path(), &config).unwrap_err();

    assert!(error.to_string().contains("unknown rule id 'unknown.rule'"));
}

#[test]
fn topology_policy_activates_only_from_explicit_configuration() {
    let temp = tempdir().unwrap();
    let implicit = evaluate_rule_activation(temp.path(), &Config::default()).unwrap();
    assert_eq!(
        implicit.decision("governance.topology").state,
        RuleState::Inactive
    );

    let configured: Config =
        serde_yaml::from_str("governance:\n  topology:\n    maxFanIn: 8\n    forbidCycles: true\n")
            .unwrap();
    let explicit = evaluate_rule_activation(temp.path(), &configured).unwrap();
    assert_eq!(
        explicit.decision("governance.topology").state,
        RuleState::Error
    );
    assert_eq!(explicit.facts.configured_topology_policies, 2);
}
