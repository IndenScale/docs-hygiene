    #[test]
    fn detects_duplicate_document_numbers() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
        fs::write(temp.path().join("docs/01_setup.md"), "# Setup\n").unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();

        assert_has_code(&report, "DH_SEQ_002");
    }

    #[test]
    fn detects_docs_that_exceed_max_lines() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        let long_doc = (0..25)
            .map(|idx| format!("line {idx}"))
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(temp.path().join("docs/01_long.md"), long_doc).unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();

        assert_has_code(&report, "DH_SIZE_001");
    }

    #[test]
    fn detects_dead_repository_markdown_links_and_ignores_external_targets() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/assets")).unwrap();
        fs::write(temp.path().join("README.md"), "# Home\n").unwrap();
        fs::write(temp.path().join("docs/assets/logo.svg"), "<svg/>\n").unwrap();
        fs::write(
            temp.path().join("docs/01_links.md"),
            "# Links\n\n[home](../README.md)\n![logo](assets/logo.svg)\n[external](https://example.com/missing)\n[anchor](#links)\n`[example](not-a-link.md)`\n[dead](missing.md)\n[reference][missing]\n\n[missing]: ../absent.md\n",
        )
        .unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();
        let links = report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "DH_LINK_001")
            .collect::<Vec<_>>();

        assert_eq!(links.len(), 2, "{:?}", report.diagnostics);
        assert!(
            links
                .iter()
                .any(|diagnostic| diagnostic.message.contains("missing.md"))
        );
        assert!(
            links
                .iter()
                .any(|diagnostic| diagnostic.message.contains("../absent.md"))
        );
    }

    #[test]
    fn detects_language_threshold_violations_and_ignores_code_blocks() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/01_case.md"),
            "# Case\n\nEnglish text.\n\n```text\n大量中文大量中文大量中文\n```\n\n正文中文过多。\n",
        )
        .unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
languageRepresentations:
  canonical: en
language:
  en:
    maxCjkRatio: 0.05
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert_has_code(&report, "DH_LANG_002");
    }

    #[test]
    fn detects_orphan_concepts_when_enabled() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::create_dir_all(temp.path().join("concept")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
        fs::write(temp.path().join("concept/Unused.md"), "# Unused\n").unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();

        assert_has_code(&report, "DH_CONCEPT_002");
    }

    #[test]
    fn detects_missing_adapter_command() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
adapters:
  markdownlint:
    enabled: true
    command: definitely-not-a-real-docs-hygiene-test-command
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert_has_code(&report, "DH_ADAPTER_001");
    }

    #[test]
    fn suppresses_diagnostics_by_code_and_path() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/01_case.md"),
            "# Case\n\n大量中文 mixed into English docs.\n",
        )
        .unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
languageRepresentations:
  canonical: en
language:
  en:
    maxCjkRatio: 0.05
suppressions:
  - code: DH_LANG_002
    paths:
      - docs/01_case.md
    reason: Test case intentionally includes Chinese text.
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    }

    #[test]
    fn wildcard_suppression_is_limited_by_path() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/examples")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/examples/01_case.md"),
            "# Case\n\n大量中文 mixed into English docs.\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("docs/01_case.md"),
            "# Case\n\n大量中文 mixed into English docs.\n",
        )
        .unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
languageRepresentations:
  canonical: en
language:
  en:
    maxCjkRatio: 0.05
suppressions:
  - code: "*"
    paths:
      - docs/examples/**
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();
        let lang_diagnostics = report
            .diagnostics
            .iter()
            .filter(|diag| diag.code == "DH_LANG_002")
            .collect::<Vec<_>>();

        assert_eq!(lang_diagnostics.len(), 1, "{:?}", report.diagnostics);
        assert_eq!(lang_diagnostics[0].path, "docs/01_case.md");
    }

    #[test]
    fn path_and_filename_infer_contract_with_localized_heading_aliases() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/decisions")).unwrap();
        fs::write(
            temp.path().join("docs/decisions/0001-record-contracts.md"),
            "# 记录文档契约\n\n## 上下文\n\n背景。\n\n## 决策\n\n采用路径推导。\n\n## 后果\n\n保持开放扩展。\n\n## 实施说明\n\n额外章节合法。\n",
        )
        .unwrap();
        let config: Config = serde_yaml::from_str(
            r#"
docs:
  bases:
    - id: decisions
      root: docs/decisions
      patterns:
        - id: adr
          regex: "^\\d{4}-[a-z0-9-]+\\.md$"
          documentKind: adr
documentContracts:
  maturity:
    declared: maintained
  profiles:
    - id: adr
      match:
        paths: ["docs/**/decisions/*.md"]
        filenames: ["^\\d{4}-[a-z0-9-]+\\.md$"]
      orderedSections: true
      requiredSections:
        - id: context
          headings: [Context, 上下文]
        - id: decision
          headings: [Decision, 决策]
        - id: consequences
          headings: [Consequences, 后果]
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert!(!report.document_templates.proves_reuse());
    }

    #[test]
    fn contract_reports_missing_sections_fields_order_and_mature_placeholders() {
        let temp = tempdir().unwrap();
        fs::write(
            temp.path().join("ROADMAP.md"),
            "# Roadmap\n\n## 验收\n\nTODO\n\n## 目标\n\n形成稳定入口。\n",
        )
        .unwrap();
        let config: Config = serde_yaml::from_str(
            r#"
documentContracts:
  maturity:
    declared: governed
  profiles:
    - id: roadmap
      match:
        paths: [ROADMAP.md]
        filenames: ["^ROADMAP\\.md$"]
      enforceFrom: maintained
      placeholdersAllowedUntil: growing
      placeholderPatterns: ["(?i)\\bTODO\\b"]
      orderedSections: true
      requiredSections:
        - id: goal
          headings: [目标]
        - id: acceptance
          headings: [验收]
        - id: exit
          headings: [退出条件]
      requiredFields:
        - id: owner
          pattern: "(?m)^负责人："
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();

        for code in [
            "DH_CONTRACT_001",
            "DH_CONTRACT_002",
            "DH_CONTRACT_003",
            "DH_CONTRACT_004",
        ] {
            assert_has_code(&report, code);
        }
    }

    #[test]
    fn template_contract_merges_with_profile_and_proves_complete_reuse() {
        let temp = tempdir().unwrap();
        fs::write(
            temp.path().join("README.md"),
            "# Project\n\n## Context\n\nBackground.\n\n## Decision\n\nUse templates.\n",
        )
        .unwrap();
        let config: Config = serde_yaml::from_str(
            r#"
documentContracts:
  maturity:
    declared: maintained
  templates:
    - id: maintained-open-contract
      revision: 1
      compatibleFrom: 1
      enforceFrom: maintained
      orderedSections: true
      requiredSections:
        - id: context
          headings: [Context]
  profiles:
    - id: project-readme
      template: maintained-open-contract
      templateRevision: 1
      match:
        paths: [README.md]
      requiredSections:
        - id: decision
          headings: [Decision]
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert!(report.document_templates.proves_reuse());
        assert!(report.document_templates.proves_migration());
        assert_eq!(
            report.document_templates.bindings["maintained-open-contract"],
            ["project-readme"]
        );
    }

    #[test]
    fn template_registry_rejects_unknown_bindings_and_duplicate_members() {
        let temp = tempdir().unwrap();
        fs::write(temp.path().join("README.md"), "# Project\n").unwrap();
        let config: Config = serde_yaml::from_str(
            r#"
documentContracts:
  templates:
    - id: base
      requiredSections:
        - id: context
          headings: [Context]
  profiles:
    - id: duplicate
      template: base
      match:
        paths: [README.md]
      requiredSections:
        - id: context
          headings: [背景]
    - id: unknown
      template: absent
      match:
        paths: [NEVER.md]
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();

        assert_has_code(&report, "DH_TEMPLATE_001");
        assert!(!report.document_templates.registry_valid);
        assert!(!report.document_templates.proves_reuse());
    }

    #[test]
    fn template_registry_reports_unused_templates() {
        let temp = tempdir().unwrap();
        let config: Config = serde_yaml::from_str(
            r#"
documentContracts:
  templates:
    - id: unused
      enforceFrom: maintained
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();

        assert_has_code(&report, "DH_TEMPLATE_002");
        assert_eq!(report.document_templates.unused_templates, ["unused"]);
        assert!(!report.document_templates.proves_reuse());
    }

    #[test]
    fn invalid_template_expressions_remain_stable_diagnostics() {
        let temp = tempdir().unwrap();
        fs::write(
            temp.path().join("README.md"),
            "# Project\n\n## Context\n\nContent.\n",
        )
        .unwrap();
        let config: Config = serde_yaml::from_str(
            r#"
documentContracts:
  templates:
    - id: invalid-pattern
      placeholderPatterns: ["["]
      requiredSections:
        - id: context
          headings: [Context]
  profiles:
    - id: project-readme
      template: invalid-pattern
      match:
        paths: [README.md]
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();

        assert_has_code(&report, "DH_TEMPLATE_001");
        assert!(!report.document_templates.registry_valid);
    }

    #[test]
    fn repository_signals_recommend_but_do_not_force_maturity() {
        let temp = tempdir().unwrap();
        fs::write(
            temp.path().join("README.md"),
            "# Project\n\nA small project.\n",
        )
        .unwrap();
        let config: Config = serde_yaml::from_str(
            r#"
documentContracts:
  maturity:
    declared: seed
    recommendations:
      - level: growing
        minProjectLines: 2
        minProjectBytes: 10
"#,
        )
        .unwrap();

        let report = run_checks(temp.path(), &config).unwrap();

        assert_has_code(&report, "DH_MATURITY_001");
        let diagnostic = report
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "DH_MATURITY_001")
            .unwrap();
        assert!(matches!(diagnostic.severity, Severity::Info));
        assert_eq!(report.summary.warning_count, 0);
        assert_eq!(report.summary.info_count, 1);
    }
