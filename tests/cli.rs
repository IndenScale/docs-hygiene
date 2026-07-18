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
fn explain_rules_reports_stable_text_and_json_contracts() {
    let temp = tempdir().unwrap();
    std::fs::write(temp.path().join("docs-hygiene.yml"), "{}\n").unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["explain-rules", temp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("project.entry-docs"))
        .stdout(predicate::str::contains("State"))
        .stdout(predicate::str::contains("Facts:"));

    let output = Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "explain-rules",
            temp.path().to_str().unwrap(),
            "--format",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let value: Value = serde_json::from_slice(&output).unwrap();

    assert_eq!(value["schemaVersion"], "docs-hygiene.rule-activation.v1");
    assert_eq!(value["decisions"].as_array().unwrap().len(), 9);
    assert_eq!(value["decisions"][0]["rule"], "project.entry-docs");
    assert!(value["decisions"][0]["rationale"].is_string());
    assert!(value["decisions"][0]["remediation"].is_string());
}

#[test]
fn profile_reports_maturity_and_execution_state_in_one_versioned_contract() {
    let temp = tempdir().unwrap();
    std::fs::create_dir(temp.path().join("docs")).unwrap();
    std::fs::write(temp.path().join("docs/01_overview.md"), "# Overview\n").unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        r#"
docs:
  bases:
    - id: main
      root: docs
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          numbered: true
documentContracts:
  maturity:
    declared: seed
"#,
    )
    .unwrap();

    let output = Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["profile", temp.path().to_str().unwrap(), "--format", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let value: Value = serde_json::from_slice(&output).unwrap();

    assert_eq!(value["schemaVersion"], "docs-hygiene.profile.v1");
    assert_eq!(value["dimensions"][0]["dimension"], "structure");
    assert_eq!(value["dimensions"][0]["target"], "basic");
    assert_eq!(value["dimensions"][0]["observed"], "governed");
    assert_eq!(value["decisions"].as_array().unwrap().len(), 9);
    assert_eq!(value["documentTemplates"]["configuredTemplates"], 0);
    assert_eq!(value["documentTemplates"]["configuredProfiles"], 0);
    assert_eq!(value["documentTemplates"]["registryValid"], true);
    assert_eq!(value["governanceGraph"]["metrics"]["nodes"], 0);
    assert_eq!(value["governanceGraph"]["metrics"]["edges"], 0);
}

#[test]
fn profile_gate_fails_below_target_without_hiding_the_report() {
    let temp = tempdir().unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        "documentContracts:\n  maturity:\n    declared: maintained\n",
    )
    .unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "profile",
            temp.path().to_str().unwrap(),
            "--fail-below-target",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("structure"))
        .stdout(predicate::str::contains("belowTarget"))
        .stderr(predicate::str::contains("below a required target"));
}

#[test]
fn scale_only_activation_remains_non_blocking() {
    let temp = tempdir().unwrap();
    std::fs::write(temp.path().join("docs-hygiene.yml"), "{}\n").unwrap();
    for index in 0..20 {
        std::fs::write(
            temp.path().join(format!("doc-{index:02}.md")),
            format!("# Document {index}\n"),
        )
        .unwrap();
    }

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap(), "--fail-on-warning"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DH_ACTIVATION_001 Info"))
        .stdout(predicate::str::contains("Activated because:"))
        .stdout(predicate::str::contains("Why this matters:"))
        .stdout(predicate::str::contains("How to fix:"));
}

#[test]
fn disabled_rule_suppresses_its_checker_diagnostics() {
    let temp = tempdir().unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        r#"
entryDocs:
  required: [README.md]
rules:
  project.entry-docs:
    mode: disabled
"#,
    )
    .unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("DH_REQUIRED_001").not());
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
    assert!(temp.path().join("docs/README.md").exists());
    assert!(temp.path().join("docs/zh/README.md").exists());
    assert!(temp.path().join("docs/guide/overview.md").exists());
    assert!(temp.path().join("docs/zh/guide/overview.md").exists());
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
    assert!(initialized.contains("documentKind: index"));
    assert!(initialized.contains("documentKind: guide"));

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
fn repository_scale_names_are_rejected_in_favor_of_project_scale() {
    let temp = tempdir().unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        "documentContracts:\n  maturity:\n    recommendations:\n      - level: growing\n        minRepositoryLines: 10\n",
    )
    .unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "unknown field `minRepositoryLines`",
        ));
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
        minProjectLines: 1
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
        "governance:\n  manifests: [ul.yml, issue.yml]\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("ul.yml"),
        "id: UL-1\nreferenceRelation: library\nstatus: baselined\nmembers: [term.md]\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("term.md"),
        "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("issue.yml"),
        "id: ISSUE-1\nreferenceRelation: body\nstatus: proposed\n",
    )
    .unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("DH_REFERENCE_001 Error"));
}
