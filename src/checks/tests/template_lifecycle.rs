    #[test]
    fn governed_template_lifecycle_reports_migration_and_compatibility_failures() {
        let temp = tempdir().unwrap();
        let outdated: Config = serde_yaml::from_str(
            r#"
documentContracts:
  maturity:
    declared: governed
  templates:
    - id: base
      revision: 3
      compatibleFrom: 1
  profiles:
    - id: old
      template: base
      templateRevision: 2
      match: {}
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &outdated).unwrap();
        assert_has_code(&report, "DH_TEMPLATE_003");
        assert_eq!(report.document_templates.outdated_profiles, ["old"]);
        assert!(!report.document_templates.proves_migration());

        let incompatible: Config = serde_yaml::from_str(
            r#"
documentContracts:
  maturity:
    declared: governed
  templates:
    - id: base
      revision: 3
      compatibleFrom: 2
  profiles:
    - id: old
      template: base
      templateRevision: 1
      match: {}
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &incompatible).unwrap();
        assert_has_code(&report, "DH_TEMPLATE_004");
        assert_eq!(report.document_templates.incompatible_profiles, ["old"]);
    }

    #[test]
    fn governed_templates_require_revision_metadata_and_profile_pins() {
        let temp = tempdir().unwrap();
        let config: Config = serde_yaml::from_str(
            r#"
documentContracts:
  maturity:
    declared: governed
  templates:
    - id: base
  profiles:
    - id: floating
      template: base
      match: {}
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();

        assert_has_code(&report, "DH_TEMPLATE_003");
        assert_eq!(report.document_templates.unrevisioned_templates, ["base"]);
        assert_eq!(report.document_templates.unpinned_profiles, ["floating"]);
        assert!(!report.document_templates.proves_migration());
    }
