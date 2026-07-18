use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::tempdir;

#[test]
fn explicit_topology_threshold_blocks_until_an_audited_exception_applies() {
    let temp = tempdir().unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        "governance:\n  manifests: [ul.yml, a/manifest.yml, b/manifest.yml]\n  topology:\n    maxFanIn: 1\n",
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
    for name in ["a", "b"] {
        std::fs::create_dir(temp.path().join(name)).unwrap();
        std::fs::write(
            temp.path().join(format!("{name}/manifest.yml")),
            format!(
                "id: PRD-{name}\nreferenceRelation: body\nstatus: proposed\nmembers: [index.md]\n"
            ),
        )
        .unwrap();
        std::fs::write(
            temp.path().join(format!("{name}/index.md")),
            format!("---\nid: PRD-{name}-INDEX\nstatus: proposed\n---\n\n# Body\n\n[[UL-1]]\n"),
        )
        .unwrap();
    }

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args(["check", temp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("DH_TOPOLOGY_001 Error"))
        .stdout(predicate::str::contains("Fan-In 2"));

    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        "governance:\n  manifests: [ul.yml, a/manifest.yml, b/manifest.yml]\n  topology:\n    maxFanIn: 1\n    exceptions:\n      - id: shared-ul\n        node: UL-1\n        direction: fanIn\n        budget: 3\n        reason: public Library\n        owner: docs-platform\n        approvedBy: architecture-council\n        expires: 2099-12-31\n        history: [{ observedAt: 2026-01-01, degree: 1 }]\n",
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
    let report: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(report["topologyExceptions"][0]["status"], "applied");
    assert_eq!(report["topologyExceptions"][0]["currentDegree"], 2);
    assert_eq!(report["topologyExceptions"][0]["remaining"], 1);
    assert_eq!(report["topologyExceptions"][0]["trendDelta"], 1);
}
