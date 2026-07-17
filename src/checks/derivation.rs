fn refinement_rank(level: RefinementLevel) -> u8 {
    match level {
        RefinementLevel::Intent => 0,
        RefinementLevel::Definition => 1,
        RefinementLevel::Implementation => 2,
    }
}

fn check_vertical_derivation(
    asset: &GovernanceAsset,
    graph: &GovernanceGraph,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match (asset.refinement_level, asset.reference_relation) {
        (RefinementLevel::Intent, ReferenceRelation::Body) => {
            reject_vertical_edges(
                asset,
                "formalizes",
                GovernanceEdgeKind::Formalizes,
                "DH_DERIVATION_001",
                graph,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "realizes",
                GovernanceEdgeKind::Realizes,
                "DH_DERIVATION_001",
                graph,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "projects",
                GovernanceEdgeKind::Projects,
                "DH_DERIVATION_001",
                graph,
                diagnostics,
            );
        }
        (RefinementLevel::Definition, ReferenceRelation::Body) => {
            require_vertical_edge(
                asset,
                "formalizes",
                GovernanceEdgeKind::Formalizes,
                RefinementLevel::Intent,
                ReferenceRelation::Body,
                "DH_DERIVATION_001",
                graph,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "realizes",
                GovernanceEdgeKind::Realizes,
                "DH_DERIVATION_001",
                graph,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "projects",
                GovernanceEdgeKind::Projects,
                "DH_DERIVATION_001",
                graph,
                diagnostics,
            );
        }
        (RefinementLevel::Implementation, ReferenceRelation::Body) => {
            require_vertical_edge(
                asset,
                "realizes",
                GovernanceEdgeKind::Realizes,
                RefinementLevel::Definition,
                ReferenceRelation::Body,
                "DH_DERIVATION_001",
                graph,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "formalizes",
                GovernanceEdgeKind::Formalizes,
                "DH_DERIVATION_001",
                graph,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "projects",
                GovernanceEdgeKind::Projects,
                "DH_DERIVATION_001",
                graph,
                diagnostics,
            );
        }
        (RefinementLevel::Intent, ReferenceRelation::Library) => {
            reject_vertical_edges(
                asset,
                "formalizes",
                GovernanceEdgeKind::Formalizes,
                "DH_DERIVATION_002",
                graph,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "realizes",
                GovernanceEdgeKind::Realizes,
                "DH_DERIVATION_002",
                graph,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "projects",
                GovernanceEdgeKind::Projects,
                "DH_DERIVATION_002",
                graph,
                diagnostics,
            );
        }
        (RefinementLevel::Definition, ReferenceRelation::Library) => {
            require_vertical_edge(
                asset,
                "projects",
                GovernanceEdgeKind::Projects,
                RefinementLevel::Intent,
                ReferenceRelation::Library,
                "DH_DERIVATION_002",
                graph,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "formalizes",
                GovernanceEdgeKind::Formalizes,
                "DH_DERIVATION_002",
                graph,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "realizes",
                GovernanceEdgeKind::Realizes,
                "DH_DERIVATION_002",
                graph,
                diagnostics,
            );
        }
        (RefinementLevel::Implementation, ReferenceRelation::Library) => {
            require_vertical_edge(
                asset,
                "projects",
                GovernanceEdgeKind::Projects,
                RefinementLevel::Definition,
                ReferenceRelation::Library,
                "DH_DERIVATION_002",
                graph,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "formalizes",
                GovernanceEdgeKind::Formalizes,
                "DH_DERIVATION_002",
                graph,
                diagnostics,
            );
            reject_vertical_edges(
                asset,
                "realizes",
                GovernanceEdgeKind::Realizes,
                "DH_DERIVATION_002",
                graph,
                diagnostics,
            );
        }
    }
}

fn reject_vertical_edges(
    asset: &GovernanceAsset,
    edge_name: &str,
    relation: GovernanceEdgeKind,
    code: &'static str,
    graph: &GovernanceGraph,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if graph.edges_from(&asset.id, relation).next().is_some() {
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
    relation: GovernanceEdgeKind,
    expected_refinement_level: RefinementLevel,
    expected_reference_relation: ReferenceRelation,
    code: &'static str,
    graph: &GovernanceGraph,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let edges = graph.edges_from(&asset.id, relation).collect::<Vec<_>>();
    if edges.is_empty() {
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
    for edge in edges {
        let Some(target_asset) = graph.node(&edge.target) else {
            diagnostics.push(Diagnostic::new(
                code,
                Severity::Error,
                asset.path.clone(),
                format!(
                    "Vertical '{}' target '{}' does not exist.",
                    edge_name, edge.target
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
                        target_asset.identity,
                        target_asset.refinement_level.label(),
                        target_asset.reference_relation.label()
                    ),
                )
                .with_related(RelatedInformation::new(
                    target_asset.location.path.clone(),
                    "Resolved target is declared here.",
                )),
            );
        }
    }
}

fn check_vertical_derivation_completeness(
    assets: &[GovernanceAsset],
    graph: &GovernanceGraph,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for upstream in assets.iter().filter(|asset| {
        matches!(asset.status.as_str(), "baselined" | "current")
            && asset.refinement_level != RefinementLevel::Implementation
    }) {
        let relation = match (upstream.refinement_level, upstream.reference_relation) {
            (RefinementLevel::Intent, ReferenceRelation::Body) => {
                GovernanceEdgeKind::Formalizes
            }
            (RefinementLevel::Definition, ReferenceRelation::Body) => {
                GovernanceEdgeKind::Realizes
            }
            (RefinementLevel::Intent, ReferenceRelation::Library)
            | (RefinementLevel::Definition, ReferenceRelation::Library) => {
                GovernanceEdgeKind::Projects
            }
            (RefinementLevel::Implementation, _) => unreachable!(),
        };
        let derived = graph.edges_to(&upstream.id, relation).next().is_some();
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
