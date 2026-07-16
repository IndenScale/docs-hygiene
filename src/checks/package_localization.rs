fn validate_package_identity(
    item: &PackageMember,
    path: &Path,
    code: &'static str,
    identities: &mut BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !is_governance_lifecycle_status(&item.status) {
        diagnostics.push(Diagnostic::new(
            code,
            Severity::Error,
            path.display().to_string(),
            format!(
                "Package identity '{}' has invalid lifecycle status '{}'.",
                item.id, item.status
            ),
        ));
    }
    if !identities.insert(item.id.clone()) {
        diagnostics.push(Diagnostic::new(
            code,
            Severity::Error,
            path.display().to_string(),
            format!("Package identity '{}' is declared more than once.", item.id),
        ));
    }
}

fn check_localized_package(
    root: &Path,
    config: &Config,
    package_rel: &Path,
    reference_relation: ReferenceRelation,
    canonical_nodes: &BTreeMap<PathBuf, CanonicalPackageNode>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let code = package_diagnostic_code(reference_relation);
    let localized_roots = localized_package_roots(config, package_rel);
    if localized_roots.is_empty() {
        return;
    }
    for (language, localized_root_rel) in localized_roots {
        let canonical_entries = canonical_nodes
            .keys()
            .flat_map(|path| {
                let mut entries = vec![path.clone()];
                if path.file_name().and_then(|value| value.to_str()) == Some("manifest.yml") {
                    if let Some(parent) = path
                        .parent()
                        .filter(|parent| !parent.as_os_str().is_empty())
                    {
                        entries.push(parent.to_path_buf());
                    }
                }
                entries
            })
            .collect::<BTreeSet<_>>();
        for (node_rel, canonical) in canonical_nodes {
            let localized_path = root.join(&localized_root_rel).join(node_rel);
            if !localized_path.is_file() {
                diagnostics.push(Diagnostic::new(
                    code,
                    Severity::Error,
                    localized_root_rel.join(node_rel).display().to_string(),
                    format!("Localized package node for '{language}' is missing."),
                ));
                continue;
            }
            let path = localized_root_rel.join(node_rel).display().to_string();
            let text = std::fs::read_to_string(&localized_path).ok();
            let localized =
                if node_rel.file_name().and_then(|value| value.to_str()) == Some("manifest.yml") {
                    text.filter(|text| !yaml_declares_removed_member_metadata(text))
                        .and_then(|text| serde_yaml::from_str::<PackageManifestNode>(&text).ok())
                        .map(|node| CanonicalPackageNode {
                            identity: PackageMember {
                                id: node.id,
                                status: node.status,
                            },
                            kind: node.kind,
                            members: Some(node.members),
                        })
                } else {
                    text.and_then(|text| markdown_frontmatter(&text).map(str::to_owned))
                        .filter(|frontmatter| !yaml_declares_removed_member_metadata(frontmatter))
                        .and_then(|frontmatter| {
                            serde_yaml::from_str::<PackageMember>(&frontmatter).ok()
                        })
                        .map(|identity| CanonicalPackageNode {
                            identity,
                            kind: None,
                            members: None,
                        })
                };
            let Some(localized) = localized else {
                diagnostics.push(Diagnostic::new(
                    code,
                    Severity::Error,
                    path,
                    format!(
                        "Localized package node for '{language}' has invalid identity metadata."
                    ),
                ));
                continue;
            };
            if localized.identity.id != canonical.identity.id
                || localized.identity.status != canonical.identity.status
                || localized.kind != canonical.kind
                || localized.members != canonical.members
            {
                diagnostics.push(Diagnostic::new(
                    code,
                    Severity::Error,
                    path,
                    format!("Localized package node for '{language}' must preserve canonical id, status, kind, and direct members."),
                ));
            }
        }
        let localized_root = root.join(&localized_root_rel);
        let localized_entries = WalkDir::new(&localized_root)
            .min_depth(1)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_type().is_dir()
                    || entry.path().extension().and_then(|value| value.to_str()) == Some("md")
                    || entry.path().file_name().and_then(|value| value.to_str())
                        == Some("manifest.yml")
            })
            .filter_map(|entry| {
                entry
                    .path()
                    .strip_prefix(&localized_root)
                    .ok()
                    .map(Path::to_path_buf)
            })
            .collect::<BTreeSet<_>>();
        for extra in localized_entries.difference(&canonical_entries) {
            diagnostics.push(Diagnostic::new(
                code,
                Severity::Error,
                localized_root_rel.join(extra).display().to_string(),
                format!("Localized package tree for '{language}' contains an extra node."),
            ));
        }
    }
}

fn localized_package_roots(config: &Config, package_rel: &Path) -> Vec<(String, PathBuf)> {
    let Some(base) = config
        .docs
        .bases
        .iter()
        .filter(|base| package_rel.starts_with(&base.root))
        .max_by_key(|base| base.root.components().count())
    else {
        return Vec::new();
    };
    let suffix = package_rel
        .strip_prefix(&base.root)
        .unwrap_or_else(|_| Path::new(""));
    base.localized_roots
        .iter()
        .map(|(language, root)| (language.clone(), root.join(suffix)))
        .collect()
}

fn markdown_frontmatter(text: &str) -> Option<&str> {
    let rest = text.strip_prefix("---\n")?;
    let end = rest.find("\n---")?;
    Some(&rest[..end])
}

fn yaml_declares_document_version(yaml: &str) -> bool {
    yaml_declares_field(yaml, "version")
}

fn yaml_declares_removed_member_metadata(yaml: &str) -> bool {
    yaml_declares_document_version(yaml) || yaml_declares_field(yaml, "source")
}

fn yaml_declares_field(yaml: &str, field: &str) -> bool {
    serde_yaml::from_str::<serde_yaml::Value>(yaml)
        .ok()
        .and_then(|value| value.as_mapping().cloned())
        .is_some_and(|mapping| mapping.contains_key(serde_yaml::Value::String(field.to_owned())))
}

fn resolve_governance_target<'a>(
    target: &GovernanceTarget,
    assets: &'a [GovernanceAsset],
    index: &BTreeMap<&str, usize>,
) -> Option<&'a GovernanceAsset> {
    index
        .get(target.id.as_str())
        .map(|position| &assets[*position])
}
