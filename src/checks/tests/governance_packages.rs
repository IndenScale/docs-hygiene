    fn governance_config(manifests: &[&str], _require_complete: bool) -> Config {
        let manifest_lines = manifests
            .iter()
            .map(|path| format!("    - {path}"))
            .collect::<Vec<_>>()
            .join("\n");
        serde_yaml::from_str(&format!("governance:\n  manifests:\n{manifest_lines}\n"))
        .unwrap()
    }

    fn write_asset(root: &Path, path: &str, yaml: &str) {
        let asset_path = root.join(path);
        if let Some(parent) = asset_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        if yaml.contains("referenceRelation: library") && !yaml.contains("members:") {
            let stem = Path::new(path)
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap();
            let term = format!("{stem}-term.md");
            fs::write(
                asset_path.parent().unwrap().join(&term),
                format!("---\nid: TERM-{stem}\nstatus: baselined\n---\n\n# Term\n"),
            )
            .unwrap();
            fs::write(asset_path, format!("{yaml}members: [{term}]\n")).unwrap();
        } else {
            fs::write(asset_path, yaml).unwrap();
        }
    }

    #[test]
    fn accepts_ul_and_prd_governance_graph_without_refinement_layers() {
        let temp = tempdir().unwrap();
        let manifests = ["ul.yml", "prd/manifest.yml"];
        write_asset(
            temp.path(),
            "ul.yml",
            "id: UL-1\nreferenceRelation: library\nstatus: baselined\n",
        );
        write_asset(
            temp.path(),
            "prd/manifest.yml",
            "id: PRD-1\nreferenceRelation: body\nstatus: baselined\nmembers: [index.md]\n",
        );
        fs::write(
            temp.path().join("prd/index.md"),
            "---\nid: PRD-1-INDEX\nstatus: baselined\n---\n\n# PRD\n\n[[UL-1]]\n[Navigation only](../ul.yml)\n",
        )
        .unwrap();
        let report = run_checks(temp.path(), &governance_config(&manifests, true)).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert_eq!(report.governance_graph.metrics.nodes, 2);
        assert_eq!(report.governance_graph.metrics.edges, 1);
        assert_eq!(report.governance_graph.metrics.resolved_edges, 1);
        assert_eq!(report.governance_graph.metrics.unresolved_edges, 0);
        assert_eq!(report.governance_graph.metrics.isolated_nodes, 0);
        assert_eq!(
            report
                .governance_graph
                .metrics
                .relation_counts
                .get(&GovernanceEdgeKind::SemanticReference),
            Some(&1)
        );
        assert!(report.governance_graph.edges.iter().any(|edge| {
            edge.source == "PRD-1"
                && edge.target == "UL-1"
                && edge.relation == GovernanceEdgeKind::SemanticReference
        }));
        assert!(!report
            .governance_graph
            .edges
            .iter()
            .any(|edge| edge.target.contains("ul.yml")));
        assert!(!report
            .governance_graph
            .edges
            .iter()
            .any(|edge| edge.target == "NOT-A-DEPENDENCY"));
    }

    #[test]
    fn rejects_legacy_refinement_and_projection_metadata() {
        let temp = tempdir().unwrap();
        write_asset(
            temp.path(),
            "legacy.yml",
            "id: SPEC-1\nrefinementLevel: definition\nreferenceRelation: body\nstatus: baselined\nformalizes: PRD-1\n",
        );

        let config = governance_config(&["legacy.yml"], false);
        let report = run_checks(temp.path(), &config).unwrap();
        let legacy = report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "DH_GOVERNANCE_001")
            .collect::<Vec<_>>();

        assert_eq!(legacy.len(), 1, "{:?}", report.diagnostics);
        assert!(legacy[0].message.contains("Legacy refinement metadata"));
    }

    #[test]
    fn validates_library_directory_members_and_term_frontmatter() {
        let temp = tempdir().unwrap();
        let library = temp.path().join("docs/engineering/ul");
        fs::create_dir_all(&library).unwrap();
        fs::write(
            library.join("manifest.yml"),
            "id: UL-1\nreferenceRelation: library\nstatus: baselined\nmembers: [declared.md, ../escaped.md]\n",
        )
        .unwrap();
        fs::write(
            library.join("declared.md"),
            "---\nid: TERM-1\nstatus: unknown\n---\n\n# Declared\n",
        )
        .unwrap();
        fs::write(
            library.join("orphan.md"),
            "---\nid: TERM-2\nstatus: proposed\n---\n\n# Orphan\n",
        )
        .unwrap();

        let report = run_checks(
            temp.path(),
            &governance_config(&["docs/engineering/ul/manifest.yml"], false),
        )
        .unwrap();
        let members = report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "DH_LIBRARY_001")
            .collect::<Vec<_>>();

        assert_eq!(members.len(), 3, "{:?}", report.diagnostics);
        assert!(members.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("must name one direct child without traversal")
        }));
        assert!(
            members
                .iter()
                .any(|diagnostic| diagnostic.message.contains("invalid lifecycle status"))
        );
        assert!(
            members
                .iter()
                .any(|diagnostic| diagnostic.message.contains("not declared"))
        );

        let localized = temp.path().join("docs/zh/engineering/ul");
        fs::create_dir_all(&localized).unwrap();
        fs::write(
            localized.join("declared.md"),
            "---\nid: DIFFERENT-TERM\nstatus: unknown\n---\n\n# 术语\n",
        )
        .unwrap();
        let config: Config = serde_yaml::from_str(
            r#"
docs:
  bases:
    - id: ul
      root: docs/engineering/ul
      localizedRoots:
        zh: docs/zh/engineering/ul
      patterns:
        - id: term
          regex: "^[a-z0-9-]+\\.md$"
governance:
  manifests: [docs/engineering/ul/manifest.yml]
"#,
        )
        .unwrap();
        let localized_report = run_checks(temp.path(), &config).unwrap();
        assert!(localized_report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_LIBRARY_001"
                && diagnostic
                    .message
                    .contains("must preserve canonical id, status, supersededBy, kind, and direct members")
        }));
    }

    #[test]
    fn validates_recursive_body_package_and_localized_domain_members() {
        let temp = tempdir().unwrap();
        for root in ["docs/engineering/prd/example", "docs/zh/engineering/prd/example"] {
            let package = temp.path().join(root);
            fs::create_dir_all(package.join("stories")).unwrap();
            fs::write(
                package.join("manifest.yml"),
                "id: PRD-1\nreferenceRelation: body\nstatus: proposed\nmembers: [stories]\n",
            )
            .unwrap();
            fs::write(
                package.join("stories/manifest.yml"),
                "id: PRD-1-STORIES\nkind: domain\nstatus: proposed\nmembers: [story.md]\n",
            )
            .unwrap();
            fs::write(
                package.join("stories/story.md"),
                "---\nid: PRD-1-STORY-1\nstatus: proposed\n---\n\n# Story\n",
            )
            .unwrap();
        }
        let config: Config = serde_yaml::from_str(
            r#"
docs:
  bases:
    - id: prd
      root: docs/engineering/prd/example
      localizedRoots:
        zh: docs/zh/engineering/prd/example
      patterns:
        - id: item
          regex: "^[a-z0-9-]+\\.md$"
governance:
  manifests: [docs/engineering/prd/example/manifest.yml]
"#,
        )
        .unwrap();

        let clean = run_checks(temp.path(), &config).unwrap();
        assert!(
            clean
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "DH_BODY_001"),
            "{:?}",
            clean.diagnostics
        );

        fs::write(
            temp.path()
                .join("docs/zh/engineering/prd/example/stories/manifest.yml"),
            "id: PRD-1-STORIES\nkind: domain\nstatus: proposed\nmembers: []\n",
        )
        .unwrap();
        let drifted = run_checks(temp.path(), &config).unwrap();
        assert!(drifted.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_BODY_001" && diagnostic.message.contains("direct members")
        }));

        fs::write(
            temp.path()
                .join("docs/zh/engineering/prd/example/stories/manifest.yml"),
            "id: PRD-1-STORIES\nkind: domain\nstatus: proposed\nmembers: [story.md]\n",
        )
        .unwrap();
        let extra = temp.path().join("docs/zh/engineering/prd/example/extra");
        fs::create_dir_all(&extra).unwrap();
        fs::write(
            extra.join("manifest.yml"),
            "id: EXTRA\nkind: domain\nstatus: proposed\nmembers: []\n",
        )
        .unwrap();
        let extra_report = run_checks(temp.path(), &config).unwrap();
        assert!(extra_report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_BODY_001"
                && diagnostic.message.contains("contains an extra node")
        }));
    }
