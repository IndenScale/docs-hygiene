    #[test]
    fn reports_missing_and_non_library_body_references() {
        let temp = tempdir().unwrap();
        let manifests = [
            "ul.yml",
            "prd-missing/manifest.yml",
            "issue-wrong/manifest.yml",
        ];
        write_asset(
            temp.path(),
            "ul.yml",
            "id: UL-1\nreferenceRelation: library\nstatus: baselined\n",
        );
        write_asset(
            temp.path(),
            "prd-missing/manifest.yml",
            "id: PRD-1\nreferenceRelation: body\nstatus: proposed\nmembers: [index.md]\n",
        );
        fs::write(
            temp.path().join("prd-missing/index.md"),
            "---\nid: PRD-1-INDEX\nstatus: proposed\n---\n\n# PRD\n",
        )
        .unwrap();
        write_asset(
            temp.path(),
            "issue-wrong/manifest.yml",
            "id: ISSUE-1\nreferenceRelation: body\nstatus: proposed\nmembers: [index.md]\n",
        );
        fs::write(
            temp.path().join("issue-wrong/index.md"),
            "---\nid: ISSUE-1-INDEX\nstatus: proposed\n---\n\n# Issue\n\n[[PRD-1]]\n",
        )
        .unwrap();

        let report = run_checks(temp.path(), &governance_config(&manifests, false)).unwrap();
        let references = report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "DH_REFERENCE_001")
            .collect::<Vec<_>>();

        assert_eq!(references.len(), 2, "{:?}", report.diagnostics);
        assert!(
            references
                .iter()
                .any(|diagnostic| diagnostic.path == "prd-missing/manifest.yml")
        );
        assert!(
            references
                .iter()
                .any(|diagnostic| diagnostic.path == "issue-wrong/manifest.yml")
        );
    }

    #[test]
    fn validates_wiki_link_content_hash_and_localized_parity() {
        let temp = tempdir().unwrap();
        let library = temp.path().join("docs/ul");
        fs::create_dir_all(&library).unwrap();
        fs::write(
            library.join("manifest.yml"),
            "id: UL-1\nreferenceRelation: library\nstatus: baselined\nmembers: [term.md]\n",
        )
        .unwrap();
        let term = "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n";
        fs::write(library.join("term.md"), term).unwrap();
        let hash = format!("{:x}", Sha256::digest(term.as_bytes()));

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
                format!(
                    "---\nid: PRD-1-INDEX\nstatus: proposed\n---\n\n# Body\n\n[[TERM-1#term@sha256:{hash}|Term]]\n"
                ),
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
                .all(|diagnostic| diagnostic.code != "DH_REFERENCE_001"),
            "{:?}",
            clean.diagnostics
        );
        let pinned = clean
            .governance_graph
            .edges
            .iter()
            .find(|edge| edge.relation == GovernanceEdgeKind::PinnedReference)
            .unwrap();
        assert_eq!(pinned.source, "PRD-1");
        assert_eq!(pinned.target, "TERM-1");
        assert_eq!(pinned.source_location.path, "docs/prd/example/index.md");
        assert_eq!(pinned.source_location.line, Some(8));
        assert_eq!(pinned.selector.as_deref(), Some("term"));
        assert_eq!(
            pinned.content_anchor.as_ref().map(|anchor| anchor.digest.as_str()),
            Some(hash.as_str())
        );
        assert_eq!(pinned.lifecycle.source_status, "proposed");
        assert_eq!(
            pinned.lifecycle.target_status.as_deref(),
            Some("baselined")
        );

        fs::write(library.join("term.md"), format!("{term}\nChanged.\n")).unwrap();
        let changed = run_checks(temp.path(), &config).unwrap();
        assert!(changed.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_REFERENCE_001"
                && diagnostic.message.contains("TERM-1")
                && diagnostic.message.contains("changed")
        }));

        fs::write(library.join("term.md"), term).unwrap();
        fs::write(
            temp.path().join("docs/zh/prd/example/index.md"),
            "---\nid: PRD-1-INDEX\nstatus: proposed\n---\n\n# Body\n\n[[TERM-1@sha256:bad]]\n",
        )
        .unwrap();
        let localized_drift = run_checks(temp.path(), &config).unwrap();
        assert!(localized_drift.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_REFERENCE_001"
                && diagnostic.message.contains("Invalid semantic Wiki Link")
        }));
        assert!(localized_drift.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_REFERENCE_001" && diagnostic.message.contains("Localized Body")
        }));
    }

    #[test]
    fn rejects_removed_version_and_manifest_reference_metadata() {
        let temp = tempdir().unwrap();
        write_asset(
            temp.path(),
            "ul.yml",
            "id: UL-1\nversion: 1.0.0\nreferenceRelation: library\nstatus: baselined\n",
        );
        write_asset(
            temp.path(),
            "prd.yml",
            "id: PRD-1\nreferenceRelation: body\nstatus: proposed\nreferences: UL-1\n",
        );

        let report = run_checks(
            temp.path(),
            &governance_config(&["ul.yml", "prd.yml"], false),
        )
        .unwrap();

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_GOVERNANCE_001"
                && diagnostic
                    .message
                    .contains("version fields are not supported")
        }));
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_GOVERNANCE_001"
                && diagnostic.message.contains("Manifest-level 'references'")
        }));
    }
