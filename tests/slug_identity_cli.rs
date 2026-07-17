use assert_cmd::Command;
use serde_json::Value;
use tempfile::tempdir;

#[test]
fn slug_diagnostic_json_exposes_normalization_conflict_and_remediation_data() {
    let temp = tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join("docs")).unwrap();
    std::fs::write(
        temp.path().join("docs-hygiene.yml"),
        r#"
docs:
  bases:
    - id: articles
      root: docs
      patterns:
        - id: article
          regex: "^.+\\.md$"
          documentKind: article
  slugSchemas:
    - documentKind: article
      source: { type: frontmatter, field: slug }
      pattern: "^[a-z][a-z0-9-]*$"
      reserved: [admin]
      normalization: lowercaseKebab
"#,
    )
    .unwrap();
    std::fs::write(
        temp.path().join("docs/admin.md"),
        "---\nid: DOC-1\nslug: Admin\n---\n\n# Admin\n",
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
    let diagnostic = value["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|diagnostic| diagnostic["code"] == "DH_SLUG_001")
        .expect("slug diagnostic");

    assert_eq!(diagnostic["data"]["originalValue"], "Admin");
    assert_eq!(diagnostic["data"]["normalizedValue"], "admin");
    assert_eq!(diagnostic["data"]["documentKind"], "article");
    assert!(diagnostic["data"]["conflictPath"].is_null());
    assert!(
        diagnostic["data"]["remediation"]
            .as_str()
            .unwrap()
            .contains("non-reserved")
    );
}
