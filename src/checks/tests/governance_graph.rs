    #[test]
    fn reports_missing_and_cross_refinement_level_horizontal_references() {
        let temp = tempdir().unwrap();
        let manifests = [
            "ul.yml",
            "prd-missing/manifest.yml",
            "spec-wrong/manifest.yml",
        ];
        write_asset(
            temp.path(),
            "ul.yml",
            "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\n",
        );
        write_asset(
            temp.path(),
            "prd-missing/manifest.yml",
            "id: PRD-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nmembers: [index.md]\n",
        );
        fs::write(
            temp.path().join("prd-missing/index.md"),
            "---\nid: PRD-1-INDEX\nstatus: proposed\n---\n\n# PRD\n",
        )
        .unwrap();
        write_asset(
            temp.path(),
            "spec-wrong/manifest.yml",
            "id: SPEC-1\nrefinementLevel: definition\nreferenceRelation: body\nstatus: proposed\nformalizes: PRD-1\nmembers: [index.md]\n",
        );
        fs::write(
            temp.path().join("spec-wrong/index.md"),
            "---\nid: SPEC-1-INDEX\nstatus: proposed\n---\n\n# Spec\n\n[[UL-1]]\n",
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
                .any(|diagnostic| diagnostic.path == "spec-wrong/manifest.yml")
        );
    }

    #[test]
    fn validates_wiki_link_content_hash_and_localized_parity() {
        let temp = tempdir().unwrap();
        let library = temp.path().join("docs/ul");
        fs::create_dir_all(&library).unwrap();
        fs::write(
            library.join("manifest.yml"),
            "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [term.md]\n",
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
                "id: PRD-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nmembers: [index.md]\n",
            )
            .unwrap();
            fs::write(
                package.join("index.md"),
                format!(
                    "---\nid: PRD-1-INDEX\nstatus: proposed\n---\n\n# Body\n\n[[TERM-1@sha256:{hash}|Term]]\n"
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
            "id: UL-1\nversion: 1.0.0\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\n",
        );
        write_asset(
            temp.path(),
            "prd.yml",
            "id: PRD-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nreferences: UL-1\n",
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

    #[test]
    fn reports_missing_and_invalid_vertical_body_derivation() {
        let temp = tempdir().unwrap();
        let manifests = ["ul.yml", "glossary.yml", "spec.yml", "impl.yml"];
        write_asset(
            temp.path(),
            "ul.yml",
            "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\n",
        );
        write_asset(
            temp.path(),
            "glossary.yml",
            "id: GLOSSARY-1\nrefinementLevel: definition\nreferenceRelation: library\nstatus: baselined\nprojects: { id: UL-1 }\n",
        );
        write_asset(
            temp.path(),
            "spec.yml",
            "id: SPEC-1\nrefinementLevel: definition\nreferenceRelation: body\nstatus: proposed\n",
        );
        write_asset(
            temp.path(),
            "impl.yml",
            "id: IMPL-1\nrefinementLevel: implementation\nreferenceRelation: body\nstatus: current\nrealizes: { id: GLOSSARY-1 }\n",
        );

        let report = run_checks(temp.path(), &governance_config(&manifests, false)).unwrap();
        let derivations = report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "DH_DERIVATION_001")
            .collect::<Vec<_>>();

        assert_eq!(derivations.len(), 2, "{:?}", report.diagnostics);
        assert!(
            derivations
                .iter()
                .any(|diagnostic| diagnostic.path == "spec.yml")
        );
        assert!(
            derivations
                .iter()
                .any(|diagnostic| diagnostic.path == "impl.yml")
        );
    }

    #[test]
    fn reports_library_projection_and_reverse_completeness_gaps() {
        let temp = tempdir().unwrap();
        let manifests = ["ul.yml", "prd.yml", "glossary.yml"];
        write_asset(
            temp.path(),
            "ul.yml",
            "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\n",
        );
        write_asset(
            temp.path(),
            "prd.yml",
            "id: PRD-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: baselined\n",
        );
        write_asset(
            temp.path(),
            "glossary.yml",
            "id: GLOSSARY-1\nrefinementLevel: definition\nreferenceRelation: library\nstatus: baselined\n",
        );

        let report = run_checks(temp.path(), &governance_config(&manifests, true)).unwrap();

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_DERIVATION_002" && diagnostic.path == "glossary.yml"
        }));
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_DERIVATION_001" && diagnostic.path == "prd.yml"
        }));
    }
