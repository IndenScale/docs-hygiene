    fn write_core_claim_fixture(root: &Path, body: &str) {
        fs::create_dir_all(root.join("docs/ul")).unwrap();
        fs::create_dir_all(root.join("docs/prd")).unwrap();
        fs::write(
            root.join("docs/ul/manifest.yml"),
            "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [term.md]\n",
        )
        .unwrap();
        fs::write(
            root.join("docs/ul/term.md"),
            "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n\n## Canonical Claim\n\nA reliable retry policy uses bounded exponential backoff and deterministic jitter.\n",
        )
        .unwrap();
        fs::write(
            root.join("docs/prd/manifest.yml"),
            "id: BODY-1\nrefinementLevel: intent\nreferenceRelation: body\nstatus: proposed\nmembers: [body.md]\n",
        )
        .unwrap();
        fs::write(root.join("docs/prd/body.md"), body).unwrap();
    }

    fn core_claim_config(claims: &str) -> Config {
        serde_yaml::from_str(&format!(
            r#"
governance:
  manifests:
    - docs/ul/manifest.yml
    - docs/prd/manifest.yml
  coreClaims:
{claims}
rules:
  governance.identity:
    mode: required
  governance.traceability:
    mode: disabled
"#
        ))
        .unwrap()
    }

    #[test]
    fn explicit_claim_policies_report_forbidden_expired_and_unpinned_occurrences() {
        let temp = tempdir().unwrap();
        write_core_claim_fixture(
            temp.path(),
            "---\nid: BODY-ITEM\nstatus: proposed\n---\n\n# Body\n\n[[TERM-1]]\n\n## Forbidden Copy\n\nCopied definition.\n\n## Expired Copy\n\nCopied definition.\n\n## Excerpt\n\nControlled text without a pin.\n",
        );
        let config = core_claim_config(
            r#"    - id: retry-policy
      authority: { id: TERM-1, selector: canonical-claim }
      occurrences:
        - path: docs/prd/body.md
          selector: forbidden-copy
          policy: forbidden
        - path: docs/prd/body.md
          selector: expired-copy
          policy: migrate
          migrateBy: 2000-01-01
        - path: docs/prd/body.md
          selector: excerpt
          policy: controlledExcerpt"#,
        );

        let report = run_checks(temp.path(), &config).unwrap();
        let claims = report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "DH_CLAIM_001")
            .collect::<Vec<_>>();
        assert!(claims.iter().any(|diagnostic| {
            diagnostic.message.contains("duplicate definition")
                && diagnostic.message.contains("is forbidden")
        }));
        assert!(claims.iter().any(|diagnostic| {
            diagnostic.message.contains("passed its migration deadline 2000-01-01")
        }));
        assert!(claims.iter().any(|diagnostic| {
            diagnostic.message.contains("must declare a block-scope frontmatter anchor")
        }));
        assert!(claims.iter().all(|diagnostic| diagnostic.path == "docs/prd/body.md"));
        assert!(claims.iter().all(|diagnostic| diagnostic.range.start.line > 0));
    }

    #[test]
    fn stale_controlled_excerpt_reports_expected_authority_digest() {
        let temp = tempdir().unwrap();
        write_core_claim_fixture(
            temp.path(),
            &format!(
                "---\nid: BODY-ITEM\nstatus: proposed\nanchors:\n  - target: TERM-1\n    algorithm: sha256\n    digest: {}\n    scope: block\n    locator: canonical-claim\n---\n\n# Body\n\n[[TERM-1]]\n\n## Excerpt\n\nStale controlled text.\n",
                "0".repeat(64)
            ),
        );
        let config = core_claim_config(
            r#"    - id: retry-policy
      authority: { id: TERM-1, selector: canonical-claim }
      occurrences:
        - path: docs/prd/body.md
          selector: excerpt
          policy: controlledExcerpt"#,
        );

        let report = run_checks(temp.path(), &config).unwrap();
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_CLAIM_001"
                && diagnostic.message.contains("is stale")
                && diagnostic.message.contains("refresh the pin")
        }));
    }

    #[test]
    fn valid_controlled_excerpt_is_a_pinned_dependency_with_transitive_impact() {
        let temp = tempdir().unwrap();
        let term = "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n\n## Canonical Claim\n\nA reliable retry policy uses bounded exponential backoff and deterministic jitter.\n";
        let hash = format!(
            "{:x}",
            Sha256::digest(markdown_heading_block(term, "canonical-claim").unwrap())
        );
        write_core_claim_fixture(
            temp.path(),
            &format!(
                "---\nid: BODY-ITEM\nstatus: proposed\nanchors:\n  - target: TERM-1\n    algorithm: sha256\n    digest: {hash}\n    scope: block\n    locator: canonical-claim\n---\n\n# Body\n\n[[TERM-1]]\n\n## Excerpt\n\nReviewed controlled excerpt.\n"
            ),
        );
        let config = core_claim_config(
            r#"    - id: retry-policy
      authority: { id: TERM-1, selector: canonical-claim }
      occurrences:
        - path: docs/prd/body.md
          selector: excerpt
          policy: controlledExcerpt"#,
        );

        let report = run_checks(temp.path(), &config).unwrap();
        assert!(!report.diagnostics.iter().any(|diagnostic| {
            matches!(diagnostic.code, "DH_CLAIM_001" | "DH_REFERENCE_001")
        }), "{:?}", report.diagnostics);
        assert!(report.governance_graph.edges.iter().any(|edge| {
            edge.source == "BODY-1"
                && edge.target == "TERM-1"
                && edge.relation == GovernanceEdgeKind::PinnedReference
                && edge.content_anchor.as_ref().is_some_and(|anchor| {
                    anchor.scope == ContentAnchorScope::Block
                        && anchor.locator.as_deref() == Some("canonical-claim")
                })
        }));
        assert_eq!(
            report.governance_graph.transitive_impact.get("TERM-1"),
            Some(&vec!["BODY-1".to_owned()])
        );
    }

    #[test]
    fn duplicate_claim_authorities_and_superseded_authority_have_remediation() {
        let temp = tempdir().unwrap();
        write_core_claim_fixture(
            temp.path(),
            "---\nid: BODY-ITEM\nstatus: proposed\n---\n\n# Body\n\n[[TERM-2]]\n",
        );
        fs::write(
            temp.path().join("docs/ul/manifest.yml"),
            "id: UL-1\nrefinementLevel: intent\nreferenceRelation: library\nstatus: baselined\nmembers: [term.md, successor.md]\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("docs/ul/term.md"),
            "---\nid: TERM-1\nstatus: superseded\nsupersededBy: TERM-2\n---\n\n# Term\n\n## Canonical Claim\n\nOld definition.\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("docs/ul/successor.md"),
            "---\nid: TERM-2\nstatus: baselined\n---\n\n# Successor\n\n## Canonical Claim\n\nNew definition.\n",
        )
        .unwrap();
        let config = core_claim_config(
            r#"    - id: retry-policy
      authority: { id: TERM-1, selector: canonical-claim }
    - id: retry-policy
      authority: { id: TERM-2, selector: canonical-claim }"#,
        );

        let report = run_checks(temp.path(), &config).unwrap();
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_CLAIM_001"
                && diagnostic.message.contains("multiple canonical Library authorities")
        }));
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_CLAIM_001"
                && diagnostic.message.contains("set authority.id to 'TERM-2'")
        }));
    }
