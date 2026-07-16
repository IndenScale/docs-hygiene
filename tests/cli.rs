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

    let initialized = std::fs::read_to_string(&config).unwrap();
    assert!(initialized.contains("languageRepresentations:"));
    assert!(initialized.contains("canonical: en"));
    assert!(initialized.contains("documentKind: numbered"));

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "lang",
            "add",
            "en",
            "--canonical",
            "--config",
            config.to_str().unwrap(),
        ])
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
        .stdout(predicate::str::contains("canonical: en"))
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
fn legacy_configuration_names_are_rejected() {
    let temp = tempdir().unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        "i18n:\n  rootLang: en\n  languages: [zh]\n",
    )
    .unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown field `i18n`"));
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
        .failure()
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

#[test]
fn warnings_are_advisory_unless_fail_on_warning_is_set() {
    let temp = tempdir().unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        r#"
documentContracts:
  maturity:
    declared: seed
  profiles:
    - id: readme
      match:
        paths: [README.md]
      enforceFrom: maintained
      requiredSections:
        - id: quick-start
          headings: [Quick Start]
"#,
    )
    .unwrap();
    std::fs::write(temp.path().join("README.md"), "# Early project\n").unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("DH_CONTRACT_001 Warning"));

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap(), "--fail-on-warning"])
        .assert()
        .failure();
}

#[test]
fn informational_maturity_advice_never_fails_the_gate() {
    let temp = tempdir().unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        r#"
documentContracts:
  maturity:
    declared: seed
    recommendations:
      - level: growing
        minRepositoryLines: 1
"#,
    )
    .unwrap();
    std::fs::write(temp.path().join("README.md"), "# Project\n").unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap(), "--fail-on-warning"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DH_MATURITY_001 Info"));
}

#[test]
fn governance_graph_failures_block_the_cli_gate() {
    let temp = tempdir().unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        "governance:\n  manifests: [ul.yml, spec.yml]\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("ul.yml"),
        "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [term.md]\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("term.md"),
        "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("spec.yml"),
        "id: SPEC-1\nrefinementLevel: definition\nreferenceRelation: body\nstatus: proposed\n",
    )
    .unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("DH_REFERENCE_001 Error"))
        .stdout(predicate::str::contains("DH_DERIVATION_001 Error"));
}
