use assert_cmd::Command;
use docs_hygiene::Config;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn migrate_templates_checks_and_advances_only_compatible_revision_pins() {
    let temp = tempdir().unwrap();
    let config_path = temp.path().join("docs-hygiene.yml");
    std::fs::write(
        &config_path,
        r#"
documentContracts:
  templates:
    - id: base
      revision: 2
      compatibleFrom: 1
  profiles:
    - id: old
      template: base
      templateRevision: 1
      match: {}
"#,
    )
    .unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "migrate-templates",
            temp.path().to_str().unwrap(),
            "--check",
            "--format",
            "json",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains(
            "docs-hygiene.template-migration.v1",
        ))
        .stdout(predicate::str::contains("\"fromRevision\": 1"))
        .stderr(predicate::str::contains("migration is required"));

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["migrate-templates", temp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("1 change(s)"));

    let migrated = Config::load(&config_path).unwrap();
    assert_eq!(
        migrated.document_contracts.profiles[0].template_revision,
        Some(2)
    );

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "migrate-templates",
            temp.path().to_str().unwrap(),
            "--check",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("0 change(s)"));
}
