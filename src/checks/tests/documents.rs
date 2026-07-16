    fn config() -> Config {
        serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
  requireContinuousNumbering: true
  maxLines: 20
languageRepresentations:
  canonical: en
  localized: [zh]
  requireDocumentParity: true
  requireNumberParity: true
concepts:
  dir: concept
  requireConceptFile: true
  failOnOrphanConcept: warn
"#,
        )
        .unwrap()
    }

    fn codes(report: &Report) -> Vec<&'static str> {
        report
            .diagnostics
            .iter()
            .map(|diag| diag.code)
            .collect::<Vec<_>>()
    }

    fn assert_has_code(report: &Report, code: &str) {
        assert!(
            report.diagnostics.iter().any(|diag| diag.code == code),
            "expected {code}, got {:?}",
            report.diagnostics
        );
    }

    #[test]
    fn detects_missing_required_number_and_concept() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(
            temp.path().join("docs/02_architecture.md"),
            "# Architecture\n\nUses **Event Sourcing**.\n",
        )
        .unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();
        let codes = codes(&report);

        assert!(codes.contains(&"DH_REQUIRED_001"));
        assert!(codes.contains(&"DH_SEQ_001"));
        assert!(codes.contains(&"DH_REPRESENTATION_001"));
        assert!(codes.contains(&"DH_CONCEPT_001"));
    }

    #[test]
    fn accepts_clean_numbered_language_representations_with_concept() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/zh")).unwrap();
        fs::create_dir_all(temp.path().join("concept")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/01_architecture.md"),
            "# Architecture\n\nUses **Event Sourcing**.\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("docs/zh/01_architecture.md"),
            "# 架构\n\n使用 **Event Sourcing**。\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("concept/Event Sourcing.md"),
            "# Event Sourcing\n",
        )
        .unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();
        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    }

    #[test]
    fn detects_ascii_art_when_enabled() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/01_architecture.md"),
            "# Architecture\n\n+---------+\n| Client  | ---> API\n+---------+\n",
        )
        .unwrap();

        let mut policy = config();
        policy.docs.forbid_ascii_art = true;
        let report = run_checks(temp.path(), &policy).unwrap();
        let diagnostic = report
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "DH_ASCII_001")
            .expect("ASCII art diagnostic");
        assert_eq!(diagnostic.path, "docs/01_architecture.md");
        assert_eq!(diagnostic.range.start.line, 2);
    }

    #[test]
    fn detects_mixed_language_ascii_art_diagram() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/01_architecture.md"),
            r#"# Architecture

Specification IR                         Assembly graph
需求 / 不变式 / 验收条件 / Closure target   对象 / 接口 / 关系 / 证据
                  \                       /
                   \--- compile(...) ----/
                              |
            diagnostics / metrics / Closure / acceptance
"#,
        )
        .unwrap();

        let mut policy = config();
        policy.docs.forbid_ascii_art = true;
        let report = run_checks(temp.path(), &policy).unwrap();
        let diagnostic = report
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "DH_ASCII_001")
            .expect("mixed-language ASCII art diagnostic");
        assert_eq!(diagnostic.range.start.line, 4);
    }

    #[test]
    fn detects_ascii_art_in_text_fences_but_ignores_code_and_markdown_tables() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/01_architecture.md"),
            "# Architecture\n\n```text\n+-----+\n| API |\n+-----+\n```\n\n```python\n+-----+\n| API |\n+-----+\n```\n\n| A | B |\n|---|---|\n| C | D |\n",
        )
        .unwrap();

        let mut policy = config();
        policy.docs.forbid_ascii_art = true;
        let report = run_checks(temp.path(), &policy).unwrap();
        let diagnostics = report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "DH_ASCII_001")
            .collect::<Vec<_>>();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].range.start.line, 3);
    }

    #[test]
    fn detects_invalid_filename_and_skips_dependent_doc_rules_for_that_file() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/intro.md"), "# Intro\n").unwrap();

        let report = run_checks(temp.path(), &config()).unwrap();

        assert_has_code(&report, "DH_NAME_001");
        assert!(!codes(&report).contains(&"DH_SEQ_001"));
    }

    #[test]
    fn ignores_non_markdown_files_under_docs() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
        fs::write(temp.path().join("docs/02_notes.txt"), "not markdown\n").unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
  requireContinuousNumbering: true
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert_eq!(report.summary.files_checked, 1);
    }

    #[test]
    fn ignores_configured_directories() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/generated")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
        fs::write(temp.path().join("docs/generated/bad.md"), "# Generated\n").unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
requiredFiles:
  - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
  requireContinuousNumbering: true
ignore:
  paths:
    - docs/generated/**
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert_eq!(report.summary.files_checked, 1);
    }

    #[test]
    fn root_markdown_files_are_allowed_when_not_declared() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("AGENTS.md"), "# Agent Notes\n").unwrap();
        fs::write(temp.path().join("CLAUDE.md"), "# Claude Notes\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
entryDocs:
  required:
    - README.md
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z0-9_-]+\\.md$"
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    }

    #[test]
    fn docs_base_denies_unknown_markdown_but_allows_index_pattern() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
        fs::write(temp.path().join("docs/INDEX.md"), "# Index\n").unwrap();
        fs::write(temp.path().join("docs/freeform.md"), "# Freeform\n").unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
entryDocs:
  required:
    - README.md
docs:
  bases:
    - id: main
      root: docs
      requireContinuousNumbering: true
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          documentKind: numbered
          numbered: true
        - id: index
          regex: "^INDEX\\.md$"
          documentKind: index
          numbered: false
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert_has_code(&report, "DH_NAME_001");
        assert_eq!(
            report
                .diagnostics
                .iter()
                .filter(|diag| diag.code == "DH_NAME_001")
                .count(),
            1
        );
        assert!(!codes(&report).contains(&"DH_SEQ_001"));
    }

    #[test]
    fn multiple_bases_use_their_own_patterns() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/guide")).unwrap();
        fs::create_dir_all(temp.path().join("docs/adr")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/guide/01_intro.md"), "# Intro\n").unwrap();
        fs::write(
            temp.path().join("docs/adr/ADR-0001_record.md"),
            "# Record\n",
        )
        .unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
entryDocs:
  required:
    - README.md
docs:
  bases:
    - id: guide
      root: docs/guide
      requireContinuousNumbering: true
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          documentKind: numbered
          numbered: true
    - id: adr
      root: docs/adr
      patterns:
        - id: adr
          regex: "^ADR-\\d{4}_[a-z0-9_-]+\\.md$"
          documentKind: freeform
          numbered: false
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    }

    #[test]
    fn localized_roots_keep_locale_and_semantic_hierarchies_orthogonal() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/intent")).unwrap();
        fs::create_dir_all(temp.path().join("docs/zh/intent")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(
            temp.path().join("docs/intent/01_language.md"),
            "# Language\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("docs/zh/intent/01_language.md"),
            "# 语言\n",
        )
        .unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
entryDocs:
  required: [README.md]
docs:
  bases:
    - id: intent
      root: docs/intent
      localizedRoots:
        zh: docs/zh/intent
      requireContinuousNumbering: true
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          numbered: true
languageRepresentations:
  canonical: en
  localized: [zh]
  requireDocumentParity: true
  requireNumberParity: true
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert_eq!(report.summary.files_checked, 2);
    }

    #[test]
    fn base_ignore_does_not_hide_files_from_other_bases() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/records")).unwrap();
        fs::write(temp.path().join("README.md"), "# Example\n").unwrap();
        fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
        fs::write(temp.path().join("docs/records/0001-note.md"), "# Note\n").unwrap();

        let config: Config = serde_yaml::from_str(
            r#"
entryDocs:
  required:
    - README.md
docs:
  bases:
    - id: main
      root: docs
      requireContinuousNumbering: true
      ignore:
        - docs/records/**
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          documentKind: numbered
          numbered: true
    - id: records
      root: docs/records
      requireContinuousNumbering: false
      patterns:
        - id: record
          regex: "^\\d{4}-[a-z0-9_-]+\\.md$"
          documentKind: record
          numbered: false
"#,
        )
        .unwrap();
        let report = run_checks(temp.path(), &config).unwrap();

        assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
        assert_eq!(report.summary.files_checked, 2);
    }
