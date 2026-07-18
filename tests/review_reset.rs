use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::tempdir;

fn policy() -> &'static str {
    r#"governance:
  manifests: [asset.yml, other.yml]
  ownership:
    enabled: true
    confirmationMaxAgeDays: 50000
    resetAuditLog: .docs-hygiene/review-resets.jsonl
    principals:
      - { id: "person:alice", kind: person }
      - { id: "person:bob", kind: person }
"#
}

fn identity(id: &str, review_by: &str) -> String {
    format!(
        r#"id: {id}
referenceRelation: body
status: current
ownership:
  owner: person:alice
  understoodBy:
    - {{ principal: person:alice, confirmedAt: 2000-01-01 }}
    - {{ principal: person:bob, confirmedAt: 2000-01-02 }}
review:
  reviewBy: {review_by}
"#
    )
}

#[test]
fn review_reset_is_dry_run_by_default_then_applies_one_target_with_audit() {
    let temp = tempdir().unwrap();
    std::fs::write(temp.path().join("docs-hygiene.yml"), policy()).unwrap();
    let original = identity("ASSET-1", "2000-01-01");
    let other = identity("ASSET-2", "2098-12-31");
    std::fs::write(temp.path().join("asset.yml"), &original).unwrap();
    std::fs::write(temp.path().join("other.yml"), &other).unwrap();

    let output = Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap(), "--format", "json"])
        .assert()
        .failure()
        .get_output()
        .stdout
        .clone();
    let check: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(
        check["ownership"]["responsibilityCoverage"]["percentage"],
        100
    );
    assert_eq!(
        check["ownership"]["knowledgeRedundancyCoverage"]["percentage"],
        100
    );
    assert_eq!(check["ownership"]["reviewsExpired"], 1);

    let output = Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "reset-review",
            "ASSET-1",
            "--root",
            temp.path().to_str().unwrap(),
            "--actor",
            "person:bob",
            "--reason",
            "quarterly semantic review",
            "--review-by",
            "2099-12-31",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let dry_run: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(dry_run["applied"], false);
    assert_eq!(dry_run["changes"][0]["identity"], "ASSET-1");
    assert_eq!(
        std::fs::read_to_string(temp.path().join("asset.yml")).unwrap(),
        original
    );
    assert!(
        !temp
            .path()
            .join(".docs-hygiene/review-resets.jsonl")
            .exists()
    );

    let output = Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "reset-review",
            "ASSET-1",
            "--root",
            temp.path().to_str().unwrap(),
            "--actor",
            "person:bob",
            "--reason",
            "quarterly semantic review",
            "--review-by",
            "2099-12-31",
            "--apply",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let applied: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(applied["applied"], true);
    let target = std::fs::read_to_string(temp.path().join("asset.yml")).unwrap();
    assert!(target.contains("reviewBy: 2099-12-31"));
    assert!(target.contains("lastReset:"));
    assert!(target.contains("by: person:bob"));
    assert!(target.contains("reason: quarterly semantic review"));
    assert_eq!(
        std::fs::read_to_string(temp.path().join("other.yml")).unwrap(),
        other
    );
    let audit =
        std::fs::read_to_string(temp.path().join(".docs-hygiene/review-resets.jsonl")).unwrap();
    let record: Value = serde_json::from_str(audit.trim()).unwrap();
    assert_eq!(record["identity"], "ASSET-1");
    assert_eq!(record["reviewBy"], "2099-12-31");
}

#[test]
fn invalid_review_reset_is_atomic_and_writes_no_audit() {
    let temp = tempdir().unwrap();
    std::fs::write(temp.path().join("docs-hygiene.yml"), policy()).unwrap();
    let original = identity("ASSET-1", "2000-01-01");
    std::fs::write(temp.path().join("asset.yml"), &original).unwrap();
    std::fs::write(
        temp.path().join("other.yml"),
        identity("ASSET-2", "2098-12-31"),
    )
    .unwrap();

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "reset-review",
            "ASSET-1",
            "--root",
            temp.path().to_str().unwrap(),
            "--actor",
            "person:missing",
            "--reason",
            "review",
            "--review-by",
            "2099-12-31",
            "--apply",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("Blocked review reset"));

    assert_eq!(
        std::fs::read_to_string(temp.path().join("asset.yml")).unwrap(),
        original
    );
    assert!(
        !temp
            .path()
            .join(".docs-hygiene/review-resets.jsonl")
            .exists()
    );

    let malformed = original.replace("review:\n  reviewBy: 2000-01-01", "review: invalid");
    std::fs::write(temp.path().join("asset.yml"), &malformed).unwrap();
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "reset-review",
            "ASSET-1",
            "--root",
            temp.path().to_str().unwrap(),
            "--actor",
            "person:bob",
            "--reason",
            "review",
            "--review-by",
            "2099-12-31",
            "--apply",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("cannot be updated"));
    assert_eq!(
        std::fs::read_to_string(temp.path().join("asset.yml")).unwrap(),
        malformed
    );
    assert!(
        !temp
            .path()
            .join(".docs-hygiene/review-resets.jsonl")
            .exists()
    );
}
