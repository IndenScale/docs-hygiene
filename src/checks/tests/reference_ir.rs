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
            "lifecycle": { "sourceStatus": "proposed" }
        })
    );
}
