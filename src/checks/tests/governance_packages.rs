    fn governance_config(manifests: &[&str], require_complete: bool) -> Config {
        let manifest_lines = manifests
            .iter()
            .map(|path| format!("    - {path}"))
            .collect::<Vec<_>>()
            .join("\n");
        serde_yaml::from_str(&format!(
            "governance:\n  manifests:\n{manifest_lines}\n  requireCompleteVerticalDerivation: {require_complete}\n"
        ))
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
    fn accepts_complete_horizontal_and_vertical_governance_graph() {
        let temp = tempdir().unwrap();
        let manifests = [
            "ul.yml",
            "prd/manifest.yml",
            "glossary.yml",
            "spec/manifest.yml",
            "sdk.yml",
            "impl.yml",
        ];
        write_asset(
            temp.path(),
            "ul.yml",
            "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\n",
        );
        write_asset(
            temp.path(),
            "prd/manifest.yml",
            "id: PRD-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: baselined\nmembers: [index.md]\n",
        );
        fs::write(
            temp.path().join("prd/index.md"),
            "---\nid: PRD-1-INDEX\nstatus: baselined\n---\n\n# PRD\n\n[[UL-1]]\n[Navigation only](../ul.yml)\n",
        )
        .unwrap();
        write_asset(
            temp.path(),
            "glossary.yml",
            "id: GLOSSARY-1\nrefinementLevel: definition\nreferenceRelation: library\nstatus: baselined\nprojects: { id: UL-1 }\n",
        );
        fs::write(
            temp.path().join("glossary-term.md"),
            "---\nid: TERM-glossary\nstatus: baselined\n---\n\n# Term\n\n[[UL-1]]\n",
        )
        .unwrap();
        write_asset(
            temp.path(),
            "spec/manifest.yml",
            "id: SPEC-1\nrefinementLevel: definition\nreferenceRelation: body\nstatus: baselined\nformalizes: PRD-1\nmembers: [index.md]\n",
        );
        fs::write(
            temp.path().join("spec/index.md"),
            "---\nid: SPEC-1-INDEX\nstatus: baselined\n---\n\n# Spec\n\n[[GLOSSARY-1]]\n",
        )
        .unwrap();
        write_asset(
            temp.path(),
            "sdk.yml",
            "id: SDK-1\nrefinementLevel: implementation\nreferenceRelation: library\nstatus: current\nprojects: GLOSSARY-1\n",
        );
        fs::write(
            temp.path().join("sdk-term.md"),
            "---\nid: TERM-sdk\nstatus: baselined\n---\n\n# Term\n\n[[GLOSSARY-1]]\n",
        )
        .unwrap();
        fs::create_dir_all(temp.path().join("src")).unwrap();
        fs::write(
            temp.path().join("src/main.rs"),
            "// [[SDK-1]]\nconst EXAMPLE: &str = \"[[NOT-A-DEPENDENCY]]\";\nfn main() {}\n",
        )
        .unwrap();
        write_asset(
            temp.path(),
            "impl.yml",
            "id: IMPL-1\nrefinementLevel: implementation\nreferenceRelation: body\nstatus: current\nrealizes: SPEC-1\nmembers:\n  code: [src/main.rs]\n",
        );

        let report = run_checks(temp.path(), &governance_config(&manifests, true)).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert_eq!(report.governance_graph.metrics.nodes, 6);
        assert_eq!(report.governance_graph.metrics.edges, 7);
        assert_eq!(report.governance_graph.metrics.resolved_edges, 7);
        assert_eq!(report.governance_graph.metrics.unresolved_edges, 0);
        assert_eq!(report.governance_graph.metrics.isolated_nodes, 0);
        assert_eq!(
            report
                .governance_graph
                .metrics
                .relation_counts
                .get(&GovernanceEdgeKind::SemanticReference),
            Some(&3)
        );
        assert!(report.governance_graph.edges.iter().any(|edge| {
            edge.source == "SPEC-1"
                && edge.target == "PRD-1"
                && edge.relation == GovernanceEdgeKind::Formalizes
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
    fn validates_declared_implementation_body_members() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("src")).unwrap();
        fs::write(temp.path().join("src/main.rs"), "fn main() {}\n").unwrap();
        write_asset(
            temp.path(),
            "impl-valid.yml",
            "id: IMPL-VALID\nrefinementLevel: implementation\nreferenceRelation: body\nstatus: current\nrealizes: { id: SPEC-1 }\nmembers:\n  code: [src/main.rs]\n",
        );
        write_asset(
            temp.path(),
            "impl-invalid.yml",
            "id: IMPL-INVALID\nrefinementLevel: implementation\nreferenceRelation: body\nstatus: current\nrealizes: { id: SPEC-1 }\nmembers:\n  code: [src/missing.rs, ../escaped.rs, src/main.rs]\n  configuration: [src/main.rs]\n",
        );

        let config = governance_config(&["impl-valid.yml", "impl-invalid.yml"], false);
        let report = run_checks(temp.path(), &config).unwrap();
        let member_diagnostics = report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "DH_BODY_001")
            .collect::<Vec<_>>();

        assert_eq!(member_diagnostics.len(), 3, "{:?}", report.diagnostics);
        assert!(member_diagnostics.iter().any(|diagnostic| {
            diagnostic.message.contains("src/missing.rs")
                && diagnostic.message.contains("does not exist")
        }));
        assert!(member_diagnostics.iter().any(|diagnostic| {
            diagnostic.message.contains("../escaped.rs")
                && diagnostic.message.contains("without traversal")
        }));
        assert!(member_diagnostics.iter().any(|diagnostic| {
            diagnostic.message.contains("src/main.rs")
                && diagnostic.message.contains("more than once")
        }));
        assert!(
            member_diagnostics
                .iter()
                .all(|diagnostic| diagnostic.path == "impl-invalid.yml")
        );
    }

    #[test]
    fn validates_library_directory_members_and_term_frontmatter() {
        let temp = tempdir().unwrap();
        let library = temp.path().join("docs/intent/ul");
        fs::create_dir_all(&library).unwrap();
        fs::write(
            library.join("manifest.yml"),
            "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [declared.md, ../escaped.md]\n",
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
            &governance_config(&["docs/intent/ul/manifest.yml"], false),
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

        let localized = temp.path().join("docs/zh/intent/ul");
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
      root: docs/intent/ul
      localizedRoots:
        zh: docs/zh/intent/ul
      patterns:
        - id: term
          regex: "^[a-z0-9-]+\\.md$"
governance:
  manifests: [docs/intent/ul/manifest.yml]
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
        for root in ["docs/intent/prd/example", "docs/zh/intent/prd/example"] {
            let package = temp.path().join(root);
            fs::create_dir_all(package.join("stories")).unwrap();
            fs::write(
                package.join("manifest.yml"),
                "id: PRD-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nmembers: [stories]\n",
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
      root: docs/intent/prd/example
      localizedRoots:
        zh: docs/zh/intent/prd/example
      patterns:
        - id: item
          regex: "^[a-z0-9-]+\\.md$"
governance:
  manifests: [docs/intent/prd/example/manifest.yml]
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
                .join("docs/zh/intent/prd/example/stories/manifest.yml"),
            "id: PRD-1-STORIES\nkind: domain\nstatus: proposed\nmembers: []\n",
        )
        .unwrap();
        let drifted = run_checks(temp.path(), &config).unwrap();
        assert!(drifted.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_BODY_001" && diagnostic.message.contains("direct members")
        }));

        fs::write(
            temp.path()
                .join("docs/zh/intent/prd/example/stories/manifest.yml"),
            "id: PRD-1-STORIES\nkind: domain\nstatus: proposed\nmembers: [story.md]\n",
        )
        .unwrap();
        let extra = temp.path().join("docs/zh/intent/prd/example/extra");
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
