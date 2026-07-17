#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GovernanceTarget {
    id: String,
}

#[derive(Clone, Debug, Default)]
struct GovernanceTargets(Vec<GovernanceTarget>);

impl GovernanceTargets {
    fn iter(&self) -> impl Iterator<Item = &GovernanceTarget> {
        self.0.iter()
    }

}

impl<'de> Deserialize<'de> for GovernanceTargets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum OneOrMany {
            OneId(String),
            One(GovernanceTarget),
            ManyIds(Vec<String>),
            Many(Vec<GovernanceTarget>),
        }

        Ok(match OneOrMany::deserialize(deserializer)? {
            OneOrMany::OneId(id) => Self(vec![GovernanceTarget { id }]),
            OneOrMany::One(target) => Self(vec![target]),
            OneOrMany::ManyIds(ids) => {
                Self(ids.into_iter().map(|id| GovernanceTarget { id }).collect())
            }
            OneOrMany::Many(targets) => Self(targets),
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GovernanceAsset {
    id: String,
    refinement_level: RefinementLevel,
    reference_relation: ReferenceRelation,
    status: String,
    #[serde(default)]
    superseded_by: Option<String>,
    #[serde(default)]
    formalizes: GovernanceTargets,
    #[serde(default)]
    realizes: GovernanceTargets,
    #[serde(default)]
    projects: GovernanceTargets,
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
    matches!(
        status,
        "draft"
            | "review"
            | "proposed"
            | "baselined"
            | "current"
            | "superseded"
            | "archived"
            | "abandoned"
    )
}

fn check_governance(
    root: &Path,
    config: &Config,
    diagnostics: &mut Vec<Diagnostic>,
) -> GovernanceGraph {
    if config.governance.manifests.is_empty() {
        let graph = GovernanceGraph::default();
        check_portable_snapshots(root, config, &graph, diagnostics);
        return graph;
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
            refinement_level: asset.refinement_level,
            reference_relation: asset.reference_relation,
            lifecycle_status: asset.status.clone(),
            location: GovernanceLocation {
                path: asset.path.clone(),
                line: None,
            },
        })
        .collect::<Vec<_>>();
    let mut edges = collect_vertical_edges(&assets, &index);
    let wiki = check_governed_references(root, config, &assets, diagnostics);
    nodes.extend(wiki.nodes);
    edges.extend(wiki.edges);
    let mut graph = GovernanceGraph::new(nodes, edges);

    for asset in &assets {
        check_package_members(root, config, asset, diagnostics);
        check_vertical_derivation(asset, &graph, diagnostics);
    }
    if config.governance.require_complete_vertical_derivation {
        check_vertical_derivation_completeness(&assets, &graph, diagnostics);
    }
    let identities = collect_governed_identity_records(root, &assets);
    graph.authority_migrations = check_identity_lifecycle(&identities, &graph, diagnostics);
    check_critical_dependencies(root, config, &graph, diagnostics);
    check_portable_snapshots(root, config, &graph, diagnostics);
    graph
}

fn collect_vertical_edges(
    assets: &[GovernanceAsset],
    index: &BTreeMap<&str, usize>,
) -> Vec<GovernanceEdge> {
    let mut edges = Vec::new();
    for asset in assets {
        for (relation, targets) in [
            (GovernanceEdgeKind::Formalizes, &asset.formalizes),
            (GovernanceEdgeKind::Realizes, &asset.realizes),
            (GovernanceEdgeKind::Projects, &asset.projects),
        ] {
            edges.extend(targets.iter().map(|target| GovernanceEdge {
                source: asset.id.clone(),
                target: target.id.clone(),
                relation,
                source_location: GovernanceLocation {
                    path: asset.path.clone(),
                    line: None,
                },
                selector: None,
                content_anchor: None,
                lifecycle: LifecycleProvenance {
                    source_status: asset.status.clone(),
                    target_status: index
                        .get(target.id.as_str())
                        .map(|position| assets[*position].status.clone()),
                },
            }));
        }
    }
    edges
}
