use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::tempdir;

#[test]
fn help_and_version_are_available() {
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: docs-hygiene"));

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("docs-hygiene"));
}

#[test]
fn scaffold_creates_starter_docs_tree() {
    let temp = tempdir().unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["scaffold", temp.path().to_str().unwrap()])
        .assert()
        .success();

    assert!(temp.path().join("docs-hygiene.yml").exists());
    assert!(temp.path().join(".markdownlint.yaml").exists());
    assert!(temp.path().join("docs/01_overview.md").exists());
    assert!(temp.path().join("docs/zh/01_overview.md").exists());
    assert!(temp.path().join("concept/Policy Engine.md").exists());
}

#[test]
fn lang_commands_update_config() {
    let temp = tempdir().unwrap();
    let config = temp.path().join("docs-hygiene.yml");

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["init", "--path", config.to_str().unwrap()])
        .assert()
        .success();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "lang",
            "add",
            "ja",
            "--config",
            config.to_str().unwrap(),
            "--min-cjk-ratio",
            "0.10",
        ])
        .assert()
        .success();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "lang",
            "set-threshold",
            "ja",
            "--config",
            config.to_str().unwrap(),
            "--max-cjk-ratio",
            "0.90",
        ])
        .assert()
        .success();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["lang", "list", "--config", config.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("ja"))
        .stdout(predicate::str::contains("minCjkRatio=0.1"))
        .stdout(predicate::str::contains("maxCjkRatio=0.9"));

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["lang", "remove", "ja", "--config", config.to_str().unwrap()])
        .assert()
        .success();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["lang", "list", "--config", config.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("ja").not());
}

#[test]
fn json_output_uses_versioned_lsp_style_diagnostics() {
    let temp = tempdir().unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        r#"
entryDocs:
  required:
    - README.md
"#,
    )
    .unwrap();

    let output = Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap(), "--format", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let value: Value = serde_json::from_slice(&output).unwrap();
    let diagnostic = &value["diagnostics"][0];

    assert_eq!(value["schemaVersion"], "docs-hygiene.diagnostic.v1");
    assert_eq!(value["summary"]["diagnosticCount"], 1);
    assert_eq!(diagnostic["source"], "docs-hygiene");
    assert_eq!(diagnostic["code"], "DH_REQUIRED_001");
    assert_eq!(diagnostic["severity"], "error");
    assert_eq!(diagnostic["path"], "README.md");
    assert!(diagnostic["uri"].as_str().unwrap().starts_with("file://"));
    assert_eq!(diagnostic["range"]["start"]["line"], 0);
    assert_eq!(diagnostic["range"]["start"]["character"], 0);
    assert!(
        diagnostic["relatedInformation"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}
