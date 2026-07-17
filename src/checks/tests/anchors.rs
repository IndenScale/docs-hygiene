#[test]
fn frontmatter_multi_anchors_validate_file_and_block_scopes_independently() {
    let temp = tempdir().unwrap();
    let library = temp.path().join("docs/ul");
    let body = temp.path().join("docs/prd");
    fs::create_dir_all(&library).unwrap();
    fs::create_dir_all(&body).unwrap();
    fs::write(
        library.join("manifest.yml"),
        "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [term-1.md, term-2.md]\n",
    )
    .unwrap();
    let term_one = "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term One\n\n## Watched\nkeep\n\n## Other\noutside\n";
    let term_two = "---\nid: TERM-2\nstatus: baselined\n---\n\n# Term Two\nwhole file\n";
    fs::write(library.join("term-1.md"), term_one).unwrap();
    fs::write(library.join("term-2.md"), term_two).unwrap();
    let block_hash = format!(
        "{:x}",
        Sha256::digest(markdown_heading_block(term_one, "watched").unwrap())
    );
    let file_hash = format!("{:x}", Sha256::digest(term_two.as_bytes()));
    fs::write(
        body.join("manifest.yml"),
        "id: PRD-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nmembers: [index.md]\n",
    )
    .unwrap();
    let body_text = format!(
        "---\nid: PRD-1-INDEX\nstatus: proposed\nanchors:\n  - target: TERM-1\n    algorithm: sha256\n    digest: {block_hash}\n    scope: block\n    locator: watched\n  - target: TERM-2\n    algorithm: sha256\n    digest: {file_hash}\n    scope: file\n---\n\n# Body\n"
    );
    fs::write(body.join("index.md"), &body_text).unwrap();
    let config: Config = serde_yaml::from_str(
        "governance:\n  manifests: [docs/ul/manifest.yml, docs/prd/manifest.yml]\n",
    )
    .unwrap();

    let clean = run_checks(temp.path(), &config).unwrap();
    assert!(clean.diagnostics.is_empty(), "{:?}", clean.diagnostics);
    let pinned = clean
        .governance_graph
        .edges
        .iter()
        .filter(|edge| edge.relation == GovernanceEdgeKind::PinnedReference)
        .collect::<Vec<_>>();
    assert_eq!(pinned.len(), 2);
    assert_eq!(pinned[0].target, "TERM-1");
    assert_eq!(pinned[0].source_location.line, Some(5));
    assert_eq!(pinned[0].selector.as_deref(), Some("watched"));
    let block_anchor = pinned[0].content_anchor.as_ref().unwrap();
    assert_eq!(block_anchor.scope, ContentAnchorScope::Block);
    assert_eq!(block_anchor.locator.as_deref(), Some("watched"));
    let block_json = serde_json::to_value(pinned[0]).unwrap();
    assert_eq!(block_json["contentAnchor"]["scope"], "block");
    assert_eq!(block_json["contentAnchor"]["locator"], "watched");
    assert_eq!(pinned[1].target, "TERM-2");
    assert_eq!(
        pinned[1].content_anchor.as_ref().unwrap().scope,
        ContentAnchorScope::File
    );
    let file_json = serde_json::to_value(pinned[1]).unwrap();
    assert!(file_json["contentAnchor"].get("scope").is_none());
    assert!(file_json["contentAnchor"].get("locator").is_none());
    let profile = crate::profile::evaluate_hygiene_profile(temp.path(), &config).unwrap();
    let dependency = profile
        .dimensions
        .iter()
        .find(|dimension| {
            dimension.dimension == crate::activation::CapabilityDimension::Dependency
        })
        .unwrap();
    assert!(dependency.evidence.iter().any(|evidence| {
        evidence.invariant == "dependency.scoped-anchor"
            && evidence.outcome == crate::profile::InvariantOutcome::Passed
    }));

    let outside_change = term_one.replace("outside", "outside changed");
    fs::write(library.join("term-1.md"), &outside_change).unwrap();
    let still_clean = run_checks(temp.path(), &config).unwrap();
    assert!(
        still_clean.diagnostics.is_empty(),
        "{:?}",
        still_clean.diagnostics
    );

    let watched_change = outside_change.replace("keep", "changed");
    fs::write(library.join("term-1.md"), watched_change).unwrap();
    let stale = run_checks(temp.path(), &config).unwrap();
    let diagnostic = stale
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.message.contains("Pinned block '#watched'"))
        .unwrap();
    assert_eq!(diagnostic.path, "docs/prd/index.md");
    assert_eq!(diagnostic.range.start.line, 4);
}

#[test]
fn invalid_frontmatter_anchors_are_reported_at_each_declaration() {
    let rel = Path::new("docs/body.md");
    let text = "---\nid: BODY-1\nstatus: proposed\nanchors:\n  - target: TERM-1\n    algorithm: sha256\n    digest: bad\n    scope: block\n  - target: TERM-2\n    algorithm: git\n    digest: bad\n    scope: commit\n---\n";
    let mut diagnostics = Vec::new();

    let occurrences = collect_frontmatter_occurrences(rel, text, &mut diagnostics);

    assert_eq!(occurrences.len(), 1);
    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].range.start.line, 4);
    assert_eq!(diagnostics[1].range.start.line, 8);
}

#[test]
fn commit_anchor_requires_opt_in_and_compares_the_governed_target_to_git() {
    let temp = tempdir().unwrap();
    let library = temp.path().join("docs/ul");
    let body = temp.path().join("docs/prd");
    fs::create_dir_all(&library).unwrap();
    fs::create_dir_all(&body).unwrap();
    let term = "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\ncommitted\n";
    fs::write(library.join("term.md"), term).unwrap();
    for args in [
        vec!["init", "-q"],
        vec!["config", "user.email", "docs-hygiene@example.invalid"],
        vec!["config", "user.name", "Docs Hygiene Test"],
        vec!["add", "docs/ul/term.md"],
        vec!["commit", "-q", "-m", "anchor target"],
    ] {
        assert!(
            Command::new("git")
                .args(args)
                .current_dir(temp.path())
                .status()
                .unwrap()
                .success()
        );
    }
    let commit = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert!(commit.status.success());
    let commit = String::from_utf8(commit.stdout).unwrap().trim().to_owned();

    fs::write(
        library.join("manifest.yml"),
        "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [term.md]\n",
    )
    .unwrap();
    fs::write(
        body.join("manifest.yml"),
        "id: PRD-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nmembers: [index.md]\n",
    )
    .unwrap();
    fs::write(
        body.join("index.md"),
        format!(
            "---\nid: PRD-1-INDEX\nstatus: proposed\nanchors:\n  - target: TERM-1\n    algorithm: git\n    digest: {commit}\n    scope: commit\n---\n\n# Body\n"
        ),
    )
    .unwrap();
    let enabled: Config = serde_yaml::from_str(
        "governance:\n  manifests: [docs/ul/manifest.yml, docs/prd/manifest.yml]\n  contentAnchors:\n    verifyGitCommits: true\n",
    )
    .unwrap();
    let disabled: Config = serde_yaml::from_str(
        "governance:\n  manifests: [docs/ul/manifest.yml, docs/prd/manifest.yml]\n",
    )
    .unwrap();

    let clean = run_checks(temp.path(), &enabled).unwrap();
    assert!(clean.diagnostics.is_empty(), "{:?}", clean.diagnostics);
    let not_opted_in = run_checks(temp.path(), &disabled).unwrap();
    assert!(not_opted_in.diagnostics.iter().any(|diagnostic| {
        diagnostic
            .message
            .contains("verifyGitCommits: true")
            && diagnostic.range.start.line == 4
    }));

    fs::write(library.join("term.md"), term.replace("committed", "changed")).unwrap();
    let changed = run_checks(temp.path(), &enabled).unwrap();
    assert!(changed.diagnostics.iter().any(|diagnostic| {
        diagnostic
            .message
            .contains("differs from Git object")
    }));
}
