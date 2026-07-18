    #[test]
    fn selectors_resolve_headings_enter_edges_and_preserve_localized_signatures() {
        let temp = tempdir().unwrap();
        let library = temp.path().join("docs/ul");
        fs::create_dir_all(&library).unwrap();
        fs::write(
            library.join("manifest.yml"),
            "id: UL-1\nreferenceRelation: library\nstatus: baselined\nmembers: [term.md]\n",
        )
        .unwrap();
        fs::write(
            library.join("term.md"),
            "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n\n## Decision `Scope`!\n\nDetails.\n",
        )
        .unwrap();
        for package in ["docs/prd/example", "docs/zh/prd/example"] {
            let package = temp.path().join(package);
            fs::create_dir_all(&package).unwrap();
            fs::write(
                package.join("manifest.yml"),
                "id: PRD-1\nreferenceRelation: body\nstatus: proposed\nmembers: [index.md]\n",
            )
            .unwrap();
            fs::write(
                package.join("index.md"),
                "---\nid: PRD-1-INDEX\nstatus: proposed\n---\n\n# Body\n\n[[TERM-1#decision-scope|scope]]\n",
            )
            .unwrap();
        }
        let config: Config = serde_yaml::from_str(
            r#"
docs:
  bases:
    - id: body
      root: docs/prd
      localizedRoots:
        zh: docs/zh/prd
      patterns:
        - id: item
          regex: "^[a-z0-9-]+\\.md$"
governance:
  manifests: [docs/ul/manifest.yml, docs/prd/example/manifest.yml]
"#,
        )
        .unwrap();

        let clean = run_checks(temp.path(), &config).unwrap();
        assert!(
            clean
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "DH_SELECTOR_001"),
            "{:?}",
            clean.diagnostics
        );
        let selected = clean
            .governance_graph
            .edges
            .iter()
            .find(|edge| edge.target == "TERM-1")
            .unwrap();
        assert_eq!(selected.selector.as_deref(), Some("decision-scope"));

        fs::write(
            temp.path().join("docs/zh/prd/example/index.md"),
            "---\nid: PRD-1-INDEX\nstatus: proposed\n---\n\n# Body\n\n[[TERM-1#other-scope|scope]]\n",
        )
        .unwrap();
        let localized_drift = run_checks(temp.path(), &config).unwrap();
        assert!(localized_drift.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_REFERENCE_001"
                && diagnostic.message.contains("targets, selectors")
        }));

        for path in [
            "docs/prd/example/index.md",
            "docs/zh/prd/example/index.md",
        ] {
            fs::write(
                temp.path().join(path),
                "---\nid: PRD-1-INDEX\nstatus: proposed\n---\n\n# Body\n\n[[TERM-1#missing-scope|scope]]\n",
            )
            .unwrap();
        }
        let missing = run_checks(temp.path(), &config).unwrap();
        let diagnostic = missing
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "DH_SELECTOR_001")
            .unwrap();
        assert_eq!(diagnostic.path, "docs/prd/example/index.md");
        assert_eq!(diagnostic.range.start.line, 7);
        assert!(diagnostic.message.contains("#missing-scope"));
        assert_eq!(diagnostic.related_information[0].path, "docs/ul/term.md");

        fs::write(
            library.join("term.md"),
            "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n\n## Decision Scope\n\nOne.\n\n## Decision `Scope`!\n\nTwo.\n",
        )
        .unwrap();
        for path in [
            "docs/prd/example/index.md",
            "docs/zh/prd/example/index.md",
        ] {
            fs::write(
                temp.path().join(path),
                "---\nid: PRD-1-INDEX\nstatus: proposed\n---\n\n# Body\n\n[[TERM-1#decision-scope|scope]]\n",
            )
            .unwrap();
        }
        let ambiguous = run_checks(temp.path(), &config).unwrap();
        assert!(ambiguous.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_SELECTOR_001" && diagnostic.message.contains("ambiguous")
        }));
    }
