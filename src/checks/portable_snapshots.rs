#[derive(Debug)]
struct LoadedSnapshot {
    path: String,
    manifest: PortableSnapshotManifest,
}

fn check_portable_snapshots(
    root: &Path,
    config: &Config,
    graph: &GovernanceGraph,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let has_snapshot_anchor = graph.edges.iter().any(|edge| {
        edge.content_anchor
            .as_ref()
            .is_some_and(|anchor| anchor.snapshot.is_some())
    });
    if config.governance.portable_snapshots.manifests.is_empty() && !has_snapshot_anchor {
        return;
    }
    validate_trusted_snapshot_keys(config, diagnostics);
    let mut snapshots = BTreeMap::<String, LoadedSnapshot>::new();
    for rel in &config.governance.portable_snapshots.manifests {
        let path = rel.display().to_string();
        let text = match std::fs::read_to_string(root.join(rel)) {
            Ok(text) => text,
            Err(error) => {
                push_snapshot_diagnostic(
                    "DH_SNAPSHOT_001",
                    path,
                    format!("Portable snapshot manifest cannot be read: {error}."),
                    diagnostics,
                );
                continue;
            }
        };
        let manifest = match serde_yaml::from_str::<PortableSnapshotManifest>(&text) {
            Ok(manifest) => manifest,
            Err(error) => {
                push_snapshot_diagnostic(
                    "DH_SNAPSHOT_001",
                    path,
                    format!("Portable snapshot manifest is invalid: {error}."),
                    diagnostics,
                );
                continue;
            }
        };
        validate_snapshot_manifest(root, config, rel, &manifest, diagnostics);
        if snapshots.contains_key(&manifest.id) {
            push_snapshot_diagnostic(
                "DH_SNAPSHOT_001",
                path,
                format!("Portable snapshot identity '{}' is duplicated.", manifest.id),
                diagnostics,
            );
            continue;
        }
        snapshots.insert(manifest.id.clone(), LoadedSnapshot { path, manifest });
    }
    validate_snapshot_replacements(&snapshots, diagnostics);
    for edge in &graph.edges {
        let Some(anchor) = &edge.content_anchor else {
            continue;
        };
        let Some(provenance) = &anchor.snapshot else {
            continue;
        };
        validate_snapshot_anchor(edge, anchor, provenance, &snapshots, diagnostics);
    }
}

fn validate_snapshot_manifest(
    root: &Path,
    config: &Config,
    rel: &Path,
    manifest: &PortableSnapshotManifest,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let path = rel.display().to_string();
    if manifest.schema_version != PORTABLE_SNAPSHOT_SCHEMA_VERSION
        || !valid_claim_identity(&manifest.id)
        || manifest.entries.is_empty()
    {
        push_snapshot_diagnostic(
            "DH_SNAPSHOT_001",
            path.clone(),
            format!(
                "Portable snapshot requires schemaVersion '{}', a stable id, and at least one entry.",
                PORTABLE_SNAPSHOT_SCHEMA_VERSION
            ),
            diagnostics,
        );
    }
    if !valid_repository_identity(&manifest.repository) {
        push_snapshot_diagnostic(
            "DH_SNAPSHOT_002",
            path.clone(),
            "Portable snapshot repository must be a credential-free stable identity, not a URL."
                .to_owned(),
            diagnostics,
        );
    }
    if !valid_commit_oid(&manifest.commit) {
        push_snapshot_diagnostic(
            "DH_SNAPSHOT_003",
            path.clone(),
            "Portable snapshot commit must be a full 40- or 64-hex object ID.".to_owned(),
            diagnostics,
        );
    }
    validate_snapshot_lifecycle(manifest, &path, diagnostics);
    validate_snapshot_signature(config, manifest, &path, diagnostics);
    let mut entries = BTreeSet::new();
    for entry in &manifest.entries {
        let key = format!("{}:{:?}:{:?}", entry.target, entry.scope, entry.locator);
        if !valid_claim_identity(&entry.target) || !entries.insert(key) {
            push_snapshot_diagnostic(
                "DH_SNAPSHOT_001",
                path.clone(),
                format!(
                    "Portable snapshot '{}' has an invalid or duplicate entry for '{}'.",
                    manifest.id, entry.target
                ),
                diagnostics,
            );
        }
        validate_snapshot_entry(root, rel, manifest, entry, diagnostics);
    }
}

fn validate_snapshot_lifecycle(
    manifest: &PortableSnapshotManifest,
    path: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let valid_retention = manifest
        .retain_until
        .as_deref()
        .is_some_and(|date| parse_iso_date(date).is_some());
    let valid = match manifest.status {
        PortableSnapshotStatus::Active => {
            manifest.replaced_by.is_none()
                && manifest
                    .retain_until
                    .as_deref()
                    .is_none_or(|date| parse_iso_date(date).is_some())
        }
        PortableSnapshotStatus::Replaced => {
            manifest.replaced_by.as_deref().is_some_and(|replacement| {
                valid_claim_identity(replacement) && replacement != manifest.id
            }) && valid_retention
        }
        PortableSnapshotStatus::Revoked => manifest.replaced_by.is_none() && valid_retention,
    };
    if !valid {
        push_snapshot_diagnostic(
            "DH_SNAPSHOT_007",
            path.to_owned(),
            format!(
                "Portable snapshot '{}' has inconsistent status, replacedBy, or retainUntil policy.",
                manifest.id
            ),
            diagnostics,
        );
    }
}

fn validate_snapshot_entry(
    root: &Path,
    manifest_rel: &Path,
    manifest: &PortableSnapshotManifest,
    entry: &PortableSnapshotEntry,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let manifest_path = manifest_rel.display().to_string();
    let locator_valid = match entry.scope {
        ContentAnchorScope::File => entry.locator.is_none(),
        ContentAnchorScope::Block => entry.locator.as_deref().is_some_and(valid_heading_slug),
        ContentAnchorScope::Commit => false,
    };
    if !safe_snapshot_path(&entry.path)
        || !safe_snapshot_path(&entry.payload)
        || !locator_valid
    {
        push_snapshot_diagnostic(
            "DH_SNAPSHOT_004",
            manifest_path.clone(),
            format!(
                "Portable snapshot '{}' entry '{}' has an unsafe path, payload, or scope/locator combination.",
                manifest.id, entry.target
            ),
            diagnostics,
        );
        return;
    }
    let payload = manifest_rel
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .join(Path::new(&entry.payload));
    let bytes = match entry.scope {
        ContentAnchorScope::File => std::fs::read(root.join(&payload)),
        ContentAnchorScope::Block => std::fs::read_to_string(root.join(&payload)).and_then(|text| {
            markdown_heading_block(&text, entry.locator.as_deref().unwrap_or_default())
                .map(ToOwned::to_owned)
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "snapshot block locator does not resolve exactly once",
                    )
                })
        }),
        ContentAnchorScope::Commit => unreachable!("commit scope rejected above"),
    };
    let actual = match bytes {
        Ok(bytes) => format!("{:x}", Sha256::digest(bytes)),
        Err(error) => {
            push_snapshot_diagnostic(
                "DH_SNAPSHOT_004",
                manifest_path,
                format!(
                    "Portable snapshot '{}' payload '{}' cannot be read: {error}.",
                    manifest.id,
                    payload.display()
                ),
                diagnostics,
            );
            return;
        }
    };
    if !valid_sha256(&entry.digest) || actual != entry.digest.to_ascii_lowercase() {
        push_snapshot_diagnostic(
            "DH_SNAPSHOT_005",
            manifest_rel.display().to_string(),
            format!(
                "Portable snapshot '{}' entry '{}' digest mismatch: declared '{}', actual '{}'.",
                manifest.id, entry.target, entry.digest, actual
            ),
            diagnostics,
        );
    }
}

fn validate_snapshot_replacements(
    snapshots: &BTreeMap<String, LoadedSnapshot>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for snapshot in snapshots.values() {
        if snapshot.manifest.status != PortableSnapshotStatus::Replaced {
            continue;
        }
        let replacement = snapshot.manifest.replaced_by.as_deref().and_then(|id| snapshots.get(id));
        if replacement.is_none_or(|replacement| {
            replacement.manifest.status != PortableSnapshotStatus::Active
        }) {
            push_snapshot_diagnostic(
                "DH_SNAPSHOT_007",
                snapshot.path.clone(),
                format!(
                    "Portable snapshot '{}' replacement must resolve to an active registered snapshot.",
                    snapshot.manifest.id
                ),
                diagnostics,
            );
        }
    }
}

fn validate_snapshot_anchor(
    edge: &GovernanceEdge,
    anchor: &ContentAnchor,
    provenance: &SnapshotProvenance,
    snapshots: &BTreeMap<String, LoadedSnapshot>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(snapshot) = snapshots.get(&provenance.manifest) else {
        push_snapshot_edge_diagnostic(
            "DH_SNAPSHOT_001",
            edge,
            None,
            format!(
                "Portable snapshot '{}' is not registered.",
                provenance.manifest
            ),
            diagnostics,
        );
        return;
    };
    if snapshot.manifest.status != PortableSnapshotStatus::Active {
        push_snapshot_edge_diagnostic(
            "DH_SNAPSHOT_007",
            edge,
            Some(snapshot),
            format!(
                "Portable snapshot '{}' is {:?} and cannot prove an active anchor.",
                snapshot.manifest.id, snapshot.manifest.status
            ),
            diagnostics,
        );
    }
    if provenance.repository != snapshot.manifest.repository {
        push_snapshot_edge_diagnostic(
            "DH_SNAPSHOT_002",
            edge,
            Some(snapshot),
            "Portable snapshot anchor repository does not match its manifest.".to_owned(),
            diagnostics,
        );
    }
    if provenance.commit != snapshot.manifest.commit {
        push_snapshot_edge_diagnostic(
            "DH_SNAPSHOT_003",
            edge,
            Some(snapshot),
            "Portable snapshot anchor commit does not match its manifest.".to_owned(),
            diagnostics,
        );
    }
    let entry = snapshot.manifest.entries.iter().find(|entry| {
        entry.target == edge.target
            && entry.scope == anchor.scope
            && entry.locator == anchor.locator
    });
    let Some(entry) = entry else {
        push_snapshot_edge_diagnostic(
            "DH_SNAPSHOT_005",
            edge,
            Some(snapshot),
            "Portable snapshot manifest has no matching target/scope/locator entry.".to_owned(),
            diagnostics,
        );
        return;
    };
    if provenance.path != entry.path {
        push_snapshot_edge_diagnostic(
            "DH_SNAPSHOT_004",
            edge,
            Some(snapshot),
            "Portable snapshot anchor path does not match its manifest entry.".to_owned(),
            diagnostics,
        );
    }
    if anchor.digest != entry.digest.to_ascii_lowercase() {
        push_snapshot_edge_diagnostic(
            "DH_SNAPSHOT_005",
            edge,
            Some(snapshot),
            "Portable snapshot anchor digest does not match its manifest entry.".to_owned(),
            diagnostics,
        );
    }
}

fn push_snapshot_edge_diagnostic(
    code: &'static str,
    edge: &GovernanceEdge,
    snapshot: Option<&LoadedSnapshot>,
    message: String,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut diagnostic = Diagnostic::new(
        code,
        Severity::Error,
        edge.source_location.path.clone(),
        message,
    );
    if let Some(line) = edge.source_location.line {
        diagnostic = diagnostic.at_line(line);
    }
    if let Some(snapshot) = snapshot {
        diagnostic = diagnostic.with_related(RelatedInformation::new(
            snapshot.path.clone(),
            "Portable snapshot manifest is here.",
        ));
    }
    diagnostics.push(diagnostic);
}

fn push_snapshot_diagnostic(
    code: &'static str,
    path: String,
    message: String,
    diagnostics: &mut Vec<Diagnostic>,
) {
    diagnostics.push(Diagnostic::new(code, Severity::Error, path, message));
}

fn valid_repository_identity(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && !value.contains("://")
        && !value.contains(['@', '?', '#', '\\'])
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || ".:_/-".contains(character))
}

fn valid_commit_oid(value: &str) -> bool {
    matches!(value.len(), 40 | 64) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn valid_heading_slug(value: &str) -> bool {
    Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
        .expect("static snapshot locator regex")
        .is_match(value)
}

fn safe_snapshot_path(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains('\\')
        && value
            .split('/')
            .all(|segment| !matches!(segment, "" | "." | ".."))
}
