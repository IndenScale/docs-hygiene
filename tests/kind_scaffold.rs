use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

const CONFIG: &str = r#"
docs:
  bases:
    - id: articles
      root: docs/articles
      localizedRoots:
        zh: docs/zh/articles
      patterns:
        - id: article
          regex: "^[a-z][a-z0-9-]+\\.md$"
          documentKind: article
  slugSchemas:
    - documentKind: article
      source: { type: frontmatter, field: slug }
      pattern: "^[a-z][a-z0-9-]+$"
      minLength: 3
      maxLength: 30
      reserved: [admin]
      normalization: lowercaseKebab
      identityField: id
      aliasesField: aliases
      renamePolicy: stableIdentity
documentContracts:
  maturity:
    declared: governed
  templates:
    - id: article-template
      revision: 2
      compatibleFrom: 1
      orderedSections: true
  profiles:
    - id: article-profile
      template: article-template
      templateRevision: 2
      match:
        paths: [docs/articles/*.md, docs/zh/articles/*.md]
        filenames: ["^[a-z][a-z0-9-]+\\.md$"]
      requiredSections:
        - id: context
          headings: [Context, 上下文]
        - id: decision
          headings: [Decision, 决策]
      requiredFields:
        - id: stable-id
          pattern: "(?m)^id: ARTICLE-[0-9]+$"
documentKinds:
  - id: article
    base: articles
    pattern: article
    profile: article-profile
    scaffold:
      filename: "{slug}.md"
      title: "{identity}"
      sectionHeadings:
        context: { en: Context, zh: 上下文 }
        decision: { en: Decision, zh: 决策 }
    frontmatter:
      revision: 2
      compatibleFrom: 1
      revisionField: schemaRevision
      allowUnknownFields: false
      fields:
        - id: id
          type: string
          required: true
          source: identity
          format: "^ARTICLE-[0-9]+$"
        - id: slug
          type: string
          required: true
          source: slug
          format: "^[a-z][a-z0-9-]+$"
        - id: locale
          type: string
          required: true
          source: locale
          values: [en, zh]
        - id: status
          type: string
          required: true
          values: [draft, current, superseded]
          default: draft
        - id: priority
          type: integer
          required: true
          default: 1
        - id: score
          type: number
          required: true
          default: 1.5
        - id: reviewed
          type: boolean
          required: true
          default: false
        - id: tags
          type: stringList
          required: true
          default: [docs]
        - id: supersededBy
          type: string
        - id: reviewer
          type: string
      conditions:
        - when: { field: status, equals: superseded }
          required: [supersededBy]
      invariants:
        - left: id
          operator: notEquals
          right: reviewer
languageRepresentations:
  canonical: en
  localized: [zh]
rules:
  governance.identity:
    mode: disabled
"#;

fn project() -> tempfile::TempDir {
    let temp = tempdir().unwrap();
    std::fs::write(temp.path().join("docs-hygiene.yml"), CONFIG).unwrap();
    temp
}

#[test]
fn kind_scaffold_generates_canonical_and_localized_documents_that_pass_contracts() {
    let temp = project();
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "scaffold",
            temp.path().to_str().unwrap(),
            "--kind",
            "article",
            "--identity",
            "ARTICLE-1",
            "--slug",
            "first-post",
            "--field",
            "priority=2",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "created docs/articles/first-post.md",
        ));
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "scaffold",
            temp.path().to_str().unwrap(),
            "--kind",
            "article",
            "--identity",
            "ARTICLE-2",
            "--slug",
            "localized-post",
            "--locale",
            "zh",
        ])
        .assert()
        .success();

    let canonical =
        std::fs::read_to_string(temp.path().join("docs/articles/first-post.md")).unwrap();
    assert!(canonical.contains("schemaRevision: 2"));
    assert!(canonical.contains("priority: 2"));
    assert!(canonical.contains("score: 1.5"));
    assert!(canonical.contains("reviewed: false"));
    assert!(canonical.contains("tags:\n- docs"));
    assert!(canonical.contains("## Context\n\n## Decision"));
    let localized =
        std::fs::read_to_string(temp.path().join("docs/zh/articles/localized-post.md")).unwrap();
    assert!(localized.contains("locale: zh"));
    assert!(localized.contains("## 上下文\n\n## 决策"));

    std::fs::write(
        temp.path().join("docs/articles/first-post.md"),
        format!("{canonical}\n## Additional body section\n\nOpen contracts permit this.\n"),
    )
    .unwrap();
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn kind_scaffold_dry_run_is_read_only_and_existing_file_requires_force() {
    let temp = project();
    let args = [
        "scaffold",
        temp.path().to_str().unwrap(),
        "--kind",
        "article",
        "--identity",
        "ARTICLE-3",
        "--slug",
        "dry-run",
        "--target",
        "docs/articles",
    ];
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(args)
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("docs/articles/dry-run.md"))
        .stdout(predicate::str::contains("schemaRevision: 2"));
    assert!(!temp.path().join("docs/articles/dry-run.md").exists());

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(args)
        .assert()
        .success();
    let before = std::fs::read(temp.path().join("docs/articles/dry-run.md")).unwrap();
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(args)
        .assert()
        .failure()
        .stderr(predicate::str::contains("no files were written"));
    assert_eq!(
        std::fs::read(temp.path().join("docs/articles/dry-run.md")).unwrap(),
        before
    );
}

#[test]
fn kind_scaffold_rejects_typed_conditional_and_unknown_inputs_atomically() {
    let temp = project();
    let base = [
        "scaffold",
        temp.path().to_str().unwrap(),
        "--kind",
        "article",
        "--identity",
        "ARTICLE-4",
        "--slug",
        "invalid-input",
    ];
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(base)
        .args(["--field", "priority=high"])
        .assert()
        .failure();
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(base)
        .args(["--field", "status=superseded"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("requires field 'supersededBy'"));
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(base)
        .args(["--field", "unknown=value"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown scaffold field"));
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "scaffold",
            temp.path().to_str().unwrap(),
            "--kind",
            "article",
            "--identity",
            "ARTICLE-5",
            "--slug",
            "admin",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "violates the Document Kind slug Schema",
        ));
    assert!(!temp.path().join("docs/articles/invalid-input.md").exists());
    assert!(!temp.path().join("docs/articles/admin.md").exists());

    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        CONFIG.replace(
            "(?m)^id: ARTICLE-[0-9]+$",
            "(?m)^field-that-scaffold-cannot-produce:",
        ),
    )
    .unwrap();
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(base)
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot satisfy required field"));
    assert!(!temp.path().join("docs/articles/invalid-input.md").exists());
}

#[test]
fn checker_reports_enum_type_unknown_condition_invariant_and_revision_failures() {
    let temp = project();
    std::fs::create_dir_all(temp.path().join("docs/articles")).unwrap();
    std::fs::write(
        temp.path().join("docs/articles/broken.md"),
        r#"---
schemaRevision: 1
id: ARTICLE-5
slug: broken
locale: xx
status: superseded
priority: high
reviewed: false
tags: docs
reviewer: ARTICLE-5
extra: forbidden
---

# ARTICLE-5

## Context

## Decision
"#,
    )
    .unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap(), "--fail-on-warning"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("DH_KIND_002"))
        .stdout(predicate::str::contains("DH_FRONTMATTER_001"))
        .stdout(predicate::str::contains("outside its enum"))
        .stdout(predicate::str::contains("requires field 'supersededBy'"))
        .stdout(predicate::str::contains(
            "forbids unknown frontmatter field 'extra'",
        ))
        .stdout(predicate::str::contains(
            "requires 'id' to differ from 'reviewer'",
        ));
}

#[test]
fn kind_registry_rejects_inconsistent_profile_binding_without_writing() {
    let temp = tempdir().unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        CONFIG.replace("profile: article-profile", "profile: missing-profile"),
    )
    .unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("DH_KIND_001"))
        .stdout(predicate::str::contains("unknown document profile"));
}
