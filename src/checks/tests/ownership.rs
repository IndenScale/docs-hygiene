#[test]
fn ownership_policy_reports_complete_coverage_for_two_current_people() {
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("asset.yml"),
        r#"id: ASSET-1
refinementLevel: intent
referenceRelation: body
status: current
ownership:
  owner: person:alice
  understoodBy:
    - { principal: person:alice, confirmedAt: 2000-01-01 }
    - { principal: person:bob, confirmedAt: 2000-01-02 }
review:
  reviewBy: 2099-12-31
  lastReset:
    at: 2026-01-01
    by: person:bob
    reason: semantic review
"#,
    )
    .unwrap();
    let config: Config = serde_yaml::from_str(
        r#"governance:
  manifests: [asset.yml]
  ownership:
    enabled: true
    confirmationMaxAgeDays: 50000
    principals:
      - { id: "person:alice", kind: person }
      - { id: "person:bob", kind: person }
"#,
    )
    .unwrap();

    let report = run_checks(temp.path(), &config).unwrap();

    assert!(
        !report.diagnostics.iter().any(|diagnostic| matches!(
            diagnostic.code,
            "DH_OWNERSHIP_001" | "DH_REVIEW_001" | "DH_REVIEW_002" | "DH_KNOWLEDGE_001"
        )),
        "{:?}",
        report.diagnostics
    );
    assert!(report.ownership.enabled);
    assert_eq!(report.ownership.responsibility_coverage.percentage, 100);
    assert_eq!(report.ownership.review_coverage.percentage, 100);
    assert_eq!(
        report.ownership.knowledge_redundancy_coverage.percentage,
        100
    );
    assert_eq!(report.ownership.identities[0].knowledge_bus_factor, 2);
}

#[test]
fn expired_review_inactive_person_and_group_confirmation_reduce_coverage() {
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("asset.yml"),
        r#"id: ASSET-1
refinementLevel: intent
referenceRelation: body
status: baselined
ownership:
  owner: group:docs
  understoodBy:
    - { principal: person:alice, confirmedAt: 2000-01-01 }
    - { principal: person:bob, confirmedAt: 2000-01-02 }
    - { principal: group:docs, confirmedAt: 2000-01-03 }
review:
  reviewBy: 2000-01-01
"#,
    )
    .unwrap();
    fs::write(
        temp.path().join("archived.yml"),
        "id: ARCHIVED-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: archived\n",
    )
    .unwrap();
    let config: Config = serde_yaml::from_str(
        r#"governance:
  manifests: [asset.yml, archived.yml]
  ownership:
    enabled: true
    confirmationMaxAgeDays: 50000
    principals:
      - { id: "person:alice", kind: person }
      - { id: "person:bob", kind: person, status: inactive }
      - { id: "group:docs", kind: group, members: ["person:alice", "person:bob"] }
"#,
    )
    .unwrap();

    let report = run_checks(temp.path(), &config).unwrap();

    assert_eq!(report.ownership.identities.len(), 1);
    assert_eq!(report.ownership.responsibility_coverage.covered, 1);
    assert_eq!(report.ownership.reviews_expired, 1);
    assert_eq!(report.ownership.identities[0].knowledge_bus_factor, 1);
    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DH_REVIEW_001" && diagnostic.message.contains("expired")
    }));
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "DH_KNOWLEDGE_001"));
    assert!(!report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.path == "archived.yml"
            && matches!(
                diagnostic.code,
                "DH_OWNERSHIP_001" | "DH_REVIEW_001" | "DH_KNOWLEDGE_001"
            )));
}

#[test]
fn authority_successor_must_supply_fresh_ownership_review_and_understanding() {
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("old.yml"),
        "id: OLD\nrefinementLevel: intent\nreferenceRelation: body\nstatus: superseded\nsupersededBy: NEW\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("new.yml"),
        "id: NEW\nrefinementLevel: intent\nreferenceRelation: body\nstatus: current\n",
    )
    .unwrap();
    let config: Config = serde_yaml::from_str(
        r#"governance:
  manifests: [old.yml, new.yml]
  ownership:
    enabled: true
    principals:
      - { id: "person:alice", kind: person }
      - { id: "person:bob", kind: person }
"#,
    )
    .unwrap();

    let report = run_checks(temp.path(), &config).unwrap();

    assert_eq!(report.ownership.identities.len(), 1);
    assert_eq!(report.ownership.identities[0].identity, "NEW");
    for code in ["DH_OWNERSHIP_001", "DH_REVIEW_001", "DH_KNOWLEDGE_001"] {
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == code
                && diagnostic.path == "new.yml"
                && diagnostic.message.contains("NEW")
        }));
    }
}

#[test]
fn due_review_warns_and_expired_confirmation_immediately_reduces_bus_factor() {
    let temp = tempdir().unwrap();
    let (year, month, day) = today_utc().unwrap();
    let today = format!("{year:04}-{month:02}-{day:02}");
    fs::write(
        temp.path().join("asset.yml"),
        format!(
            r#"id: ASSET-1
refinementLevel: intent
referenceRelation: body
status: current
ownership:
  owner: person:alice
  understoodBy:
    - {{ principal: person:alice, confirmedAt: {today} }}
    - {{ principal: person:bob, confirmedAt: 2000-01-01 }}
review: {{ reviewBy: {today} }}
"#
        ),
    )
    .unwrap();
    let config: Config = serde_yaml::from_str(
        r#"governance:
  manifests: [asset.yml]
  ownership:
    enabled: true
    confirmationMaxAgeDays: 30
    reviewWarningDays: 30
    principals:
      - { id: "person:alice", kind: person }
      - { id: "person:bob", kind: person }
"#,
    )
    .unwrap();

    let report = run_checks(temp.path(), &config).unwrap();

    assert_eq!(report.ownership.reviews_due_soon, 1);
    assert_eq!(report.ownership.identities[0].review_state, ReviewState::DueSoon);
    assert_eq!(report.ownership.identities[0].valid_understanders, ["person:alice"]);
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "DH_REVIEW_002"));
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "DH_KNOWLEDGE_001"));
}

#[test]
fn ownership_policy_covers_assets_package_domains_and_markdown_leaves() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("lib/domain")).unwrap();
    let evidence = r#"ownership:
  owner: person:alice
  understoodBy:
    - { principal: person:alice, confirmedAt: 2000-01-01 }
    - { principal: person:bob, confirmedAt: 2000-01-02 }
review: { reviewBy: 2099-12-31 }
"#;
    fs::write(
        temp.path().join("lib/manifest.yml"),
        format!(
            "id: LIB-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [domain]\n{evidence}"
        ),
    )
    .unwrap();
    fs::write(
        temp.path().join("lib/domain/manifest.yml"),
        format!("id: DOMAIN-1\nkind: domain\nstatus: baselined\nmembers: [term.md]\n{evidence}"),
    )
    .unwrap();
    fs::write(
        temp.path().join("lib/domain/term.md"),
        format!("---\nid: TERM-1\nstatus: baselined\n{evidence}---\n\n# Term\n"),
    )
    .unwrap();
    let config: Config = serde_yaml::from_str(
        r#"governance:
  manifests: [lib/manifest.yml]
  ownership:
    enabled: true
    confirmationMaxAgeDays: 50000
    principals:
      - { id: "person:alice", kind: person }
      - { id: "person:bob", kind: person }
"#,
    )
    .unwrap();

    let report = run_checks(temp.path(), &config).unwrap();
    let identities = report
        .ownership
        .identities
        .iter()
        .map(|identity| identity.identity.as_str())
        .collect::<Vec<_>>();

    assert_eq!(identities, ["DOMAIN-1", "LIB-1", "TERM-1"]);
    assert_eq!(report.ownership.responsibility_coverage.percentage, 100);
    assert_eq!(report.ownership.review_coverage.percentage, 100);
    assert_eq!(report.ownership.knowledge_redundancy_coverage.percentage, 100);
}

#[test]
fn duplicate_principals_unexpanded_groups_and_duplicate_confirmations_are_rejected() {
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("asset.yml"),
        r#"id: ASSET-1
refinementLevel: intent
referenceRelation: body
status: current
ownership:
  owner: group:empty
  understoodBy:
    - { principal: person:alice, confirmedAt: 2000-01-01 }
    - { principal: person:alice, confirmedAt: 2000-01-02 }
review: { reviewBy: 2099-12-31 }
"#,
    )
    .unwrap();
    let config: Config = serde_yaml::from_str(
        r#"governance:
  manifests: [asset.yml]
  ownership:
    enabled: true
    confirmationMaxAgeDays: 50000
    principals:
      - { id: "person:alice", kind: person }
      - { id: "person:alice", kind: person }
      - { id: "group:empty", kind: group }
"#,
    )
    .unwrap();

    let report = run_checks(temp.path(), &config).unwrap();

    assert!(
        report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "DH_OWNERSHIP_001")
            .count()
            >= 3
    );
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "DH_KNOWLEDGE_001"));
    assert_eq!(report.ownership.responsibility_coverage.covered, 0);
    assert_eq!(report.ownership.identities[0].knowledge_bus_factor, 1);
}

#[test]
fn ownership_policy_without_governance_manifests_cannot_prove_empty_coverage() {
    let temp = tempdir().unwrap();
    let config: Config = serde_yaml::from_str(
        r#"governance:
  ownership:
    enabled: true
    principals:
      - { id: "person:alice", kind: person }
      - { id: "person:bob", kind: person }
"#,
    )
    .unwrap();

    let report = run_checks(temp.path(), &config).unwrap();

    assert!(report.ownership.enabled);
    assert_eq!(report.ownership.responsibility_coverage.total, 0);
    for code in ["DH_OWNERSHIP_001", "DH_REVIEW_001", "DH_KNOWLEDGE_001"] {
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == code && diagnostic.path == "docs-hygiene.yml"
        }));
    }
}
