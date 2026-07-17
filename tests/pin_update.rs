use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::tempdir;

fn write_project(targets: &[&str], body: &str) -> tempfile::TempDir {
    let temp = tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join("docs/ul")).unwrap();
    std::fs::create_dir_all(temp.path().join("docs/prd")).unwrap();
    let members = targets
        .iter()
        .enumerate()
        .map(|(index, _)| format!("term-{}.md", index + 1))
        .collect::<Vec<_>>();
    std::fs::write(
        temp.path().join("docs/ul/manifest.yml"),
        format!(
            "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [{}]\n",
            members.join(", ")
        ),
    )
    .unwrap();
    for (index, target) in targets.iter().enumerate() {
        std::fs::write(
            temp.path().join(format!("docs/ul/term-{}.md", index + 1)),
            format!(
                "---\nid: {target}\nstatus: baselined\n---\n\n# {target}\n\n## Contract\n\nCanonical bytes for {target}.\n"
            ),
        )
        .unwrap();
    }
    std::fs::write(
        temp.path().join("docs/prd/manifest.yml"),
        "id: BODY-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nmembers: [body.md]\n",
    )
    .unwrap();
    std::fs::write(temp.path().join("docs/prd/body.md"), body).unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        r#"
governance:
  manifests: [docs/ul/manifest.yml, docs/prd/manifest.yml]
  pinAuditLog: .docs-hygiene/pin-updates.jsonl
  criticalDependencies:
    - id: critical-contracts
      match:
        sourceKinds: [body]
        targetKinds: [library]
        relations: [references]
        sourceIds: [BODY-1]
        targetIds: [TERM-1, TERM-2]
      require:
        algorithms: [sha256]
        minimumScope: commit
        forbidWholeFile: false
        maxAgeDays: 30
rules:
  governance.identity:
    mode: required
  governance.traceability:
    mode: required
"#,
    )
    .unwrap();
    temp
}

#[test]
fn pin_update_defaults_to_read_only_then_applies_anchor_and_audit_atomically() {
    let stale = "0".repeat(64);
    let temp = write_project(
        &["TERM-1"],
        &format!(
            "---\nid: BODY-ITEM\nstatus: proposed\n---\n\n# Body\n\n[[TERM-1#contract@sha256:{stale}]]\n"
        ),
    );
    let body_path = temp.path().join("docs/prd/body.md");
    let before = std::fs::read(&body_path).unwrap();
    let output = Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "update-pins",
            temp.path().to_str().unwrap(),
            "--actor",
            "alice",
            "--reason",
            "reviewed upstream contract",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let plan: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(plan["schemaVersion"], "docs-hygiene.pin-update.v1");
    assert_eq!(plan["changes"][0]["target"], "TERM-1");
    assert_eq!(plan["changes"][0]["oldDigest"], stale);
    assert_eq!(plan["changes"][0]["scope"], "block");
    assert_eq!(std::fs::read(&body_path).unwrap(), before);
    assert!(!temp.path().join(".docs-hygiene/pin-updates.jsonl").exists());

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "update-pins",
            temp.path().to_str().unwrap(),
            "--actor",
            "alice",
            "--reason",
            "reviewed upstream contract",
            "--apply",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Applied critical dependency pin updates",
        ));
    let body = std::fs::read_to_string(&body_path).unwrap();
    assert!(body.contains("target: TERM-1"));
    assert!(body.contains("scope: block"));
    assert!(body.contains("locator: contract"));
    assert!(body.contains("[[TERM-1#contract]]"));
    assert!(!body.contains("@sha256:"));
    assert!(body.contains("updatedBy: alice"));
    assert!(body.contains("reason: reviewed upstream contract"));
    let audit =
        std::fs::read_to_string(temp.path().join(".docs-hygiene/pin-updates.jsonl")).unwrap();
    let record: Value = serde_json::from_str(audit.trim()).unwrap();
    assert_eq!(record["target"], "TERM-1");
    assert_eq!(record["updatedBy"], "alice");
    assert_eq!(record["reason"], "reviewed upstream contract");
    assert!(record["newDigest"].as_str().unwrap().len() == 64);

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap(), "--fail-on-warning"])
        .assert()
        .success();
}

#[test]
fn pin_update_supports_partial_targets_and_invalid_selection_writes_nothing() {
    let zeros = "0".repeat(64);
    let body = format!(
        "---\nid: BODY-ITEM\nstatus: proposed\nanchors:\n  - target: TERM-1\n    algorithm: sha256\n    digest: {zeros}\n    scope: block\n    locator: contract\n    updatedAt: 2000-01-01\n    updatedBy: old\n    reason: old\n  - target: TERM-2\n    algorithm: sha256\n    digest: {zeros}\n    scope: block\n    locator: contract\n    updatedAt: 2000-01-01\n    updatedBy: old\n    reason: old\n---\n\n# Body\n"
    );
    let temp = write_project(&["TERM-1", "TERM-2"], &body);
    let body_path = temp.path().join("docs/prd/body.md");
    let before = std::fs::read(&body_path).unwrap();
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "update-pins",
            temp.path().to_str().unwrap(),
            "--target",
            "MISSING",
            "--actor",
            "alice",
            "--reason",
            "invalid selection",
            "--apply",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no changes were applied"));
    assert_eq!(std::fs::read(&body_path).unwrap(), before);

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "update-pins",
            temp.path().to_str().unwrap(),
            "--target",
            "TERM-1",
            "--actor",
            "alice",
            "--reason",
            "partial refresh",
            "--apply",
        ])
        .assert()
        .success();
    let updated = std::fs::read_to_string(&body_path).unwrap();
    assert_eq!(updated.matches(&zeros).count(), 1);
    assert!(updated.contains("reason: partial refresh"));
    assert!(updated.contains("reason: old"));
}

#[test]
fn pin_update_can_pin_a_selected_vertical_relation_between_body_kinds() {
    let temp = tempdir().unwrap();
    for package in ["docs/prd", "docs/spec"] {
        std::fs::create_dir_all(temp.path().join(package)).unwrap();
    }
    std::fs::write(
        temp.path().join("docs/prd/manifest.yml"),
        "id: PRD-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: baselined\nmembers: [body.md]\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("docs/prd/body.md"),
        "---\nid: PRD-BODY\nstatus: baselined\n---\n\n# PRD\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("docs/spec/manifest.yml"),
        "id: SPEC-1\nrefinementLevel: definition\nreferenceRelation: body\nstatus: proposed\nformalizes: PRD-1\nmembers: [body.md]\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("docs/spec/body.md"),
        "---\nid: SPEC-BODY\nstatus: proposed\n---\n\n# Spec\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        r#"
governance:
  manifests: [docs/prd/manifest.yml, docs/spec/manifest.yml]
  criticalDependencies:
    - id: formalization-pin
      match:
        sourceKinds: [body]
        targetKinds: [body]
        relations: [formalizes]
        sourceIds: [SPEC-1]
        targetIds: [PRD-1]
      require:
        algorithms: [sha256]
        minimumScope: file
rules:
  governance.identity: { mode: required }
  governance.traceability: { mode: required }
"#,
    )
    .unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "update-pins",
            temp.path().to_str().unwrap(),
            "--actor",
            "alice",
            "--reason",
            "pin formalized intent",
            "--apply",
        ])
        .assert()
        .success();
    let source = std::fs::read_to_string(temp.path().join("docs/spec/body.md")).unwrap();
    assert!(source.contains("target: PRD-1"));
    assert!(source.contains("scope: file"));

    let output = Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap(), "--format", "json"])
        .assert()
        .get_output()
        .stdout
        .clone();
    let report: Value = serde_json::from_slice(&output).unwrap();
    assert!(
        !report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|diagnostic| diagnostic["code"].as_str().unwrap().starts_with("DH_PIN_"))
    );
}
