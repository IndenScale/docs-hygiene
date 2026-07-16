use std::path::PathBuf;

use docs_hygiene::{Config, run_checks};

#[test]
fn repository_policy_and_governance_graph_pass() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut config = Config::load(&root.join("docs-hygiene.yml")).unwrap();

    // External adapters have their own environment dependencies. This test
    // exercises the repository's complete native policy and governance graph.
    config.adapters.markdownlint.enabled = false;

    let report = run_checks(&root, &config).unwrap();
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
}
