fn slug_config(extra: &str) -> Config {
    serde_yaml::from_str(&format!(
        r#"
docs:
  bases:
    - id: articles
      root: docs
      patterns:
        - id: article
          regex: "^(?P<slug>.+)\\.md$"
          documentKind: article
  slugSchemas:
    - documentKind: article
      source:
        type: frontmatter
        field: slug
      pattern: "^[a-z][a-z0-9-]*$"
      minLength: 3
      maxLength: 24
      reserved: [admin, api]
      normalization: lowercaseKebab
      identityField: id
      aliasesField: aliases
      renamePolicy: stableIdentity
{extra}
"#
    ))
    .unwrap()
}

fn write_slug_doc(root: &Path, path: &str, id: &str, slug: &str, aliases: &[&str]) {
    let aliases = if aliases.is_empty() {
        String::new()
    } else {
        format!(
            "aliases:\n{}",
            aliases
                .iter()
                .map(|alias| format!("  - {alias}\n"))
                .collect::<String>()
        )
    };
    fs::create_dir_all(root.join(path).parent().unwrap()).unwrap();
    fs::write(
        root.join(path),
        format!("---\nid: {id}\nslug: {slug}\n{aliases}---\n\n# Article\n"),
    )
    .unwrap();
}

#[test]
fn slug_schema_reports_illegal_reserved_and_missing_identity_values() {
    let temp = tempdir().unwrap();
    write_slug_doc(temp.path(), "docs/reserved.md", "DOC-1", "Admin", &[]);
    write_slug_doc(temp.path(), "docs/illegal.md", "DOC-2", "中文", &[]);
    fs::write(
        temp.path().join("docs/missing.md"),
        "---\nslug: valid-slug\n---\n\n# Missing identity\n",
    )
    .unwrap();

    let mut policy = slug_config("");
    let report = run_checks(temp.path(), &policy).unwrap();
    let slug_diagnostics = report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "DH_SLUG_001")
        .collect::<Vec<_>>();

    assert_eq!(slug_diagnostics.len(), 3, "{:?}", report.diagnostics);
    assert!(slug_diagnostics.iter().any(|diagnostic| {
        diagnostic.path == "docs/reserved.md"
            && diagnostic
                .data
                .as_ref()
                .is_some_and(|data| data.normalized_value == "admin")
    }));
    assert!(slug_diagnostics.iter().any(|diagnostic| {
        diagnostic.path == "docs/illegal.md" && diagnostic.message.contains("does not match")
    }));
    assert!(slug_diagnostics.iter().any(|diagnostic| {
        diagnostic.path == "docs/missing.md" && diagnostic.message.contains("stable identity")
    }));

    policy
        .docs
        .slug_schemas
        .push(policy.docs.slug_schemas[0].clone());
    let duplicate_schema = run_checks(temp.path(), &policy).unwrap();
    assert!(duplicate_schema.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DH_SLUG_001"
            && diagnostic.path == "docs-hygiene.yml"
            && diagnostic.message.contains("more than one slug Schema")
    }));
}

#[test]
fn slug_index_reports_case_folded_primary_and_alias_conflicts() {
    let temp = tempdir().unwrap();
    write_slug_doc(temp.path(), "docs/one.md", "DOC-1", "Alpha", &["legacy"]);
    write_slug_doc(temp.path(), "docs/two.md", "DOC-2", "alpha", &[]);
    write_slug_doc(temp.path(), "docs/three.md", "DOC-3", "legacy", &[]);

    let report = run_checks(temp.path(), &slug_config("")).unwrap();
    let conflicts = report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "DH_SLUG_001" && diagnostic.message.contains("collision")
        })
        .collect::<Vec<_>>();

    assert_eq!(conflicts.len(), 2, "{:?}", report.diagnostics);
    assert!(conflicts.iter().any(|diagnostic| {
        diagnostic.message.contains("case-folding")
            && diagnostic
                .data
                .as_ref()
                .and_then(|data| data.conflict_path.as_deref())
                == Some("docs/one.md")
    }));
    assert!(conflicts.iter().any(|diagnostic| {
        diagnostic.message.contains("normalized collision")
            && ["docs/one.md", "docs/three.md"].contains(&diagnostic.path.as_str())
    }), "{conflicts:?}");
}

#[test]
fn duplicate_stable_identity_is_allowed_only_across_language_representations() {
    let temp = tempdir().unwrap();
    write_slug_doc(temp.path(), "docs/one.md", "DOC-1", "shared", &[]);
    write_slug_doc(temp.path(), "docs/two.md", "DOC-1", "shared", &[]);

    let report = run_checks(temp.path(), &slug_config("")).unwrap();
    let conflict = report
        .diagnostics
        .iter()
        .find(|diagnostic| {
            diagnostic.code == "DH_SLUG_001" && diagnostic.message.contains("collision")
        })
        .expect("same-language duplicate identity must collide");
    assert_eq!(
        conflict
            .data
            .as_ref()
            .map(|data| data.normalized_value.as_str()),
        Some("shared")
    );
}

#[test]
fn localized_representations_share_stable_identity_and_authoritative_slug() {
    let temp = tempdir().unwrap();
    write_slug_doc(temp.path(), "docs/en/guide.md", "DOC-GUIDE", "guide", &[]);
    write_slug_doc(temp.path(), "docs/zh/指南.md", "DOC-GUIDE", "zhi-nan", &[]);
    let config: Config = serde_yaml::from_str(
        r#"
docs:
  bases:
    - id: articles
      root: docs/en
      localizedRoots:
        zh: docs/zh
      patterns:
        - id: article
          regex: "^.+\\.md$"
          documentKind: article
  slugSchemas:
    - documentKind: article
      source: { type: frontmatter, field: slug }
      pattern: "^[a-z][a-z0-9-]*$"
      normalization: lowercaseKebab
languageRepresentations:
  canonical: en
  localized: [zh]
"#,
    )
    .unwrap();

    let report = run_checks(temp.path(), &config).unwrap();
    let diagnostic = report
        .diagnostics
        .iter()
        .find(|diagnostic| {
            diagnostic.code == "DH_SLUG_001" && diagnostic.message.contains("representations")
        })
        .expect("localized slug drift diagnostic");
    assert_eq!(diagnostic.path, "docs/zh/指南.md");
    assert_eq!(
        diagnostic
            .data
            .as_ref()
            .and_then(|data| data.conflict_path.as_deref()),
        Some("docs/en/guide.md")
    );

    write_slug_doc(temp.path(), "docs/zh/指南.md", "DOC-GUIDE", "guide", &[]);
    let aligned = run_checks(temp.path(), &config).unwrap();
    assert!(!aligned
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "DH_SLUG_001"));
}

#[test]
fn filename_and_stable_id_sources_are_explicit_and_require_alias_mode_is_enforced() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("docs")).unwrap();
    fs::write(
        temp.path().join("docs/my-guide.md"),
        "---\nid: DOC-Guide\naliases: [old-guide]\n---\n\n# Guide\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("docs/other.md"),
        "---\nid: Other Guide\n---\n\n# Other\n",
    )
    .unwrap();
    let config: Config = serde_yaml::from_str(
        r#"
docs:
  bases:
    - id: articles
      root: docs
      patterns:
        - id: filename-kind
          regex: "^(?P<slug>my-guide)\\.md$"
          documentKind: filename-article
        - id: id-kind
          regex: "^other\\.md$"
          documentKind: id-article
  slugSchemas:
    - documentKind: filename-article
      source: { type: filename, capture: slug }
      pattern: "^[a-z-]+$"
      renamePolicy: requireAlias
    - documentKind: id-article
      source: { type: stableId, field: id }
      pattern: "^[a-z-]+$"
      normalization: lowercaseKebab
rules:
  governance.identity:
    mode: disabled
"#,
    )
    .unwrap();

    let report = run_checks(temp.path(), &config).unwrap();
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
}

#[test]
fn projects_without_slug_schemas_remain_compatible() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("docs")).unwrap();
    fs::write(temp.path().join("docs/01_intro.md"), "# Intro\n").unwrap();
    let config: Config = serde_yaml::from_str(
        r#"
docs:
  root: docs
  filenamePattern: "^\\d{2}_[a-z]+\\.md$"
"#,
    )
    .unwrap();

    let report = run_checks(temp.path(), &config).unwrap();
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
}
