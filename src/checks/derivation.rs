#[derive(Clone, Copy)]
struct RequiredVerticalEdge {
    field: &'static str,
    relation: GovernanceEdgeKind,
    target_level: RefinementLevel,
    target_relation: ReferenceRelation,
}

#[derive(Clone, Copy)]
struct VerticalDerivationPolicy {
    source_level: RefinementLevel,
    source_relation: ReferenceRelation,
    diagnostic: &'static str,
    required: Option<RequiredVerticalEdge>,
}

const VERTICAL_EDGE_FIELDS: [(&str, GovernanceEdgeKind); 3] = [
    ("formalizes", GovernanceEdgeKind::Formalizes),
    ("realizes", GovernanceEdgeKind::Realizes),
    ("projects", GovernanceEdgeKind::Projects),
];

const VERTICAL_DERIVATION_POLICIES: [VerticalDerivationPolicy; 6] = [
    VerticalDerivationPolicy {
        source_level: RefinementLevel::Intent,
        source_relation: ReferenceRelation::Body,
        diagnostic: "DH_DERIVATION_001",
        required: None,
    },
    VerticalDerivationPolicy {
        source_level: RefinementLevel::Definition,
        source_relation: ReferenceRelation::Body,
        diagnostic: "DH_DERIVATION_001",
        required: Some(RequiredVerticalEdge {
            field: "formalizes",
            relation: GovernanceEdgeKind::Formalizes,
            target_level: RefinementLevel::Intent,
            target_relation: ReferenceRelation::Body,
        }),
    },
    VerticalDerivationPolicy {
        source_level: RefinementLevel::Implementation,
        source_relation: ReferenceRelation::Body,
        diagnostic: "DH_DERIVATION_001",
        required: Some(RequiredVerticalEdge {
            field: "realizes",
            relation: GovernanceEdgeKind::Realizes,
            target_level: RefinementLevel::Definition,
            target_relation: ReferenceRelation::Body,
        }),
    },
    VerticalDerivationPolicy {
        source_level: RefinementLevel::Intent,
        source_relation: ReferenceRelation::Library,
        diagnostic: "DH_DERIVATION_002",
        required: None,
    },
    VerticalDerivationPolicy {
        source_level: RefinementLevel::Definition,
        source_relation: ReferenceRelation::Library,
        diagnostic: "DH_DERIVATION_002",
        required: Some(RequiredVerticalEdge {
            field: "projects",
            relation: GovernanceEdgeKind::Projects,
            target_level: RefinementLevel::Intent,
            target_relation: ReferenceRelation::Library,
        }),
    },
    VerticalDerivationPolicy {
        source_level: RefinementLevel::Implementation,
        source_relation: ReferenceRelation::Library,
        diagnostic: "DH_DERIVATION_002",
        required: Some(RequiredVerticalEdge {
            field: "projects",
            relation: GovernanceEdgeKind::Projects,
            target_level: RefinementLevel::Definition,
            target_relation: ReferenceRelation::Library,
        }),
    },
];

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
    let policy = VERTICAL_DERIVATION_POLICIES
        .iter()
        .find(|policy| {
            policy.source_level == asset.refinement_level
                && policy.source_relation == asset.reference_relation
        })
        .expect("vertical policy covers every refinement/relation pair");
    for (field, relation) in VERTICAL_EDGE_FIELDS {
        if let Some(required) = policy.required.filter(|required| required.relation == relation) {
            require_vertical_edge(
                asset,
                required.field,
                required.relation,
                required.target_level,
                required.target_relation,
                policy.diagnostic,
                graph,
                diagnostics,
            );
        } else {
            reject_vertical_edges(
                asset,
                field,
                relation,
                policy.diagnostic,
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
        LifecycleStatus::parse(&asset.status).is_some_and(LifecycleStatus::is_established)
            && asset.refinement_level != RefinementLevel::Implementation
    }) {
        let relation = VERTICAL_DERIVATION_POLICIES
            .iter()
            .filter_map(|policy| policy.required)
            .find(|required| {
                required.target_level == upstream.refinement_level
                    && required.target_relation == upstream.reference_relation
            })
            .expect("non-implementation policy has one adjacent downstream relation")
            .relation;
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
