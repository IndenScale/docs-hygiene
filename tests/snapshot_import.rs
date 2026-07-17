use std::process::Command as ProcessCommand;

use assert_cmd::Command;
use serde_json::Value;
use sha2::{Digest, Sha256};
use tempfile::tempdir;

fn git(source: &std::path::Path, args: &[&str]) -> String {
    let output = ProcessCommand::new("git")
        .args(["-C"])
        .arg(source)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

#[test]
fn explicit_import_materializes_payload_then_check_remains_offline() {
    let source = tempdir().unwrap();
    std::fs::create_dir_all(source.path().join("docs")).unwrap();
    let term = "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n\nPortable bytes.\n";
    std::fs::write(source.path().join("docs/term.md"), term).unwrap();
    git(source.path(), &["init", "--quiet"]);
    git(
        source.path(),
        &["config", "user.email", "snapshot@example.test"],
    );
    git(source.path(), &["config", "user.name", "Snapshot Test"]);
    git(source.path(), &["add", "docs/term.md"]);
    git(
        source.path(),
        &["commit", "--quiet", "-m", "snapshot source"],
    );
    let commit = git(source.path(), &["rev-parse", "HEAD"]);
    let digest = format!("{:x}", Sha256::digest(term.as_bytes()));

    let project = tempdir().unwrap();
    for path in ["docs/ul", "docs/prd", "snapshots"] {
        std::fs::create_dir_all(project.path().join(path)).unwrap();
    }
    std::fs::write(
        project.path().join("docs/ul/manifest.yml"),
        "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [term.md]\n",
    )
    .unwrap();
    std::fs::write(project.path().join("docs/ul/term.md"), term).unwrap();
    std::fs::write(
        project.path().join("docs/prd/manifest.yml"),
        "id: BODY-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nmembers: [body.md]\n",
    )
    .unwrap();
    std::fs::write(
        project.path().join("docs/prd/body.md"),
        format!(
            "---\nid: BODY-ITEM\nstatus: proposed\nanchors:\n  - target: TERM-1\n    algorithm: sha256\n    digest: {digest}\n    scope: file\n    snapshot:\n      manifest: vendor-release-1\n      repository: github:vendor/docs\n      commit: {commit}\n      path: docs/term.md\n---\n\n# Body\n"
        ),
    )
    .unwrap();
    std::fs::write(
        project.path().join("snapshots/vendor.yml"),
        format!(
            "schemaVersion: docs-hygiene.snapshot.v1\nid: vendor-release-1\nrepository: github:vendor/docs\ncommit: {commit}\nstatus: active\nreplacedBy: null\nretainUntil: 2030-01-01\nentries:\n  - target: TERM-1\n    path: docs/term.md\n    payload: payload/term.md\n    scope: file\n    locator: null\n    digest: {digest}\nsignature: null\n"
        ),
    )
    .unwrap();
    std::fs::write(
        project.path().join("docs-hygiene.yml"),
        "governance:\n  manifests: [docs/ul/manifest.yml, docs/prd/manifest.yml]\n  portableSnapshots:\n    manifests: [snapshots/vendor.yml]\nrules:\n  governance.identity: { mode: required }\n  governance.traceability: { mode: required }\n",
    )
    .unwrap();
    let payload = project.path().join("snapshots/payload/term.md");

    let output = Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "import-snapshot",
            project.path().to_str().unwrap(),
            "--manifest",
            "snapshots/vendor.yml",
            "--source",
            source.path().to_str().unwrap(),
            "--format",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let plan: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(plan["schemaVersion"], "docs-hygiene.snapshot-import.v1");
    assert_eq!(plan["changes"][0]["target"], "TERM-1");
    assert!(!payload.exists());

    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "import-snapshot",
            project.path().to_str().unwrap(),
            "--manifest",
            "snapshots/vendor.yml",
            "--source",
            source.path().to_str().unwrap(),
            "--apply",
        ])
        .assert()
        .success();
    assert_eq!(std::fs::read_to_string(&payload).unwrap(), term);

    let source_path = source.keep();
    std::fs::remove_dir_all(source_path).unwrap();
    Command::cargo_bin("docs-hygiene")
        .unwrap()
        .args([
            "check",
            project.path().to_str().unwrap(),
            "--fail-on-warning",
        ])
        .assert()
        .success();
}
