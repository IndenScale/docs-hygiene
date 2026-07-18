#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GovernanceAsset {
    id: String,
    reference_relation: ReferenceRelation,
    status: String,
    #[serde(default)]
    superseded_by: Option<String>,
    #[serde(default)]
    members: Option<serde_yaml::Value>,
    #[serde(skip)]
    path: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageMember {
    id: String,
    status: String,
    #[serde(default)]
    superseded_by: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageDomain {
    id: String,
    status: String,
    #[serde(default)]
    superseded_by: Option<String>,
    kind: String,
    members: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageManifestNode {
    id: String,
    status: String,
    #[serde(default)]
    superseded_by: Option<String>,
    #[serde(default)]
    kind: Option<String>,
    members: Vec<String>,
}

#[derive(Clone, Debug)]
struct CanonicalPackageNode {
    identity: PackageMember,
    kind: Option<String>,
    members: Option<Vec<String>>,
}

fn is_governance_lifecycle_status(status: &str) -> bool {
    LifecycleStatus::parse(status).is_some()
}

fn check_governance(
    root: &Path,
    config: &Config,
    diagnostics: &mut Vec<Diagnostic>,
) -> (GovernanceGraph, OwnershipReport) {
    if config.governance.manifests.is_empty() {
        let mut graph = GovernanceGraph::default();
        graph.compare_community_baseline(&config.governance.topology.community_baseline);
        let ownership = check_document_ownership(root, config, &[], diagnostics);
        check_portable_snapshots(root, config, &graph, diagnostics);
        return (graph, ownership);
    }

    let mut assets = Vec::new();
    for rel in &config.governance.manifests {
        let path = root.join(rel);
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) => {
                diagnostics.push(Diagnostic::new(
                    "DH_GOVERNANCE_001",
                    Severity::Error,
                    rel.display().to_string(),
                    format!("Governance manifest cannot be read: {error}."),
                ));
                continue;
            }
        };
        let yaml = if matches!(
            path.extension().and_then(|value| value.to_str()),
            Some("md")
        ) {
            match markdown_frontmatter(&text) {
                Some(frontmatter) => frontmatter,
                None => {
                    diagnostics.push(Diagnostic::new(
                        "DH_GOVERNANCE_001",
                        Severity::Error,
                        rel.display().to_string(),
                        "Governance Markdown manifest requires YAML frontmatter.",
                    ));
                    continue;
                }
            }
        } else {
            text.as_str()
        };
        if yaml_declares_document_version(yaml) {
            diagnostics.push(Diagnostic::new(
                "DH_GOVERNANCE_001",
                Severity::Error,
                rel.display().to_string(),
                "Document-level version fields are not supported; use stable IDs and optional Wiki Link content hashes.",
            ));
            continue;
        }
        if yaml_declares_field(yaml, "references") {
            diagnostics.push(Diagnostic::new(
                "DH_GOVERNANCE_001",
                Severity::Error,
                rel.display().to_string(),
                "Manifest-level 'references' metadata is not supported; place semantic Wiki Links in governed content.",
            ));
            continue;
        }
        if [
            "refinementLevel",
            "formalizes",
            "realizes",
            "projects",
        ]
        .into_iter()
        .any(|field| yaml_declares_field(yaml, field))
        {
            diagnostics.push(Diagnostic::new(
                "DH_GOVERNANCE_001",
                Severity::Error,
                rel.display().to_string(),
                "Legacy refinement metadata is not supported; use UL/PRD semantic references and Issue evidence.",
            ));
            continue;
        }
        match serde_yaml::from_str::<GovernanceAsset>(yaml) {
            Ok(mut asset) => {
                asset.path = rel.display().to_string();
                if !is_governance_lifecycle_status(&asset.status) {
                    diagnostics.push(Diagnostic::new(
                        "DH_GOVERNANCE_001",
                        Severity::Error,
                        asset.path.clone(),
                        format!(
                            "Governed asset '{}' has invalid lifecycle status '{}'.",
                            asset.id, asset.status
                        ),
                    ));
                }
                assets.push(asset);
            }
            Err(error) => diagnostics.push(Diagnostic::new(
                "DH_GOVERNANCE_001",
                Severity::Error,
                rel.display().to_string(),
                format!("Invalid governance manifest: {error}."),
            )),
        }
    }

    let mut index = BTreeMap::new();
    for (position, asset) in assets.iter().enumerate() {
        let key = asset.id.as_str();
        if let Some(existing) = index.insert(key, position) {
            diagnostics.push(
                Diagnostic::new(
                    "DH_GOVERNANCE_001",
                    Severity::Error,
                    asset.path.clone(),
                    format!("Duplicate governed asset '{}'.", asset.id),
                )
                .with_related(RelatedInformation::new(
                    assets[existing].path.clone(),
                    "First declaration is here.",
                )),
            );
        }
    }

    let mut nodes = assets
        .iter()
        .map(|asset| GovernanceNode {
            identity: asset.id.clone(),
            reference_relation: asset.reference_relation,
            document_kind: None,
            lifecycle_status: asset.status.clone(),
            location: GovernanceLocation {
                path: asset.path.clone(),
                line: None,
            },
        })
        .collect::<Vec<_>>();
    let wiki = check_governed_references(root, config, &assets, diagnostics);
    nodes.extend(wiki.nodes);
    let mut graph = GovernanceGraph::new(nodes, wiki.edges);
    graph.compare_community_baseline(&config.governance.topology.community_baseline);

    for asset in &assets {
        check_package_members(root, config, asset, diagnostics);
    }
    let identities = collect_governed_identity_records(root, &assets);
    graph.authority_migrations = check_identity_lifecycle(&identities, &graph, diagnostics);
    let ownership = check_document_ownership(root, config, &identities, diagnostics);
    check_critical_dependencies(root, config, &graph, diagnostics);
    check_portable_snapshots(root, config, &graph, diagnostics);
    (graph, ownership)
}
