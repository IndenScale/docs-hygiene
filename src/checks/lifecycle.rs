#[derive(Clone, Debug)]
struct GovernedIdentityRecord {
    id: String,
    status: String,
    superseded_by: Option<String>,
    reference_relation: ReferenceRelation,
    path: String,
}

fn collect_governed_identity_records(
    root: &Path,
    assets: &[GovernanceAsset],
) -> Vec<GovernedIdentityRecord> {
    let mut records = Vec::new();
    for asset in assets {
        records.push(GovernedIdentityRecord {
            id: asset.id.clone(),
            status: asset.status.clone(),
            superseded_by: asset.superseded_by.clone(),
            reference_relation: asset.reference_relation,
            path: asset.path.clone(),
        });
        let manifest = Path::new(&asset.path);
        if manifest.file_name().and_then(|value| value.to_str()) != Some("manifest.yml")
        {
            continue;
        }
        let Some(serde_yaml::Value::Sequence(members)) = &asset.members else {
            continue;
        };
        let Some(members) = member_strings(members) else {
            continue;
        };
        let package = manifest.parent().unwrap_or_else(|| Path::new(""));
        collect_package_identity_records(
            root,
            package,
            Path::new(""),
            &members,
            asset.reference_relation,
            &mut records,
        );
    }
    records
}

#[allow(clippy::too_many_arguments)]
fn collect_package_identity_records(
    root: &Path,
    package: &Path,
    directory: &Path,
    members: &[String],
    reference_relation: ReferenceRelation,
    records: &mut Vec<GovernedIdentityRecord>,
) {
    for member in members {
        let member_path = Path::new(member);
        if member_path.is_absolute() || member_path.components().count() != 1 {
            continue;
        }
        let relative = directory.join(member_path);
        let absolute = root.join(package).join(&relative);
        if absolute.is_file() && absolute.extension().and_then(|value| value.to_str()) == Some("md")
        {
            let identity = std::fs::read_to_string(&absolute)
                .ok()
                .and_then(|text| markdown_frontmatter(&text).map(str::to_owned))
                .and_then(|yaml| serde_yaml::from_str::<PackageMember>(&yaml).ok());
            if let Some(identity) = identity {
                records.push(GovernedIdentityRecord {
                    id: identity.id,
                    status: identity.status,
                    superseded_by: identity.superseded_by,
                    reference_relation,
                    path: package.join(&relative).display().to_string(),
                });
            }
            continue;
        }
        if !absolute.is_dir() {
            continue;
        }
        let manifest = relative.join("manifest.yml");
        let domain = std::fs::read_to_string(absolute.join("manifest.yml"))
            .ok()
            .and_then(|yaml| serde_yaml::from_str::<PackageDomain>(&yaml).ok());
        if let Some(domain) = domain {
            records.push(GovernedIdentityRecord {
                id: domain.id,
                status: domain.status,
                superseded_by: domain.superseded_by,
                reference_relation,
                path: package.join(&manifest).display().to_string(),
            });
            collect_package_identity_records(
                root,
                package,
                &relative,
                &domain.members,
                reference_relation,
                records,
            );
        }
    }
}

fn check_identity_lifecycle(
    records: &[GovernedIdentityRecord],
    graph: &GovernanceGraph,
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, String> {
    let index = records
        .iter()
        .map(|record| (record.id.as_str(), record))
        .collect::<BTreeMap<_, _>>();
    let migrations = records
        .iter()
        .filter_map(|record| {
            record
                .superseded_by
                .as_ref()
                .map(|successor| (record.id.clone(), successor.clone()))
        })
        .collect::<BTreeMap<_, _>>();

    for record in records {
        let Some(status) = LifecycleStatus::parse(&record.status) else {
            continue;
        };
        match (status.requires_successor(), record.superseded_by.as_deref()) {
            (true, None) => push_lifecycle_diagnostic(
                record,
                diagnostics,
                format!(
                    "Superseded identity '{}' must declare supersededBy.",
                    record.id
                ),
            ),
            (true, Some(successor)) => {
                validate_authority_successor(record, successor, &index, diagnostics)
            }
            (false, Some(successor)) => push_lifecycle_diagnostic(
                record,
                diagnostics,
                format!(
                    "Identity '{}' may declare supersededBy '{}' only when status is superseded.",
                    record.id, successor
                ),
            ),
            (false, None) => {}
        }
    }

    for edge in &graph.edges {
        let Some(target) = index.get(edge.target.as_str()) else {
            continue;
        };
        if !LifecycleStatus::parse(&target.status).is_some_and(LifecycleStatus::is_terminal) {
            continue;
        }
        let replacement = target
            .superseded_by
            .as_deref()
            .map(|successor| format!("; migrate the dependency to '{successor}'"))
            .unwrap_or_default();
        let mut diagnostic = Diagnostic::new(
            "DH_GOVERNANCE_001",
            Severity::Error,
            edge.source_location.path.clone(),
            format!(
                "Governance edge from '{}' targets non-authoritative {} identity '{}'{}.",
                edge.source, target.status, target.id, replacement
            ),
        )
        .with_related(RelatedInformation::new(
            target.path.clone(),
            "Non-authoritative identity is declared here.",
        ));
        if let Some(line) = edge.source_location.line {
            diagnostic = diagnostic.at_line(line);
        }
        diagnostics.push(diagnostic);
    }
    migrations
}

fn validate_authority_successor(
    record: &GovernedIdentityRecord,
    successor_id: &str,
    index: &BTreeMap<&str, &GovernedIdentityRecord>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if successor_id == record.id {
        push_lifecycle_diagnostic(
            record,
            diagnostics,
            format!("Identity '{}' cannot supersede itself.", record.id),
        );
        return;
    }
    let Some(successor) = index.get(successor_id) else {
        push_lifecycle_diagnostic(
            record,
            diagnostics,
            format!(
                "Superseded identity '{}' points to unknown successor '{}'.",
                record.id, successor_id
            ),
        );
        return;
    };
    if successor.reference_relation != record.reference_relation {
        push_lifecycle_diagnostic(
            record,
            diagnostics,
            format!(
                "Authority successor '{}' for '{}' must preserve referenceRelation.",
                successor_id, record.id
            ),
        );
    }
    if !LifecycleStatus::parse(&successor.status).is_some_and(LifecycleStatus::is_established) {
        push_lifecycle_diagnostic(
            record,
            diagnostics,
            format!(
                "Authority successor '{}' for '{}' must be baselined or current, not '{}'.",
                successor_id, record.id, successor.status
            ),
        );
    }
}

fn push_lifecycle_diagnostic(
    record: &GovernedIdentityRecord,
    diagnostics: &mut Vec<Diagnostic>,
    message: String,
) {
    diagnostics.push(Diagnostic::new(
        "DH_GOVERNANCE_001",
        Severity::Error,
        record.path.clone(),
        message,
    ));
}
