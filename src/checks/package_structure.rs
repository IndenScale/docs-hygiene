fn check_package_members(
    root: &Path,
    config: &Config,
    asset: &GovernanceAsset,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let manifest_rel = Path::new(&asset.path);
    let is_directory_package = manifest_rel.file_name().and_then(|value| value.to_str())
        == Some("manifest.yml")
        && asset.refinement_level != RefinementLevel::Implementation;
    if asset.refinement_level == RefinementLevel::Implementation
        && asset.reference_relation == ReferenceRelation::Body
    {
        check_implementation_body_members(root, asset, diagnostics);
        return;
    }
    if asset.reference_relation != ReferenceRelation::Library && !is_directory_package {
        return;
    }
    let Some(serde_yaml::Value::Sequence(members)) = &asset.members else {
        let code = package_diagnostic_code(asset.reference_relation);
        diagnostics.push(Diagnostic::new(
            code,
            Severity::Error,
            asset.path.clone(),
            format!(
                "{} '{}' must declare a member list.",
                asset.reference_relation.label(),
                asset.id
            ),
        ));
        return;
    };
    if members.is_empty() {
        let code = package_diagnostic_code(asset.reference_relation);
        diagnostics.push(Diagnostic::new(
            code,
            Severity::Error,
            asset.path.clone(),
            format!(
                "{} '{}' cannot have an empty member list.",
                asset.reference_relation.label(),
                asset.id
            ),
        ));
        return;
    }

    if !is_directory_package {
        check_non_directory_library_members(root, asset, members, diagnostics);
        return;
    }

    if asset.reference_relation == ReferenceRelation::Library {
        check_domain_fanout(
            &asset.id,
            &asset.path,
            members.len(),
            &config.governance.domain_fanout,
            diagnostics,
        );
    }

    let package_rel = manifest_rel.parent().unwrap_or_else(|| Path::new(""));
    let Some(members) = member_strings(members) else {
        diagnostics.push(Diagnostic::new(
            package_diagnostic_code(asset.reference_relation),
            Severity::Error,
            asset.path.clone(),
            "Package members must be path strings relative to their directory manifest.",
        ));
        return;
    };
    let mut identities = BTreeSet::new();
    let mut canonical_nodes = BTreeMap::new();
    canonical_nodes.insert(
        PathBuf::from("manifest.yml"),
        CanonicalPackageNode {
            identity: PackageMember {
                id: asset.id.clone(),
                status: asset.status.clone(),
            },
            kind: None,
            members: Some(members.clone()),
        },
    );
    identities.insert(asset.id.clone());
    check_package_directory(
        root,
        package_rel,
        Path::new(""),
        &members,
        asset.reference_relation,
        &mut identities,
        &mut canonical_nodes,
        &config.governance.domain_fanout,
        diagnostics,
    );
    check_localized_package(
        root,
        config,
        package_rel,
        asset.reference_relation,
        &canonical_nodes,
        diagnostics,
    );
}

fn check_implementation_body_members(
    root: &Path,
    asset: &GovernanceAsset,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(members) = &asset.members else {
        return;
    };
    let serde_yaml::Value::Mapping(groups) = members else {
        diagnostics.push(Diagnostic::new(
            "DH_BODY_001",
            Severity::Error,
            asset.path.clone(),
            "Implementation Body members must map member kinds to non-empty path lists.",
        ));
        return;
    };
    if groups.is_empty() {
        diagnostics.push(Diagnostic::new(
            "DH_BODY_001",
            Severity::Error,
            asset.path.clone(),
            "Implementation Body members cannot be empty when declared.",
        ));
        return;
    }

    let mut declared_paths = BTreeSet::new();
    for (kind, values) in groups {
        let Some(kind) = kind.as_str() else {
            diagnostics.push(Diagnostic::new(
                "DH_BODY_001",
                Severity::Error,
                asset.path.clone(),
                "Implementation Body member kinds must be strings.",
            ));
            continue;
        };
        let Some(values) = values.as_sequence() else {
            diagnostics.push(Diagnostic::new(
                "DH_BODY_001",
                Severity::Error,
                asset.path.clone(),
                format!("Implementation Body member kind '{kind}' must contain a path list."),
            ));
            continue;
        };
        if values.is_empty() {
            diagnostics.push(Diagnostic::new(
                "DH_BODY_001",
                Severity::Error,
                asset.path.clone(),
                format!("Implementation Body member kind '{kind}' cannot be empty."),
            ));
        }
        for value in values {
            let Some(member) = value.as_str() else {
                diagnostics.push(Diagnostic::new(
                    "DH_BODY_001",
                    Severity::Error,
                    asset.path.clone(),
                    format!("Implementation Body member kind '{kind}' must contain path strings."),
                ));
                continue;
            };
            let member_path = Path::new(member);
            let is_safe = !member_path.as_os_str().is_empty()
                && !member_path.is_absolute()
                && member_path
                    .components()
                    .all(|component| matches!(component, Component::Normal(_)));
            if !is_safe {
                diagnostics.push(Diagnostic::new(
                    "DH_BODY_001",
                    Severity::Error,
                    asset.path.clone(),
                    format!(
                        "Implementation Body member '{member}' must be a project-root-relative path without traversal."
                    ),
                ));
                continue;
            }
            if !declared_paths.insert(member.to_owned()) {
                diagnostics.push(Diagnostic::new(
                    "DH_BODY_001",
                    Severity::Error,
                    asset.path.clone(),
                    format!("Implementation Body member '{member}' is declared more than once."),
                ));
            }
            if !root.join(member_path).is_file() {
                diagnostics.push(Diagnostic::new(
                    "DH_BODY_001",
                    Severity::Error,
                    asset.path.clone(),
                    format!("Implementation Body member '{member}' does not exist."),
                ));
            }
        }
    }
}

fn package_diagnostic_code(reference_relation: ReferenceRelation) -> &'static str {
    match reference_relation {
        ReferenceRelation::Library => "DH_LIBRARY_001",
        ReferenceRelation::Body => "DH_BODY_001",
    }
}

fn member_strings(members: &[serde_yaml::Value]) -> Option<Vec<String>> {
    members
        .iter()
        .map(|member| member.as_str().map(str::to_owned))
        .collect()
}

fn check_non_directory_library_members(
    root: &Path,
    asset: &GovernanceAsset,
    members: &[serde_yaml::Value],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let manifest_rel = Path::new(&asset.path);
    let package_rel = manifest_rel.parent().unwrap_or_else(|| Path::new(""));
    for member in members {
        let Some(member) = member.as_str() else {
            diagnostics.push(Diagnostic::new(
                "DH_LIBRARY_001",
                Severity::Error,
                asset.path.clone(),
                "Library members must be path strings relative to the Library manifest.",
            ));
            continue;
        };
        if !root.join(package_rel).join(member).is_file() {
            diagnostics.push(Diagnostic::new(
                "DH_LIBRARY_001",
                Severity::Error,
                asset.path.clone(),
                format!("Library member '{}' does not exist.", member),
            ));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn check_package_directory(
    root: &Path,
    package_rel: &Path,
    directory_rel: &Path,
    members: &[String],
    reference_relation: ReferenceRelation,
    identities: &mut BTreeSet<String>,
    canonical_nodes: &mut BTreeMap<PathBuf, CanonicalPackageNode>,
    domain_fanout: &crate::config::DomainFanoutConfig,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let code = package_diagnostic_code(reference_relation);
    let directory = root.join(package_rel).join(directory_rel);
    let mut declared = BTreeSet::new();
    for member in members {
        let member_rel = Path::new(member);
        if member_rel.is_absolute()
            || member_rel.components().count() != 1
            || member == "manifest.yml"
        {
            diagnostics.push(Diagnostic::new(
                code,
                Severity::Error,
                package_rel
                    .join(directory_rel)
                    .join("manifest.yml")
                    .display()
                    .to_string(),
                format!("Package member '{member}' must name one direct child without traversal."),
            ));
            continue;
        }
        declared.insert(member_rel.to_path_buf());
        let node_rel = directory_rel.join(member_rel);
        let node_path = directory.join(member_rel);
        if node_path.is_dir() {
            let domain_manifest_path = node_path.join("manifest.yml");
            let domain_manifest_rel = node_rel.join("manifest.yml");
            let domain = std::fs::read_to_string(&domain_manifest_path)
                .ok()
                .filter(|text| !yaml_declares_removed_member_metadata(text))
                .and_then(|text| serde_yaml::from_str::<PackageDomain>(&text).ok());
            let Some(domain) = domain else {
                diagnostics.push(Diagnostic::new(
                    code,
                    Severity::Error,
                    package_rel.join(&domain_manifest_rel).display().to_string(),
                    "Declared package domain requires a valid manifest.yml.",
                ));
                continue;
            };
            validate_package_identity(
                &PackageMember {
                    id: domain.id.clone(),
                    status: domain.status.clone(),
                },
                &package_rel.join(&domain_manifest_rel),
                code,
                identities,
                diagnostics,
            );
            if domain.kind != "domain" {
                diagnostics.push(Diagnostic::new(
                    code,
                    Severity::Error,
                    package_rel.join(&domain_manifest_rel).display().to_string(),
                    "Nested package manifest must declare kind: domain.",
                ));
            }
            if reference_relation == ReferenceRelation::Library {
                check_domain_fanout(
                    &domain.id,
                    &package_rel.join(&domain_manifest_rel).display().to_string(),
                    domain.members.len(),
                    domain_fanout,
                    diagnostics,
                );
            }
            canonical_nodes.insert(
                domain_manifest_rel,
                CanonicalPackageNode {
                    identity: PackageMember {
                        id: domain.id,
                        status: domain.status,
                    },
                    kind: Some(domain.kind),
                    members: Some(domain.members.clone()),
                },
            );
            check_package_directory(
                root,
                package_rel,
                &node_rel,
                &domain.members,
                reference_relation,
                identities,
                canonical_nodes,
                domain_fanout,
                diagnostics,
            );
            continue;
        }
        if !node_path.is_file() {
            diagnostics.push(Diagnostic::new(
                code,
                Severity::Error,
                package_rel.join(&node_rel).display().to_string(),
                "Declared package member does not exist.",
            ));
            continue;
        }
        if member_rel.extension().and_then(|value| value.to_str()) != Some("md") {
            diagnostics.push(Diagnostic::new(
                code,
                Severity::Error,
                package_rel.join(&node_rel).display().to_string(),
                "Package leaf member must be a Markdown file.",
            ));
            continue;
        }
        let text = match std::fs::read_to_string(&node_path) {
            Ok(text) => text,
            Err(error) => {
                diagnostics.push(Diagnostic::new(
                    code,
                    Severity::Error,
                    package_rel.join(&node_rel).display().to_string(),
                    format!("Package member cannot be read: {error}."),
                ));
                continue;
            }
        };
        let Some(frontmatter) = markdown_frontmatter(&text) else {
            diagnostics.push(Diagnostic::new(
                code,
                Severity::Error,
                package_rel.join(&node_rel).display().to_string(),
                "Package Markdown member requires YAML frontmatter.",
            ));
            continue;
        };
        if yaml_declares_document_version(frontmatter) {
            diagnostics.push(Diagnostic::new(
                code,
                Severity::Error,
                package_rel.join(&node_rel).display().to_string(),
                "Document-level version fields are not supported; use stable IDs and optional Wiki Link content hashes.",
            ));
            continue;
        }
        if yaml_declares_field(frontmatter, "source") {
            diagnostics.push(Diagnostic::new(
                code,
                Severity::Error,
                package_rel.join(&node_rel).display().to_string(),
                "Frontmatter 'source' metadata is not supported; place semantic Wiki Links in governed content.",
            ));
            continue;
        }
        match serde_yaml::from_str::<PackageMember>(frontmatter) {
            Ok(item) => {
                validate_package_identity(
                    &item,
                    &package_rel.join(&node_rel),
                    code,
                    identities,
                    diagnostics,
                );
                canonical_nodes.insert(
                    node_rel,
                    CanonicalPackageNode {
                        identity: item,
                        kind: None,
                        members: None,
                    },
                );
            }
            Err(error) => diagnostics.push(Diagnostic::new(
                code,
                Severity::Error,
                package_rel.join(&node_rel).display().to_string(),
                format!("Invalid package member frontmatter: {error}."),
            )),
        }
    }

    let discovered = std::fs::read_dir(&directory)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name();
            if name == "manifest.yml" {
                return None;
            }
            let path = entry.path();
            if path.is_dir() || path.extension().and_then(|value| value.to_str()) == Some("md") {
                Some(PathBuf::from(name))
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();
    for orphan in discovered.difference(&declared) {
        diagnostics.push(Diagnostic::new(
            code,
            Severity::Error,
            package_rel
                .join(directory_rel)
                .join(orphan)
                .display()
                .to_string(),
            "Package child is not declared in its directory manifest.",
        ));
    }
}
