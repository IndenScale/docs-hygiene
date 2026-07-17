    fn critical_pin_fixture(body: &str, requirement: &str) -> (tempfile::TempDir, Config) {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/ul")).unwrap();
        fs::create_dir_all(temp.path().join("docs/prd")).unwrap();
        fs::write(
            temp.path().join("docs/ul/manifest.yml"),
            "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [term.md, other.md]\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("docs/ul/term.md"),
            "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n\n## Contract\n\nCritical contract bytes.\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("docs/ul/other.md"),
            "---\nid: TERM-2\nstatus: baselined\n---\n\n# Other\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("docs/prd/manifest.yml"),
            "id: BODY-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nmembers: [body.md]\n",
        )
        .unwrap();
        fs::write(temp.path().join("docs/prd/body.md"), body).unwrap();
        let config: Config = serde_yaml::from_str(&format!(
            r#"
governance:
  manifests: [docs/ul/manifest.yml, docs/prd/manifest.yml]
  criticalDependencies:
    - id: critical-term
      match:
        sourceKinds: [body]
        targetKinds: [library]
        relations: [references]
        sourcePaths: [docs/prd/*.md]
        sourceIds: [BODY-1]
        targetIds: [TERM-1]
      require:
{requirement}
rules:
  governance.identity:
    mode: required
  governance.traceability:
    mode: required
"#
        ))
        .unwrap();
        (temp, config)
    }

    #[test]
    fn critical_pin_policy_has_stable_missing_scope_algorithm_and_stale_diagnostics() {
        let base = "---\nid: BODY-ITEM\nstatus: proposed\n---\n\n# Body\n\n";
        let cases = [
            (
                format!("{base}[[TERM-1]]\n"),
                "        algorithms: [sha256]\n        minimumScope: file",
                "DH_PIN_001",
            ),
            (
                format!("{base}[[TERM-1@sha256:{}]]\n", "0".repeat(64)),
                "        algorithms: [sha256]\n        minimumScope: block\n        forbidWholeFile: true",
                "DH_PIN_002",
            ),
            (
                format!("{base}[[TERM-1@sha256:{}]]\n", "0".repeat(64)),
                "        algorithms: [git]\n        minimumScope: file",
                "DH_PIN_003",
            ),
            (
                format!("{base}[[TERM-1@sha256:{}]]\n", "0".repeat(64)),
                "        algorithms: [sha256]\n        minimumScope: file",
                "DH_PIN_004",
            ),
            (
                "---\nid: BODY-ITEM\nstatus: proposed\nanchors:\n  - target: TERM-1\n    algorithm: sha256\n    digest: invalid\n    scope: file\n---\n\n# Body\n\n[[TERM-1]]\n"
                    .to_owned(),
                "        algorithms: [sha256]\n        minimumScope: file",
                "DH_PIN_006",
            ),
        ];
        for (body, requirement, expected) in cases {
            let (temp, config) = critical_pin_fixture(&body, requirement);
            let report = run_checks(temp.path(), &config).unwrap();
            let diagnostic = report
                .diagnostics
                .iter()
                .find(|diagnostic| diagnostic.code == expected)
                .unwrap_or_else(|| panic!("missing {expected}: {:?}", report.diagnostics));
            assert!(diagnostic.message.contains("Direct dependent: 'BODY-1'"));
            assert!(diagnostic.message.contains("reverse transitive impact: [BODY-1]"));
        }
    }

    #[test]
    fn critical_pin_age_is_enforced_while_noncritical_reference_stays_unpinned() {
        let term = "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n\n## Contract\n\nCritical contract bytes.\n";
        let digest = format!(
            "{:x}",
            Sha256::digest(markdown_heading_block(term, "contract").unwrap())
        );
        let body = format!(
            "---\nid: BODY-ITEM\nstatus: proposed\nanchors:\n  - target: TERM-1\n    algorithm: sha256\n    digest: {digest}\n    scope: block\n    locator: contract\n    updatedAt: 2000-01-01\n    updatedBy: maintainer\n    reason: initial review\n---\n\n# Body\n\n[[TERM-2]]\n"
        );
        let (temp, config) = critical_pin_fixture(
            &body,
            "        algorithms: [sha256]\n        minimumScope: block\n        maxAgeDays: 30",
        );
        let report = run_checks(temp.path(), &config).unwrap();
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "DH_PIN_005")
        );
        assert!(!report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code.starts_with("DH_PIN_") && diagnostic.message.contains("TERM-2")
        }));
    }
