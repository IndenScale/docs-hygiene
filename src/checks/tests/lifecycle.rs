#[test]
fn validates_leaf_authority_migration_and_rejects_stale_consumers() {
    let temp = tempdir().unwrap();
    let library = temp.path().join("docs/ul");
    let body = temp.path().join("docs/prd");
    fs::create_dir_all(&library).unwrap();
    fs::create_dir_all(&body).unwrap();
    fs::write(
        library.join("manifest.yml"),
        "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [old.md, new.md]\n",
    )
    .unwrap();
    fs::write(
        library.join("old.md"),
        "---\nid: TERM-OLD\nstatus: superseded\nsupersededBy: TERM-NEW\n---\n\n# Old\n",
    )
    .unwrap();
    fs::write(
        library.join("new.md"),
        "---\nid: TERM-NEW\nstatus: baselined\n---\n\n# New\n",
    )
    .unwrap();
    fs::write(
        body.join("manifest.yml"),
        "id: PRD-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nmembers: [index.md]\n",
    )
    .unwrap();
    fs::write(
        body.join("index.md"),
        "---\nid: PRD-1-INDEX\nstatus: proposed\n---\n\n# Body\n\n[[TERM-NEW]]\n",
    )
    .unwrap();
    let config = governance_config(
        &["docs/ul/manifest.yml", "docs/prd/manifest.yml"],
        false,
    );

    let clean = run_checks(temp.path(), &config).unwrap();
    assert!(clean.diagnostics.is_empty(), "{:?}", clean.diagnostics);
    assert_eq!(
        clean.governance_graph.authority_migrations["TERM-OLD"],
        "TERM-NEW"
    );
    let profile = crate::profile::evaluate_hygiene_profile(temp.path(), &config).unwrap();
    let identity = profile
        .dimensions
        .iter()
        .find(|dimension| {
            dimension.dimension == crate::activation::CapabilityDimension::Identity
        })
        .unwrap();
    assert!(identity.evidence.iter().any(|evidence| {
        evidence.invariant == "identity.lifecycle"
            && evidence.outcome == crate::profile::InvariantOutcome::Passed
    }));
    assert!(identity.evidence.iter().any(|evidence| {
        evidence.invariant == "identity.authority-migration"
            && evidence.outcome == crate::profile::InvariantOutcome::Passed
    }));

    fs::write(
        body.join("index.md"),
        "---\nid: PRD-1-INDEX\nstatus: proposed\n---\n\n# Body\n\n[[TERM-OLD]]\n",
    )
    .unwrap();
    let stale = run_checks(temp.path(), &config).unwrap();
    let diagnostic = stale
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.message.contains("migrate the dependency to 'TERM-NEW'"))
        .unwrap();
    assert_eq!(diagnostic.path, "docs/prd/index.md");
    assert_eq!(diagnostic.range.start.line, 7);
}

#[test]
fn lifecycle_status_obligations_reject_missing_misplaced_and_unready_successors() {
    let temp = tempdir().unwrap();
    write_asset(
        temp.path(),
        "missing.yml",
        "id: OLD-MISSING\nrefinementLevel: intent\nreferenceRelation: library\nstatus: superseded\n",
    );
    write_asset(
        temp.path(),
        "misplaced.yml",
        "id: ACTIVE\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nsupersededBy: DRAFT\n",
    );
    write_asset(
        temp.path(),
        "old.yml",
        "id: OLD\nrefinementLevel: intent\nreferenceRelation: library\nstatus: superseded\nsupersededBy: DRAFT\n",
    );
    write_asset(
        temp.path(),
        "draft.yml",
        "id: DRAFT\nrefinementLevel: intent\nreferenceRelation: library\nstatus: draft\n",
    );
    let config = governance_config(
        &["missing.yml", "misplaced.yml", "old.yml", "draft.yml"],
        false,
    );

    let report = run_checks(temp.path(), &config).unwrap();
    let lifecycle = report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "DH_GOVERNANCE_001"
                && (diagnostic.message.contains("supersededBy")
                    || diagnostic.message.contains("baselined or current"))
        })
        .collect::<Vec<_>>();

    assert_eq!(lifecycle.len(), 3, "{:?}", report.diagnostics);
}
