#[derive(Clone)]
struct SemanticTarget {
    refinement_level: RefinementLevel,
    reference_relation: ReferenceRelation,
    status: String,
    superseded_by: Option<String>,
    path: String,
    document_kind: Option<String>,
    alternates: Vec<SemanticTarget>,
}

struct ReferenceAnalysis {
    nodes: Vec<GovernanceNode>,
    edges: Vec<GovernanceEdge>,
}

fn check_governed_references(
    root: &Path,
    config: &Config,
    assets: &[GovernanceAsset],
    diagnostics: &mut Vec<Diagnostic>,
) -> ReferenceAnalysis {
    let targets = build_library_target_index(root, config, assets, diagnostics);
    check_core_library_claims(root, config, assets, &targets, diagnostics);
    let mut edges = Vec::new();
    for asset in assets
        .iter()
        .filter(|asset| asset.reference_relation == ReferenceRelation::Body)
    {
        let canonical_paths = asset_content_paths(root, asset);
        let canonical =
            collect_governed_reference_occurrences(root, &canonical_paths, diagnostics);
        let mut canonical_edges = normalize_reference_edges(asset, &canonical, &targets);
        validate_asset_wiki_references(
            root,
            config,
            asset,
            &mut canonical_edges,
            &targets,
            diagnostics,
        );
        edges.extend(canonical_edges);

        if asset.refinement_level == RefinementLevel::Implementation {
            continue;
        }
        let package_rel = Path::new(&asset.path)
            .parent()
            .unwrap_or_else(|| Path::new(""));
        for (language, localized_root) in localized_package_roots(config, package_rel) {
            let localized_paths = canonical_paths
                .iter()
                .filter_map(|canonical_path| {
                    canonical_path
                        .strip_prefix(package_rel)
                        .ok()
                        .map(|suffix| localized_root.join(suffix))
                })
                .collect::<Vec<_>>();
            let localized =
                collect_governed_reference_occurrences(root, &localized_paths, diagnostics);
            if semantic_reference_signatures(&canonical, REFERENCE_POLICIES)
                != semantic_reference_signatures(&localized, REFERENCE_POLICIES)
            {
                diagnostics.push(Diagnostic::new(
                    "DH_REFERENCE_001",
                    Severity::Error,
                    localized_root.display().to_string(),
                    format!(
                        "Localized Body for '{language}' must preserve canonical Wiki Link targets, selectors, and content-hash anchors."
                    ),
                ));
            }
        }
    }
    for asset in assets
        .iter()
        .filter(|asset| asset.reference_relation == ReferenceRelation::Library)
    {
        let paths = asset_content_paths(root, asset);
        let occurrences = collect_governed_reference_occurrences(root, &paths, diagnostics)
            .into_iter()
            .filter(|occurrence| occurrence.context == CONTEXT_GOVERNED_ANCHOR)
            .collect::<BTreeSet<_>>();
        let mut pinned = normalize_reference_edges(asset, &occurrences, &targets);
        validate_optional_governed_pins(root, config, asset, &mut pinned, &targets, diagnostics);
        edges.extend(pinned);
    }
    let nodes = targets
        .into_iter()
        .flat_map(|(identity, target)| {
            let mut candidates = vec![target.clone()];
            candidates.extend(target.alternates);
            candidates.into_iter().map(move |candidate| GovernanceNode {
                identity: identity.clone(),
                refinement_level: candidate.refinement_level,
                reference_relation: candidate.reference_relation,
                document_kind: candidate.document_kind,
                lifecycle_status: candidate.status,
                location: GovernanceLocation {
                    path: candidate.path,
                    line: None,
                },
            })
        })
        .collect();
    ReferenceAnalysis { nodes, edges }
}

fn build_library_target_index(
    root: &Path,
    config: &Config,
    assets: &[GovernanceAsset],
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, SemanticTarget> {
    let mut targets = BTreeMap::new();
    for asset in assets {
        insert_library_target(
            &mut targets,
            asset.id.clone(),
            SemanticTarget {
                refinement_level: asset.refinement_level,
                reference_relation: asset.reference_relation,
                status: asset.status.clone(),
                superseded_by: asset.superseded_by.clone(),
                path: asset.path.clone(),
                document_kind: inferred_document_kind(config, &asset.path),
                alternates: Vec::new(),
            },
            diagnostics,
        );
        if asset.reference_relation != ReferenceRelation::Library {
            continue;
        }
        let manifest_rel = Path::new(&asset.path);
        if manifest_rel.file_name().and_then(|value| value.to_str()) != Some("manifest.yml")
            || asset.refinement_level == RefinementLevel::Implementation
        {
            continue;
        }
        let package_rel = manifest_rel.parent().unwrap_or_else(|| Path::new(""));
        let Some(serde_yaml::Value::Sequence(members)) = &asset.members else {
            continue;
        };
        let Some(members) = member_strings(members) else {
            continue;
        };
        collect_declared_library_targets(
            root,
            package_rel,
            Path::new(""),
            &members,
            asset.refinement_level,
            config,
            &mut targets,
            diagnostics,
        );
    }
    targets
}

#[allow(clippy::too_many_arguments)]
fn collect_declared_library_targets(
    root: &Path,
    package_rel: &Path,
    directory_rel: &Path,
    members: &[String],
    refinement_level: RefinementLevel,
    config: &Config,
    targets: &mut BTreeMap<String, SemanticTarget>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for member in members {
        let member_rel = Path::new(member);
        if member_rel.is_absolute() || member_rel.components().count() != 1 {
            continue;
        }
        let node_rel = directory_rel.join(member_rel);
        let node_path = root.join(package_rel).join(&node_rel);
        if node_path.is_file()
            && node_path.extension().and_then(|value| value.to_str()) == Some("md")
        {
            let identity = std::fs::read_to_string(&node_path)
                .ok()
                .and_then(|text| markdown_frontmatter(&text).map(str::to_owned))
                .filter(|frontmatter| !yaml_declares_removed_member_metadata(frontmatter))
                .and_then(|frontmatter| serde_yaml::from_str::<PackageMember>(&frontmatter).ok());
            if let Some(identity) = identity {
                insert_library_target(
                    targets,
                    identity.id,
                    SemanticTarget {
                        refinement_level,
                        reference_relation: ReferenceRelation::Library,
                        status: identity.status,
                        superseded_by: identity.superseded_by,
                        path: package_rel.join(&node_rel).display().to_string(),
                        document_kind: inferred_document_kind(
                            config,
                            &package_rel.join(&node_rel).display().to_string(),
                        ),
                        alternates: Vec::new(),
                    },
                    diagnostics,
                );
            }
            continue;
        }
        if !node_path.is_dir() {
            continue;
        }
        let manifest_rel = node_rel.join("manifest.yml");
        let domain = std::fs::read_to_string(node_path.join("manifest.yml"))
            .ok()
            .filter(|text| !yaml_declares_removed_member_metadata(text))
            .and_then(|text| serde_yaml::from_str::<PackageDomain>(&text).ok());
        if let Some(domain) = domain {
            insert_library_target(
                targets,
                domain.id,
                SemanticTarget {
                    refinement_level,
                    reference_relation: ReferenceRelation::Library,
                    status: domain.status,
                    superseded_by: domain.superseded_by,
                    path: package_rel.join(&manifest_rel).display().to_string(),
                    document_kind: inferred_document_kind(
                        config,
                        &package_rel.join(&manifest_rel).display().to_string(),
                    ),
                    alternates: Vec::new(),
                },
                diagnostics,
            );
            collect_declared_library_targets(
                root,
                package_rel,
                &node_rel,
                &domain.members,
                refinement_level,
                config,
                targets,
                diagnostics,
            );
        }
    }
}

fn inferred_document_kind(config: &Config, path: &str) -> Option<String> {
    let path = Path::new(path);
    let filename = path.file_name()?.to_str()?;
    normalized_bases(config).into_iter().find_map(|base| {
        let belongs = std::iter::once(base.root.as_path())
            .chain(base.localized_roots.values().map(PathBuf::as_path))
            .any(|root| path.starts_with(root));
        if !belongs {
            return None;
        }
        base.patterns.into_iter().find_map(|pattern| {
            Regex::new(&pattern.regex)
                .ok()
                .filter(|regex| regex.is_match(filename))
                .map(|_| pattern.document_kind)
        })
    })
}

fn insert_library_target(
    targets: &mut BTreeMap<String, SemanticTarget>,
    id: String,
    target: SemanticTarget,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(existing) = targets.get_mut(&id) {
        diagnostics.push(
            Diagnostic::new(
                "DH_GOVERNANCE_001",
                Severity::Error,
                target.path.clone(),
                format!("Library semantic identity '{id}' is declared more than once."),
            )
            .with_related(RelatedInformation::new(
                existing.path.clone(),
                "First declaration is here.",
            )),
        );
        existing.alternates.push(target);
    } else {
        targets.insert(id, target);
    }
}

fn asset_content_paths(root: &Path, asset: &GovernanceAsset) -> Vec<PathBuf> {
    if asset.refinement_level == RefinementLevel::Implementation {
        let members = match &asset.members {
            Some(serde_yaml::Value::Mapping(groups)) => groups
                .values()
                .filter_map(serde_yaml::Value::as_sequence)
                .flatten()
                .collect::<Vec<_>>(),
            Some(serde_yaml::Value::Sequence(members)) => members.iter().collect(),
            _ => Vec::new(),
        };
        return members
            .into_iter()
            .filter_map(serde_yaml::Value::as_str)
            .map(PathBuf::from)
            .filter(|path| root.join(path).is_file())
            .collect();
    }
    let package_rel = Path::new(&asset.path)
        .parent()
        .unwrap_or_else(|| Path::new(""));
    let Some(serde_yaml::Value::Sequence(members)) = &asset.members else {
        return Vec::new();
    };
    let Some(members) = member_strings(members) else {
        return Vec::new();
    };
    let mut paths = Vec::new();
    collect_declared_body_content_paths(root, package_rel, Path::new(""), &members, &mut paths);
    paths
}

fn collect_declared_body_content_paths(
    root: &Path,
    package_rel: &Path,
    directory_rel: &Path,
    members: &[String],
    paths: &mut Vec<PathBuf>,
) {
    for member in members {
        let member_rel = Path::new(member);
        if member_rel.is_absolute() || member_rel.components().count() != 1 {
            continue;
        }
        let node_rel = directory_rel.join(member_rel);
        let node_path = root.join(package_rel).join(&node_rel);
        if node_path.is_file()
            && node_path.extension().and_then(|value| value.to_str()) == Some("md")
        {
            paths.push(package_rel.join(node_rel));
            continue;
        }
        if !node_path.is_dir() {
            continue;
        }
        let domain = std::fs::read_to_string(node_path.join("manifest.yml"))
            .ok()
            .filter(|text| !yaml_declares_removed_member_metadata(text))
            .and_then(|text| serde_yaml::from_str::<PackageDomain>(&text).ok());
        if let Some(domain) = domain {
            collect_declared_body_content_paths(
                root,
                package_rel,
                &node_rel,
                &domain.members,
                paths,
            );
        }
    }
}

fn validate_asset_wiki_references(
    root: &Path,
    config: &Config,
    asset: &GovernanceAsset,
    edges: &mut [GovernanceEdge],
    targets: &BTreeMap<String, SemanticTarget>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut satisfies_required_relation = false;
    let mut attempted_horizontal_reference = false;
    for edge in edges {
        let declared_vertical_pin = edge.relation == GovernanceEdgeKind::PinnedReference
            && asset_declares_vertical_target(asset, &edge.target);
        attempted_horizontal_reference |= !declared_vertical_pin;
        let Some(target) = targets.get(&edge.target) else {
            diagnostics.push(Diagnostic::new(
                "DH_REFERENCE_001",
                Severity::Error,
                asset.path.clone(),
                format!(
                    "Wiki Link target '{}' is not a governed Library identity.",
                    edge.target
                ),
            ));
            continue;
        };
        if edge.resolution.outcome == ReferenceResolutionOutcome::Ambiguous {
            diagnostics.push(Diagnostic::new(
                "DH_REFERENCE_001",
                Severity::Error,
                edge.source_location.path.clone(),
                format!(
                    "Reference target '{}' is ambiguous across {} governed endpoints.",
                    edge.target,
                    edge.resolution.endpoints.len()
                ),
            ));
            continue;
        }
        let same_refinement = target.refinement_level == asset.refinement_level;
        let adjacent_upstream =
            refinement_rank(target.refinement_level) + 1 == refinement_rank(asset.refinement_level);
        let type_issues = edge
            .resolution
            .incompatibilities
            .iter()
            .copied()
            .filter(|issue| {
                matches!(
                    issue,
                    ReferenceCompatibilityIssue::RefinementLevel
                        | ReferenceCompatibilityIssue::ReferenceRelation
                        | ReferenceCompatibilityIssue::DocumentKind
                )
            })
            .collect::<Vec<_>>();
        if !type_issues.is_empty() {
            diagnostics.push(
                Diagnostic::new(
                    "DH_REFERENCE_001",
                    Severity::Error,
                    asset.path.clone(),
                    format!(
                        "Asset '{}' has incompatible reference target '{}': {:?}.",
                        asset.id,
                        edge.target,
                        type_issues
                    ),
                )
                .with_related(RelatedInformation::new(
                    target.path.clone(),
                    "Resolved target is declared here.",
                )),
            );
            continue;
        }
        satisfies_required_relation |= target.reference_relation == ReferenceRelation::Library
            && match asset.reference_relation {
            ReferenceRelation::Body => same_refinement,
            ReferenceRelation::Library => adjacent_upstream,
        };
        if !validate_edge_selector(root, edge, target, diagnostics) {
            edge.resolution
                .add_incompatibility(ReferenceCompatibilityIssue::Selector);
        }
        if !validate_edge_anchor(root, config, edge, target, diagnostics) {
            edge.resolution
                .add_incompatibility(ReferenceCompatibilityIssue::Anchor);
        }
    }
    let missing_required_reference = match asset.reference_relation {
        ReferenceRelation::Body => {
            !satisfies_required_relation && !attempted_horizontal_reference
        }
        ReferenceRelation::Library => {
            asset.refinement_level != RefinementLevel::Intent && !satisfies_required_relation
        }
    };
    if missing_required_reference {
        let expected = match asset.reference_relation {
            ReferenceRelation::Body => format!(
                "a Library at the same {} refinement level",
                asset.refinement_level.label()
            ),
            ReferenceRelation::Library => "an adjacent upstream Library".to_owned(),
        };
        diagnostics.push(Diagnostic::new(
            "DH_REFERENCE_001",
            Severity::Error,
            asset.path.clone(),
            format!(
                "Asset '{}' must contain a Wiki Link to {expected}.",
                asset.id
            ),
        ));
    }
}

fn validate_optional_governed_pins(
    root: &Path,
    config: &Config,
    asset: &GovernanceAsset,
    edges: &mut [GovernanceEdge],
    targets: &BTreeMap<String, SemanticTarget>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for edge in edges {
        let Some(target) = targets.get(&edge.target) else {
            diagnostics.push(Diagnostic::new(
                "DH_REFERENCE_001",
                Severity::Error,
                edge.source_location.path.clone(),
                format!("Pinned target '{}' is not a governed identity.", edge.target),
            ));
            continue;
        };
        if edge.resolution.outcome == ReferenceResolutionOutcome::Ambiguous {
            diagnostics.push(Diagnostic::new(
                "DH_REFERENCE_001",
                Severity::Error,
                edge.source_location.path.clone(),
                format!(
                    "Pinned target '{}' is ambiguous across {} governed endpoints.",
                    edge.target,
                    edge.resolution.endpoints.len()
                ),
            ));
            continue;
        }
        let type_issues = edge
            .resolution
            .incompatibilities
            .iter()
            .copied()
            .filter(|issue| {
                matches!(
                    issue,
                    ReferenceCompatibilityIssue::RefinementLevel
                        | ReferenceCompatibilityIssue::ReferenceRelation
                        | ReferenceCompatibilityIssue::DocumentKind
                )
            })
            .collect::<Vec<_>>();
        if !type_issues.is_empty() {
            diagnostics.push(Diagnostic::new(
                "DH_REFERENCE_001",
                Severity::Error,
                edge.source_location.path.clone(),
                format!(
                    "Pinned target '{}' is incompatible with Library '{}': {:?}.",
                    edge.target, asset.id, type_issues
                ),
            ));
            continue;
        }
        if !validate_edge_selector(root, edge, target, diagnostics) {
            edge.resolution
                .add_incompatibility(ReferenceCompatibilityIssue::Selector);
        }
        if !validate_edge_anchor(root, config, edge, target, diagnostics) {
            edge.resolution
                .add_incompatibility(ReferenceCompatibilityIssue::Anchor);
        }
    }
}

fn asset_declares_vertical_target(asset: &GovernanceAsset, target: &str) -> bool {
    asset
        .formalizes
        .iter()
        .chain(asset.realizes.iter())
        .chain(asset.projects.iter())
        .any(|candidate| candidate.id == target)
}
