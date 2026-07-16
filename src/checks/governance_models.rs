#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
enum RefinementLevel {
    Intent,
    Definition,
    Implementation,
}

impl RefinementLevel {
    fn label(self) -> &'static str {
        match self {
            Self::Intent => "intent",
            Self::Definition => "definition",
            Self::Implementation => "implementation",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
enum ReferenceRelation {
    Body,
    Library,
}

impl ReferenceRelation {
    fn label(self) -> &'static str {
        match self {
            Self::Body => "body",
            Self::Library => "library",
        }
    }
}

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

    fn is_empty(&self) -> bool {
        self.0.is_empty()
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
struct PackageMember {
    id: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct PackageDomain {
    id: String,
    status: String,
    kind: String,
    members: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct PackageManifestNode {
    id: String,
    status: String,
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

fn check_governance(root: &Path, config: &Config, diagnostics: &mut Vec<Diagnostic>) {
    if config.governance.manifests.is_empty() {
        return;
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

    for asset in &assets {
        check_package_members(root, config, asset, diagnostics);
        check_vertical_derivation(asset, &assets, &index, diagnostics);
    }
    check_wiki_references(root, config, &assets, diagnostics);
    if config.governance.require_complete_vertical_derivation {
        check_vertical_derivation_completeness(&assets, diagnostics);
    }
}
