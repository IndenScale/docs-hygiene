    fn snapshot_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn write_snapshot_project(
        targets: &[(&str, &str)],
        anchors: &str,
        manifest: &PortableSnapshotManifest,
        require_signature: bool,
        trusted_key: Option<&str>,
    ) -> (tempfile::TempDir, Config) {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join("docs/ul")).unwrap();
        fs::create_dir_all(temp.path().join("docs/prd")).unwrap();
        fs::create_dir_all(temp.path().join("snapshots/payload")).unwrap();
        let members = targets
            .iter()
            .map(|(id, _)| format!("{}.md", id.to_ascii_lowercase()))
            .collect::<Vec<_>>();
        fs::write(
            temp.path().join("docs/ul/manifest.yml"),
            format!(
                "id: UL-1\nreferenceRelation: library\nstatus: baselined\nmembers: [{}]\n",
                members.join(", ")
            ),
        )
        .unwrap();
        for (id, content) in targets {
            let member = format!("{}.md", id.to_ascii_lowercase());
            fs::write(temp.path().join("docs/ul").join(&member), content).unwrap();
            fs::write(temp.path().join("snapshots/payload").join(member), content).unwrap();
        }
        fs::write(
            temp.path().join("docs/prd/manifest.yml"),
            "id: BODY-1\nreferenceRelation: body\nstatus: proposed\nmembers: [body.md]\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("docs/prd/body.md"),
            format!("---\nid: BODY-ITEM\nstatus: proposed\nanchors:\n{anchors}---\n\n# Body\n"),
        )
        .unwrap();
        fs::write(
            temp.path().join("snapshots/vendor.yml"),
            serde_yaml::to_string(manifest).unwrap(),
        )
        .unwrap();
        let trusted = trusted_key
            .map(|key| format!("\n      release: {key}"))
            .unwrap_or_else(|| " {}".to_owned());
        let config: Config = serde_yaml::from_str(&format!(
            "governance:\n  manifests: [docs/ul/manifest.yml, docs/prd/manifest.yml]\n  portableSnapshots:\n    manifests: [snapshots/vendor.yml]\n    requireSignatures: {require_signature}\n    trustedKeys:{trusted}\nrules:\n  governance.identity: {{ mode: required }}\n  governance.traceability: {{ mode: required }}\n"
        ))
        .unwrap();
        (temp, config)
    }

    fn snapshot_manifest(entries: Vec<PortableSnapshotEntry>) -> PortableSnapshotManifest {
        PortableSnapshotManifest {
            schema_version: PORTABLE_SNAPSHOT_SCHEMA_VERSION.to_owned(),
            id: "vendor-release-1".to_owned(),
            repository: "github:vendor/docs".to_owned(),
            commit: "a".repeat(40),
            status: PortableSnapshotStatus::Active,
            replaced_by: None,
            retain_until: Some("2030-01-01".to_owned()),
            entries,
            signature: None,
        }
    }

    #[test]
    fn signed_portable_file_and_block_snapshots_verify_offline_without_git() {
        use ed25519_dalek::Signer;

        let term_1 = "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term One\n";
        let term_2 = "---\nid: TERM-2\nstatus: baselined\n---\n\n# Term Two\n\n## Contract\n\nStable block.\n\n## Notes\n\nMutable.\n";
        let file_digest = format!("{:x}", Sha256::digest(term_1.as_bytes()));
        let block_digest = format!(
            "{:x}",
            Sha256::digest(markdown_heading_block(term_2, "contract").unwrap())
        );
        let mut manifest = snapshot_manifest(vec![
            PortableSnapshotEntry {
                target: "TERM-1".to_owned(),
                path: "docs/term-one.md".to_owned(),
                payload: "payload/term-1.md".to_owned(),
                scope: ContentAnchorScope::File,
                locator: None,
                digest: file_digest.clone(),
            },
            PortableSnapshotEntry {
                target: "TERM-2".to_owned(),
                path: "docs/term-two.md".to_owned(),
                payload: "payload/term-2.md".to_owned(),
                scope: ContentAnchorScope::Block,
                locator: Some("contract".to_owned()),
                digest: block_digest.clone(),
            },
        ]);
        let signing = ed25519_dalek::SigningKey::from_bytes(&[7; 32]);
        let signature = signing.sign(&manifest.signing_bytes().unwrap());
        manifest.signature = Some(crate::PortableSnapshotSignature {
            algorithm: "ed25519".to_owned(),
            key_id: "release".to_owned(),
            value: snapshot_hex(&signature.to_bytes()),
        });
        let anchors = format!(
            "  - target: TERM-1\n    algorithm: sha256\n    digest: {file_digest}\n    scope: file\n    snapshot:\n      manifest: vendor-release-1\n      repository: github:vendor/docs\n      commit: {}\n      path: docs/term-one.md\n  - target: TERM-2\n    algorithm: sha256\n    digest: {block_digest}\n    scope: block\n    locator: contract\n    snapshot:\n      manifest: vendor-release-1\n      repository: github:vendor/docs\n      commit: {}\n      path: docs/term-two.md\n",
            "a".repeat(40),
            "a".repeat(40)
        );
        let key = snapshot_hex(signing.verifying_key().as_bytes());
        let (temp, config) = write_snapshot_project(
            &[("TERM-1", term_1), ("TERM-2", term_2)],
            &anchors,
            &manifest,
            true,
            Some(&key),
        );

        let report = run_checks(temp.path(), &config).unwrap();
        assert!(
            report
                .diagnostics
                .iter()
                .all(|diagnostic| !diagnostic.code.starts_with("DH_SNAPSHOT_")
                    && diagnostic.code != "DH_REFERENCE_001"),
            "{:?}",
            report.diagnostics
        );
    }

    #[test]
    fn snapshot_provenance_reports_repository_commit_path_and_digest_mismatches() {
        let term = "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n";
        let digest = format!("{:x}", Sha256::digest(term.as_bytes()));
        let manifest = snapshot_manifest(vec![PortableSnapshotEntry {
            target: "TERM-1".to_owned(),
            path: "docs/term.md".to_owned(),
            payload: "payload/term-1.md".to_owned(),
            scope: ContentAnchorScope::File,
            locator: None,
            digest,
        }]);
        let wrong_digest = "0".repeat(64);
        let anchors = format!(
            "  - target: TERM-1\n    algorithm: sha256\n    digest: {wrong_digest}\n    scope: file\n    snapshot:\n      manifest: vendor-release-1\n      repository: github:other/docs\n      commit: {}\n      path: docs/other.md\n",
            "b".repeat(40)
        );
        let (temp, config) =
            write_snapshot_project(&[("TERM-1", term)], &anchors, &manifest, false, None);

        let report = run_checks(temp.path(), &config).unwrap();
        let codes = report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<BTreeSet<_>>();
        for code in [
            "DH_SNAPSHOT_002",
            "DH_SNAPSHOT_003",
            "DH_SNAPSHOT_004",
            "DH_SNAPSHOT_005",
        ] {
            assert!(codes.contains(code), "missing {code}: {:?}", report.diagnostics);
        }

        let isolated = tempdir().unwrap();
        let config: Config = serde_yaml::from_str(
            "governance:\n  portableSnapshots:\n    manifests: [snapshots/missing.yml]\n",
        )
        .unwrap();
        let report = run_checks(isolated.path(), &config).unwrap();
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "DH_SNAPSHOT_001")
        );
    }

    #[test]
    fn tampering_and_inactive_snapshot_lifecycle_are_rejected() {
        use ed25519_dalek::Signer;

        let term = "---\nid: TERM-1\nstatus: baselined\n---\n\n# Term\n";
        let digest = format!("{:x}", Sha256::digest(term.as_bytes()));
        let mut manifest = snapshot_manifest(vec![PortableSnapshotEntry {
            target: "TERM-1".to_owned(),
            path: "docs/term.md".to_owned(),
            payload: "payload/term-1.md".to_owned(),
            scope: ContentAnchorScope::File,
            locator: None,
            digest: digest.clone(),
        }]);
        let signing = ed25519_dalek::SigningKey::from_bytes(&[9; 32]);
        manifest.signature = Some(crate::PortableSnapshotSignature {
            algorithm: "ed25519".to_owned(),
            key_id: "release".to_owned(),
            value: snapshot_hex(&signing.sign(&manifest.signing_bytes().unwrap()).to_bytes()),
        });
        manifest.status = PortableSnapshotStatus::Revoked;
        let anchors = format!(
            "  - target: TERM-1\n    algorithm: sha256\n    digest: {digest}\n    scope: file\n    snapshot:\n      manifest: vendor-release-1\n      repository: github:vendor/docs\n      commit: {}\n      path: docs/term.md\n",
            "a".repeat(40)
        );
        let key = snapshot_hex(signing.verifying_key().as_bytes());
        let (temp, config) = write_snapshot_project(
            &[("TERM-1", term)],
            &anchors,
            &manifest,
            true,
            Some(&key),
        );

        let report = run_checks(temp.path(), &config).unwrap();
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "DH_SNAPSHOT_006")
        );
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "DH_SNAPSHOT_007")
        );
    }
