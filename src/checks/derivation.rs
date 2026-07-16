fn refinement_rank(level: RefinementLevel) -> u8 {
    match level {
        RefinementLevel::Intent => 0,
        RefinementLevel::Definition => 1,
        RefinementLevel::Implementation => 2,
    }
}

fn check_vertical_derivation(
    asset: &GovernanceAsset,
    assets: &[GovernanceAsset],
    index: &BTreeMap<&str, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match (asset.refinement_level, asset.reference_relation) {
        (RefinementLevel::Intent, ReferenceRelation::Body) => {
            reject_vertical_edges(
                asset,
                "formalizes",
                &asset.formalizes,
                "DH_DERIVATION_001",
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "realizes",
                &asset.realizes,
                "DH_DERIVATION_001",
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "projects",
                &asset.projects,
                "DH_DERIVATION_001",
                diagnostics,
            );
        }
        (RefinementLevel::Definition, ReferenceRelation::Body) => {
            require_vertical_edge(
                asset,
                "formalizes",
                &asset.formalizes,
                RefinementLevel::Intent,
                ReferenceRelation::Body,
                "DH_DERIVATION_001",
                assets,
                index,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "realizes",
                &asset.realizes,
                "DH_DERIVATION_001",
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "projects",
                &asset.projects,
                "DH_DERIVATION_001",
                diagnostics,
            );
        }
        (RefinementLevel::Implementation, ReferenceRelation::Body) => {
            require_vertical_edge(
                asset,
                "realizes",
                &asset.realizes,
                RefinementLevel::Definition,
                ReferenceRelation::Body,
                "DH_DERIVATION_001",
                assets,
                index,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "formalizes",
                &asset.formalizes,
                "DH_DERIVATION_001",
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "projects",
                &asset.projects,
                "DH_DERIVATION_001",
                diagnostics,
            );
        }
        (RefinementLevel::Intent, ReferenceRelation::Library) => {
            reject_vertical_edges(
                asset,
                "formalizes",
                &asset.formalizes,
                "DH_DERIVATION_002",
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "realizes",
                &asset.realizes,
                "DH_DERIVATION_002",
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "projects",
                &asset.projects,
                "DH_DERIVATION_002",
                diagnostics,
            );
        }
        (RefinementLevel::Definition, ReferenceRelation::Library) => {
            require_vertical_edge(
                asset,
                "projects",
                &asset.projects,
                RefinementLevel::Intent,
                ReferenceRelation::Library,
                "DH_DERIVATION_002",
                assets,
                index,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "formalizes",
                &asset.formalizes,
                "DH_DERIVATION_002",
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "realizes",
                &asset.realizes,
                "DH_DERIVATION_002",
                diagnostics,
            );
        }
        (RefinementLevel::Implementation, ReferenceRelation::Library) => {
            require_vertical_edge(
                asset,
                "projects",
                &asset.projects,
                RefinementLevel::Definition,
                ReferenceRelation::Library,
                "DH_DERIVATION_002",
                assets,
                index,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "formalizes",
                &asset.formalizes,
                "DH_DERIVATION_002",
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "realizes",
                &asset.realizes,
                "DH_DERIVATION_002",
                diagnostics,
            );
        }
    }
}

fn reject_vertical_edges(
    asset: &GovernanceAsset,
    edge_name: &str,
    targets: &GovernanceTargets,
    code: &'static str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !targets.is_empty() {
        diagnostics.push(Diagnostic::new(
            code,
            Severity::Error,
            asset.path.clone(),
            format!(
                "{} {} '{}' cannot declare vertical '{}' edges.",
                asset.refinement_level.label(),
                asset.reference_relation.label(),
                asset.id,
                edge_name
            ),
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn require_vertical_edge(
    asset: &GovernanceAsset,
    edge_name: &str,
    targets: &GovernanceTargets,
    expected_refinement_level: RefinementLevel,
    expected_reference_relation: ReferenceRelation,
    code: &'static str,
    assets: &[GovernanceAsset],
    index: &BTreeMap<&str, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if targets.is_empty() {
        diagnostics.push(Diagnostic::new(
            code,
            Severity::Error,
            asset.path.clone(),
            format!(
                "{} {} '{}' must declare a vertical '{}' edge to an {} {}.",
                asset.refinement_level.label(),
                asset.reference_relation.label(),
                asset.id,
                edge_name,
                expected_refinement_level.label(),
                expected_reference_relation.label()
            ),
        ));
        return;
    }
    for target in targets.iter() {
        let Some(target_asset) = resolve_governance_target(target, assets, index) else {
            diagnostics.push(Diagnostic::new(
                code,
                Severity::Error,
                asset.path.clone(),
                format!(
                    "Vertical '{}' target '{}' does not exist.",
                    edge_name, target.id
                ),
            ));
            continue;
        };
        if target_asset.refinement_level != expected_refinement_level
            || target_asset.reference_relation != expected_reference_relation
        {
            diagnostics.push(
                Diagnostic::new(
                    code,
                    Severity::Error,
                    asset.path.clone(),
                    format!(
                        "Vertical '{}' from '{}' must target an {} {}, but '{}' is {} {}.",
                        edge_name,
                        asset.id,
                        expected_refinement_level.label(),
                        expected_reference_relation.label(),
                        target_asset.id,
                        target_asset.refinement_level.label(),
                        target_asset.reference_relation.label()
                    ),
                )
                .with_related(RelatedInformation::new(
                    target_asset.path.clone(),
                    "Resolved target is declared here.",
                )),
            );
        }
    }
}

fn check_vertical_derivation_completeness(
    assets: &[GovernanceAsset],
    diagnostics: &mut Vec<Diagnostic>,
) {
    for upstream in assets.iter().filter(|asset| {
        matches!(asset.status.as_str(), "baselined" | "current")
            && asset.refinement_level != RefinementLevel::Implementation
    }) {
        let derived = assets.iter().any(|downstream| {
            match (upstream.refinement_level, upstream.reference_relation) {
                (RefinementLevel::Intent, ReferenceRelation::Body) => downstream
                    .formalizes
                    .iter()
                    .any(|target| target.id == upstream.id),
                (RefinementLevel::Definition, ReferenceRelation::Body) => downstream
                    .realizes
                    .iter()
                    .any(|target| target.id == upstream.id),
                (RefinementLevel::Intent, ReferenceRelation::Library)
                | (RefinementLevel::Definition, ReferenceRelation::Library) => downstream
                    .projects
                    .iter()
                    .any(|target| target.id == upstream.id),
                (RefinementLevel::Implementation, _) => true,
            }
        });
        if !derived {
            let code = if upstream.reference_relation == ReferenceRelation::Body {
                "DH_DERIVATION_001"
            } else {
                "DH_DERIVATION_002"
            };
            diagnostics.push(Diagnostic::new(
                code,
                Severity::Error,
                upstream.path.clone(),
                format!(
                    "Baselined {} '{}' has no adjacent downstream derivation.",
                    upstream.reference_relation.label(),
                    upstream.id
                ),
            ));
        }
    }
}
