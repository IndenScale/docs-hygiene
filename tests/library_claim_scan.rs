use assert_cmd::Command;
use serde_json::Value;
use tempfile::tempdir;

#[test]
fn candidate_scan_reports_evidence_without_turning_similarity_into_a_gate() {
    let temp = tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join("docs/ul")).unwrap();
    std::fs::create_dir_all(temp.path().join("docs/prd")).unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        r#"
governance:
  manifests:
    - docs/ul/manifest.yml
    - docs/prd/manifest.yml
  coreClaims:
    - id: retry-policy
      authority: { id: TERM-1, selector: canonical-claim }
      candidatePaths: [docs/prd/*.md]
      similarityThreshold: 0.8
rules:
  governance.identity:
    mode: required
  governance.traceability:
    mode: disabled
"#,
    )
    .unwrap();
    std::fs::write(
        temp.path().join("docs/ul/manifest.yml"),
        "id: UL-1\nreferenceRelation: library\nstatus: baselined\nmembers: [term.md]\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("docs/ul/term.md"),
        "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n\n## Canonical Claim\n\nA reliable retry policy uses bounded exponential backoff and deterministic jitter.\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("docs/prd/manifest.yml"),
        "id: BODY-1\nreferenceRelation: body\nstatus: proposed\nmembers: [body.md]\n",
    )
    .unwrap();
    std::fs::write(
        temp.path().join("docs/prd/body.md"),
        "---\nid: BODY-ITEM\nstatus: proposed\n---\n\n# Body\n\n[[TERM-1]]\n\n## Copied Claim\n\nA reliable retry policy uses bounded exponential backoff and deterministic jitter.\n",
    )
    .unwrap();

    let output = Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "scan-library-claims",
            temp.path().to_str().unwrap(),
            "--format",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let report: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(
        report["schemaVersion"],
        "docs-hygiene.library-claim-scan.v1"
    );
    assert_eq!(report["candidates"][0]["claim"], "retry-policy");
    assert_eq!(report["candidates"][0]["candidatePath"], "docs/prd/body.md");
    assert_eq!(report["candidates"][0]["candidateSelector"], "copied-claim");
    assert!(report["candidates"][0]["similarity"].as_f64().unwrap() >= 0.8);
    assert!(
        report["candidates"][0]["candidateFragment"]
            .as_str()
            .unwrap()
            .contains("bounded exponential backoff")
    );

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap(), "--fail-on-warning"])
        .assert()
        .success();
}
