    fn topology_node(identity: &str) -> GovernanceNode {
        GovernanceNode {
            identity: identity.to_owned(),
            refinement_level: RefinementLevel::Intent,
            reference_relation: ReferenceRelation::Body,
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

        check_topology_policy(&config, &graph, &mut diagnostics);

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
    fn configured_fan_threshold_runs_on_normalized_repository_edges() {
        let temp = tempdir().unwrap();
        write_asset(
            temp.path(),
            "ul.yml",
            "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\n",
        );
        for name in ["a", "b"] {
            write_asset(
                temp.path(),
                &format!("{name}/manifest.yml"),
                &format!(
                    "id: PRD-{name}\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nmembers: [index.md]\n"
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
