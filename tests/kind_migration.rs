use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::tempdir;

const CONFIG: &str = r#"
docs:
  bases:
    - id: records
      root: docs/records
      patterns:
        - id: record
          regex: "^[a-z-]+\\.md$"
          documentKind: record
documentContracts:
  maturity:
    declared: governed
  templates:
    - id: record-template
      revision: 2
      compatibleFrom: 1
  profiles:
    - id: record-profile
      template: record-template
      templateRevision: 1
      match:
        paths: [docs/records/*.md]
      requiredSections:
        - id: context
          headings: [Context]
documentKinds:
  - id: record
    base: records
    pattern: record
    profile: record-profile
    scaffold:
      filename: "{slug}.md"
      sectionHeadings:
        context: { en: Context }
    frontmatter:
      revision: 2
      compatibleFrom: 1
      allowUnknownFields: false
      fields:
        - id: id
          type: string
          required: true
          source: identity
languageRepresentations:
  canonical: en
rules:
  governance.identity:
    mode: disabled
"#;

fn write_project(revision: u64) -> tempfile::TempDir {
    let temp = tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join("docs/records")).unwrap();
    std::fs::write(temp.path().join("docs-hygiene.yml"), CONFIG).unwrap();
    std::fs::write(
        temp.path().join("docs/records/first.md"),
        format!("---\nschemaRevision: {revision}\nid: RECORD-1\n---\n\n# Record\n\n## Context\n"),
    )
    .unwrap();
    temp
}

#[test]
fn kind_migration_checks_then_atomically_advances_schema_and_template_revisions() {
    let temp = write_project(1);
    let config_path = temp.path().join("docs-hygiene.yml");
    let doc_path = temp.path().join("docs/records/first.md");
    let before_config = std::fs::read(&config_path).unwrap();
    let before_doc = std::fs::read(&doc_path).unwrap();

    let output = Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "migrate-kinds",
            temp.path().to_str().unwrap(),
            "--check",
            "--format",
            "json",
        ])
        .assert()
        .failure()
        .get_output()
        .stdout
        .clone();
    let report: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(report["schemaVersion"], "docs-hygiene.kind-migration.v1");
    assert_eq!(report["schemaChanges"][0]["fromRevision"], 1);
    assert_eq!(report["schemaChanges"][0]["toRevision"], 2);
    assert_eq!(report["templateChanges"][0]["fromRevision"], 1);
    assert_eq!(report["templateChanges"][0]["toRevision"], 2);
    assert_eq!(std::fs::read(&config_path).unwrap(), before_config);
    assert_eq!(std::fs::read(&doc_path).unwrap(), before_doc);

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["migrate-kinds", temp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Applied all compatible"));
    assert!(
        std::fs::read_to_string(&config_path)
            .unwrap()
            .contains("templateRevision: 2")
    );
    assert!(
        std::fs::read_to_string(&doc_path)
            .unwrap()
            .contains("schemaRevision: 2")
    );
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn incompatible_schema_blocks_every_document_and_template_write() {
    let temp = write_project(0);
    let config_path = temp.path().join("docs-hygiene.yml");
    let doc_path = temp.path().join("docs/records/first.md");
    let before_config = std::fs::read(&config_path).unwrap();
    let before_doc = std::fs::read(&doc_path).unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["migrate-kinds", temp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("outside compatibility window"))
        .stderr(predicate::str::contains("no changes were applied"));
    assert_eq!(std::fs::read(&config_path).unwrap(), before_config);
    assert_eq!(std::fs::read(&doc_path).unwrap(), before_doc);
}
