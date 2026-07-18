#[test]
fn collectors_emit_one_versioned_reference_ir_and_explicit_policies() {
    let rel = Path::new("docs/body.md");
    let text = "---\nid: BODY-1\n---\n\n# Body\n\n[Guide](../README.md)\n[[TERM-1#term]]\n";
    let mut diagnostics = Vec::new();

    let wiki = collect_wiki_link_occurrences(rel, text, true, &mut diagnostics);
    let markdown = collect_markdown_link_occurrences(rel, text);
    let frontmatter = collect_frontmatter_occurrences(rel, text, &mut diagnostics);

    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    assert_eq!(wiki.len(), 1);
    assert_eq!(markdown.len(), 1);
    assert_eq!(frontmatter.len(), 1);
    for occurrence in wiki.iter().chain(&markdown).chain(&frontmatter) {
        assert_eq!(
            occurrence.schema_version,
            crate::reference::REFERENCE_OCCURRENCE_SCHEMA_VERSION
        );
    }
    let wiki = wiki.first().unwrap();
    assert_eq!(wiki.raw_target, "TERM-1");
    assert_eq!(wiki.payload.selector.as_deref(), Some("term"));
    assert_eq!(
        reference_disposition(wiki, REFERENCE_POLICIES),
        Some(ReferenceDisposition::SemanticDependency)
    );
    assert_eq!(
        reference_disposition(markdown.first().unwrap(), REFERENCE_POLICIES),
        Some(ReferenceDisposition::NavigationOnly)
    );
    assert_eq!(
        reference_disposition(frontmatter.first().unwrap(), REFERENCE_POLICIES),
        Some(ReferenceDisposition::IdentityDeclaration)
    );
}

#[test]
fn governance_target_lists_do_not_alias_the_second_id_as_a_document_kind() {
    let targets: GovernanceTargets = serde_yaml::from_str("- SPEC-001\n- SPEC-002\n").unwrap();
    let targets = targets.iter().collect::<Vec<_>>();
    assert_eq!(targets.len(), 2);
    assert_eq!(targets[0].id, "SPEC-001");
    assert_eq!(targets[1].id, "SPEC-002");
    assert!(targets.iter().all(|target| target.document_kind.is_none()));

    let typed: GovernanceTargets =
        serde_yaml::from_str("id: SPEC-001\ndocumentKind: specification\n").unwrap();
    let typed = typed.iter().next().unwrap();
    assert_eq!(typed.id, "SPEC-001");
    assert_eq!(typed.document_kind.as_deref(), Some("specification"));
}

#[test]
fn a_fourth_syntax_connects_through_collector_output_and_policy_only() {
    fn collect_hypothetical_reference(rel: &Path, target: &str) -> ReferenceOccurrence {
        ReferenceOccurrence::new(
            target,
            "hypotheticalSyntax",
            CONTEXT_GOVERNED_CONTENT,
            GovernanceLocation {
                path: rel.display().to_string(),
                line: Some(3),
            },
            ReferencePayload::default(),
        )
    }

    const POLICIES: &[ReferencePolicy] = &[ReferencePolicy {
        syntax: "hypotheticalSyntax",
        context: CONTEXT_GOVERNED_CONTENT,
        disposition: ReferenceDisposition::SemanticDependency,
    }];
    let asset = GovernanceAsset {
        id: "PRD-1".to_owned(),
        refinement_level: RefinementLevel::Intent,
        reference_relation: ReferenceRelation::Body,
        status: "proposed".to_owned(),
        superseded_by: None,
        formalizes: GovernanceTargets::default(),
        realizes: GovernanceTargets::default(),
        projects: GovernanceTargets::default(),
        members: None,
        path: "prd.yml".to_owned(),
    };
    let occurrences = BTreeSet::from([collect_hypothetical_reference(
        Path::new("docs/prd.md"),
        "TERM-1",
    )]);

    let edges = normalize_reference_edges_with_policies(
        &asset,
        &occurrences,
        &BTreeMap::new(),
        POLICIES,
    );

    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].source, "PRD-1");
    assert_eq!(edges[0].target, "TERM-1");
    assert_eq!(edges[0].relation, GovernanceEdgeKind::SemanticReference);
    assert_eq!(edges[0].source_location.path, "docs/prd.md");
    assert_eq!(
        serde_json::to_value(&edges[0]).unwrap(),
        serde_json::json!({
            "source": "PRD-1",
            "target": "TERM-1",
            "relation": "semanticReference",
            "sourceLocation": { "path": "docs/prd.md", "line": 3 },
            "lifecycle": { "sourceStatus": "proposed" },
            "expectation": {
                "relation": "semanticReference",
                "endpoint": {
                    "refinementLevels": ["intent"],
                    "referenceRelations": ["library"]
                }
            },
            "resolution": {
                "outcome": "unresolved",
                "incompatibilities": ["missingTarget"]
            }
        })
    );
}

#[test]
fn normalization_preserves_expected_kind_and_explicit_resolution_outcomes() {
    let asset = GovernanceAsset {
        id: "PRD-1".to_owned(),
        refinement_level: RefinementLevel::Intent,
        reference_relation: ReferenceRelation::Body,
        status: "current".to_owned(),
        superseded_by: None,
        formalizes: GovernanceTargets::default(),
        realizes: GovernanceTargets::default(),
        projects: GovernanceTargets::default(),
        members: None,
        path: "prd.yml".to_owned(),
    };
    let occurrence = ReferenceOccurrence::new(
        "TERM-1",
        SYNTAX_FRONTMATTER,
        CONTEXT_GOVERNED_ANCHOR,
        GovernanceLocation {
            path: "body.md".to_owned(),
            line: Some(5),
        },
        ReferencePayload {
            selector: None,
            anchor: Some(ReferenceAnchorPayload {
                algorithm: "sha256".to_owned(),
                digest: "0".repeat(64),
                scope: ContentAnchorScope::File,
                locator: None,
                expected_document_kind: Some("term".to_owned()),
                updated_at: None,
                updated_by: None,
                reason: None,
                snapshot: None,
            }),
        },
    );
    let alternate = SemanticTarget {
        refinement_level: RefinementLevel::Intent,
        reference_relation: ReferenceRelation::Library,
        status: "current".to_owned(),
        superseded_by: None,
        path: "other.md".to_owned(),
        document_kind: Some("term".to_owned()),
        alternates: Vec::new(),
    };
    let incompatible_target = SemanticTarget {
        refinement_level: RefinementLevel::Intent,
        reference_relation: ReferenceRelation::Library,
        status: "current".to_owned(),
        superseded_by: None,
        path: "term.md".to_owned(),
        document_kind: Some("article".to_owned()),
        alternates: Vec::new(),
    };
    let occurrences = BTreeSet::from([occurrence]);
    let incompatible = normalize_reference_edges(
        &asset,
        &occurrences,
        &BTreeMap::from([("TERM-1".to_owned(), incompatible_target.clone())]),
    );
    assert_eq!(
        incompatible[0].expectation.endpoint.document_kinds,
        vec!["term"]
    );
    assert_eq!(
        incompatible[0].resolution.outcome,
        ReferenceResolutionOutcome::Incompatible
    );
    assert_eq!(
        incompatible[0].resolution.incompatibilities,
        vec![ReferenceCompatibilityIssue::DocumentKind]
    );

    let mut ambiguous_target = incompatible_target;
    ambiguous_target.document_kind = Some("term".to_owned());
    ambiguous_target.alternates.push(alternate);
    let ambiguous = normalize_reference_edges(
        &asset,
        &occurrences,
        &BTreeMap::from([("TERM-1".to_owned(), ambiguous_target)]),
    );
    assert_eq!(
        ambiguous[0].resolution.outcome,
        ReferenceResolutionOutcome::Ambiguous
    );
    assert_eq!(ambiguous[0].resolution.endpoints.len(), 2);
}
