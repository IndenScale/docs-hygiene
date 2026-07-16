#[derive(Clone)]
struct SemanticTarget {
    refinement_level: RefinementLevel,
    path: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WikiReference {
    id: String,
    content_hash: Option<String>,
}

fn check_wiki_references(
    root: &Path,
    config: &Config,
    assets: &[GovernanceAsset],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let targets = build_library_target_index(root, assets, diagnostics);
    for asset in assets
        .iter()
        .filter(|asset| asset.reference_relation == ReferenceRelation::Body)
    {
        let canonical_paths = asset_content_paths(root, asset);
        let canonical = collect_wiki_references(root, &canonical_paths, diagnostics);
        validate_asset_wiki_references(root, asset, &canonical, &targets, diagnostics);

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
            let localized = collect_wiki_references(root, &localized_paths, diagnostics);
            if canonical != localized {
                diagnostics.push(Diagnostic::new(
                    "DH_REFERENCE_001",
                    Severity::Error,
                    localized_root.display().to_string(),
                    format!(
                        "Localized Body for '{language}' must preserve canonical Wiki Link targets and content-hash anchors."
                    ),
                ));
            }
        }
    }
}

fn build_library_target_index(
    root: &Path,
    assets: &[GovernanceAsset],
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, SemanticTarget> {
    let mut targets = BTreeMap::new();
    for asset in assets
        .iter()
        .filter(|asset| asset.reference_relation == ReferenceRelation::Library)
    {
        insert_library_target(
            &mut targets,
            asset.id.clone(),
            SemanticTarget {
                refinement_level: asset.refinement_level,
                path: asset.path.clone(),
            },
            diagnostics,
        );
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
                        path: package_rel.join(&node_rel).display().to_string(),
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
                    path: package_rel.join(&manifest_rel).display().to_string(),
                },
                diagnostics,
            );
            collect_declared_library_targets(
                root,
                package_rel,
                &node_rel,
                &domain.members,
                refinement_level,
                targets,
                diagnostics,
            );
        }
    }
}

fn insert_library_target(
    targets: &mut BTreeMap<String, SemanticTarget>,
    id: String,
    target: SemanticTarget,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(existing) = targets.insert(id.clone(), target.clone()) {
        diagnostics.push(
            Diagnostic::new(
                "DH_GOVERNANCE_001",
                Severity::Error,
                target.path,
                format!("Library semantic identity '{id}' is declared more than once."),
            )
            .with_related(RelatedInformation::new(
                existing.path,
                "First declaration is here.",
            )),
        );
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

fn collect_wiki_references(
    root: &Path,
    paths: &[PathBuf],
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeSet<WikiReference> {
    let wiki_link = Regex::new(r"\[\[([^\]\n]+)\]\]").expect("static Wiki Link regex");
    let target = Regex::new(r"^([A-Za-z0-9._-]+)(?:@sha256:([A-Fa-f0-9]{64}))?(?:\|[^\]\n]+)?$")
        .expect("static Wiki Link target regex");
    let mut references = BTreeSet::new();
    for rel in paths {
        let text = match std::fs::read_to_string(root.join(rel)) {
            Ok(text) => text,
            Err(error) => {
                diagnostics.push(Diagnostic::new(
                    "DH_REFERENCE_001",
                    Severity::Error,
                    rel.display().to_string(),
                    format!("Body content cannot be read for Wiki Link analysis: {error}."),
                ));
                continue;
            }
        };
        for wiki_capture in wiki_link.captures_iter(&strip_markdown_code(&text)) {
            let Some(captures) = target.captures(&wiki_capture[1]) else {
                diagnostics.push(Diagnostic::new(
                    "DH_REFERENCE_001",
                    Severity::Error,
                    rel.display().to_string(),
                    format!(
                        "Invalid semantic Wiki Link '[[{}]]'; expected [[ID]], [[ID|label]], or [[ID@sha256:<64-hex>|label]].",
                        &wiki_capture[1]
                    ),
                ));
                continue;
            };
            references.insert(WikiReference {
                id: captures[1].to_owned(),
                content_hash: captures
                    .get(2)
                    .map(|value| value.as_str().to_ascii_lowercase()),
            });
        }
    }
    references
}

fn validate_asset_wiki_references(
    root: &Path,
    asset: &GovernanceAsset,
    references: &BTreeSet<WikiReference>,
    targets: &BTreeMap<String, SemanticTarget>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut satisfies_required_relation = false;
    for reference in references {
        let Some(target) = targets.get(&reference.id) else {
            diagnostics.push(Diagnostic::new(
                "DH_REFERENCE_001",
                Severity::Error,
                asset.path.clone(),
                format!(
                    "Wiki Link target '{}' is not a governed Library identity.",
                    reference.id
                ),
            ));
            continue;
        };
        let same_refinement = target.refinement_level == asset.refinement_level;
        let adjacent_upstream =
            refinement_rank(target.refinement_level) + 1 == refinement_rank(asset.refinement_level);
        let valid_relation = match asset.reference_relation {
            ReferenceRelation::Body => same_refinement,
            ReferenceRelation::Library => same_refinement || adjacent_upstream,
        };
        if !valid_relation {
            diagnostics.push(
                Diagnostic::new(
                    "DH_REFERENCE_001",
                    Severity::Error,
                    asset.path.clone(),
                    format!(
                        "Asset '{}' cannot reference Library Wiki Link target '{}' from {} refinement level.",
                        asset.id,
                        reference.id,
                        target.refinement_level.label()
                    ),
                )
                .with_related(RelatedInformation::new(
                    target.path.clone(),
                    "Resolved target is declared here.",
                )),
            );
            continue;
        }
        satisfies_required_relation |= match asset.reference_relation {
            ReferenceRelation::Body => same_refinement,
            ReferenceRelation::Library => adjacent_upstream,
        };
        if let Some(expected_hash) = &reference.content_hash {
            let target_bytes = match std::fs::read(root.join(&target.path)) {
                Ok(bytes) => bytes,
                Err(error) => {
                    diagnostics.push(Diagnostic::new(
                        "DH_REFERENCE_001",
                        Severity::Error,
                        asset.path.clone(),
                        format!(
                            "Content-hash target '{}' cannot be read: {error}.",
                            reference.id
                        ),
                    ));
                    continue;
                }
            };
            let actual_hash = format!("{:x}", Sha256::digest(target_bytes));
            if &actual_hash != expected_hash {
                diagnostics.push(
                    Diagnostic::new(
                        "DH_REFERENCE_001",
                        Severity::Error,
                        asset.path.clone(),
                        format!(
                            "Wiki Link target '{}' changed: expected sha256:{expected_hash}, actual sha256:{actual_hash}.",
                            reference.id
                        ),
                    )
                    .with_related(RelatedInformation::new(
                        target.path.clone(),
                        "Changed Library content is here.",
                    )),
                );
            }
        }
    }
    let missing_required_reference = match asset.reference_relation {
        ReferenceRelation::Body => references.is_empty(),
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
