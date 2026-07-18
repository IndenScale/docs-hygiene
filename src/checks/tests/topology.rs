    use crate::governance::ReferenceResolution;

    fn topology_node(identity: &str) -> GovernanceNode {
        GovernanceNode {
            identity: identity.to_owned(),
            reference_relation: ReferenceRelation::Body,
            document_kind: None,
            lifecycle_status: "current".to_owned(),
            location: GovernanceLocation {
                path: format!("{identity}.yml"),
                line: None,
            },
        }
    }

    fn topology_edge(source: &str, target: &str) -> GovernanceEdge {
        GovernanceEdge {
            source: source.to_owned(),
            target: target.to_owned(),
            relation: GovernanceEdgeKind::SemanticReference,
            source_location: GovernanceLocation {
                path: format!("{source}.md"),
                line: Some(1),
            },
            selector: None,
            content_anchor: None,
            lifecycle: LifecycleProvenance {
                source_status: "current".to_owned(),
                target_status: Some("current".to_owned()),
            },
            expectation: ReferenceExpectation::new(
                GovernanceEdgeKind::SemanticReference,
                vec![ReferenceRelation::Body],
                Vec::new(),
            ),
            resolution: ReferenceResolution::unresolved(),
        }
    }

    #[test]
    fn topology_policy_reports_fan_and_cycle_violations() {
        let graph = GovernanceGraph::new(
            vec![topology_node("A"), topology_node("B"), topology_node("C")],
            vec![
                topology_edge("A", "B"),
                topology_edge("A", "C"),
                topology_edge("B", "A"),
                topology_edge("C", "A"),
            ],
        );
        let config: Config = serde_yaml::from_str(
            "governance:\n  topology:\n    maxFanIn: 1\n    maxFanOut: 1\n    forbidCycles: true\n",
        )
        .unwrap();
        let mut diagnostics = Vec::new();

        let exceptions = check_topology_policy(&config, &graph, &mut diagnostics);
        assert!(exceptions.is_empty());

        assert_eq!(
            diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.code == "DH_TOPOLOGY_001")
                .count(),
            2
        );
        assert_eq!(
            diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.code == "DH_TOPOLOGY_002")
                .count(),
            1
        );
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.message.contains("Fan-In 2")
                && diagnostic.message.contains("maxFanIn 1")
        }));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.message.contains("Fan-Out 2")
                && diagnostic.message.contains("maxFanOut 1")
        }));
    }

    #[test]
    fn community_baseline_is_reported_advisory_until_explicitly_enforced() {
        let mut graph = GovernanceGraph::new(
            vec![topology_node("A"), topology_node("B")],
            vec![topology_edge("A", "B")],
        );
        let advisory: Config = serde_yaml::from_str(
            "governance:\n  topology:\n    communityBaseline:\n      A: community:A\n      B: community:A\n",
        )
        .unwrap();
        graph.compare_community_baseline(&advisory.governance.topology.community_baseline);
        assert_eq!(graph.community_changes.len(), 1);
        let mut diagnostics = Vec::new();
        check_topology_policy(&advisory, &graph, &mut diagnostics);
        assert!(diagnostics.is_empty());

        let enforced: Config = serde_yaml::from_str(
            "governance:\n  topology:\n    communityBaseline:\n      A: community:A\n      B: community:A\n    enforceCommunityBaseline: true\n",
        )
        .unwrap();
        check_topology_policy(&enforced, &graph, &mut diagnostics);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "DH_TOPOLOGY_006");
    }

    #[test]
    fn audited_supernode_exception_is_visible_and_only_relaxes_one_node_direction() {
        let graph = GovernanceGraph::new(
            vec![
                topology_node("A"),
                topology_node("B"),
                topology_node("C"),
                topology_node("X"),
            ],
            vec![
                topology_edge("A", "X"),
                topology_edge("B", "X"),
                topology_edge("C", "X"),
                topology_edge("A", "B"),
                topology_edge("A", "C"),
            ],
        );
        let config: Config = serde_yaml::from_str(
            r#"
governance:
  topology:
    maxFanIn: 1
    maxFanOut: 1
    exceptions:
      - id: public-x
        node: X
        direction: fanIn
        budget: 4
        reason: shared public contract
        owner: platform-docs
        approvedBy: architecture-council
        expires: 2099-12-31
        history:
          - observedAt: 2026-01-01
            degree: 2
"#,
        )
        .unwrap();
        let mut diagnostics = Vec::new();

        let exceptions = check_topology_policy(&config, &graph, &mut diagnostics);

        assert_eq!(
            diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.code == "DH_TOPOLOGY_001")
                .count(),
            1,
            "{diagnostics:?}"
        );
        assert!(diagnostics[0].message.contains("'A' has Fan-Out 3"));
        let exception = &exceptions[0];
        assert_eq!(exception.status, TopologyExceptionStatus::Applied);
        assert_eq!(exception.current_degree, Some(3));
        assert_eq!(exception.remaining, Some(1));
        assert_eq!(exception.trend_delta, Some(1));
        assert_eq!(exception.transitive_impact, vec!["A", "B", "C"]);
    }

    #[test]
    fn expired_missing_history_and_idle_supernode_exceptions_are_deterministic() {
        let graph = GovernanceGraph::new(
            vec![topology_node("A"), topology_node("B"), topology_node("X")],
            vec![topology_edge("A", "X"), topology_edge("B", "X")],
        );
        let expired: Config = serde_yaml::from_str(
            r#"
governance:
  topology:
    maxFanIn: 1
    exceptions:
      - id: expired-x
        node: X
        direction: fanIn
        budget: 3
        reason: old approval
        owner: docs
        approvedBy: council
        expires: 2000-01-01
        history: [{ observedAt: 1999-01-01, degree: 2 }]
"#,
        )
        .unwrap();
        let mut diagnostics = Vec::new();
        let evidence = check_topology_policy(&expired, &graph, &mut diagnostics);
        assert_eq!(evidence[0].status, TopologyExceptionStatus::Expired);
        assert!(diagnostics.iter().any(|item| item.code == "DH_TOPOLOGY_003"));
        assert!(diagnostics.iter().any(|item| item.code == "DH_TOPOLOGY_001"));

        let missing_history: Config = serde_yaml::from_str(
            r#"
governance:
  topology:
    maxFanIn: 1
    exceptions:
      - id: no-history-x
        node: X
        direction: fanIn
        budget: 3
        reason: needs evidence
        owner: docs
        approvedBy: council
        expires: 2099-12-31
"#,
        )
        .unwrap();
        diagnostics.clear();
        let evidence = check_topology_policy(&missing_history, &graph, &mut diagnostics);
        assert_eq!(evidence[0].status, TopologyExceptionStatus::Invalid);
        assert!(diagnostics.iter().any(|item| item.code == "DH_TOPOLOGY_005"));
        assert!(diagnostics.iter().any(|item| item.code == "DH_TOPOLOGY_001"));

        let idle_graph = GovernanceGraph::new(
            vec![topology_node("A"), topology_node("X")],
            vec![topology_edge("A", "X")],
        );
        diagnostics.clear();
        let evidence = check_topology_policy(&missing_history, &idle_graph, &mut diagnostics);
        assert_eq!(evidence[0].status, TopologyExceptionStatus::Idle);
        assert!(diagnostics.iter().any(|item| item.code == "DH_TOPOLOGY_004"));
        assert!(!diagnostics.iter().any(|item| item.code == "DH_TOPOLOGY_001"));

        let invalid_metadata: Config = serde_yaml::from_str(
            r#"
governance:
  topology:
    maxFanIn: 1
    exceptions:
      - id: missing-metadata
        node: X
        direction: fanIn
        budget: 3
        reason: ""
        owner: ""
        approvedBy: ""
        expires: ""
      - id: missing-target
        node: MISSING
        direction: fanIn
        budget: 3
        reason: no such node
        owner: docs
        approvedBy: council
        expires: 2099-12-31
"#,
        )
        .unwrap();
        diagnostics.clear();
        let evidence = check_topology_policy(&invalid_metadata, &graph, &mut diagnostics);
        assert!(
            evidence
                .iter()
                .all(|item| item.status == TopologyExceptionStatus::Invalid)
        );
        assert_eq!(
            diagnostics
                .iter()
                .filter(|item| item.code == "DH_TOPOLOGY_003")
                .count(),
            2
        );
    }

    #[test]
    fn configured_fan_threshold_runs_on_normalized_repository_edges() {
        let temp = tempdir().unwrap();
        write_asset(
            temp.path(),
            "ul.yml",
            "id: UL-1\nreferenceRelation: library\nstatus: baselined\n",
        );
        for name in ["a", "b"] {
            write_asset(
                temp.path(),
                &format!("{name}/manifest.yml"),
                &format!(
                    "id: PRD-{name}\nreferenceRelation: body\nstatus: proposed\nmembers: [index.md]\n"
                ),
            );
            fs::write(
                temp.path().join(format!("{name}/index.md")),
                format!("---\nid: PRD-{name}-INDEX\nstatus: proposed\n---\n\n# Body\n\n[[UL-1]]\n"),
            )
            .unwrap();
        }
        let config: Config = serde_yaml::from_str(
            r#"
governance:
  manifests: [ul.yml, a/manifest.yml, b/manifest.yml]
  topology:
    maxFanIn: 1
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();

        let topology = report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "DH_TOPOLOGY_001")
            .collect::<Vec<_>>();
        assert_eq!(topology.len(), 1, "{:?}", report.diagnostics);
        assert!(topology[0].message.contains("'UL-1' has Fan-In 2"));
        assert_eq!(report.governance_graph.metrics.fan_in["UL-1"], 2);
    }
