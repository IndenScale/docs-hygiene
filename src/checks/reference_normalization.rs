type ReferenceSignature = (
    String,
    Option<String>,
    Option<(
        String,
        String,
        ContentAnchorScope,
        Option<String>,
        Option<String>,
        Option<SnapshotProvenance>,
    )>,
);

fn semantic_reference_signatures(
    occurrences: &BTreeSet<ReferenceOccurrence>,
    policies: &[ReferencePolicy],
) -> BTreeSet<ReferenceSignature> {
    occurrences
        .iter()
        .filter(|occurrence| {
            reference_disposition(occurrence, policies)
                == Some(ReferenceDisposition::SemanticDependency)
        })
        .map(|occurrence| {
            (
                occurrence.raw_target.clone(),
                occurrence.payload.selector.clone(),
                occurrence
                    .payload
                    .anchor
                    .as_ref()
                    .map(|anchor| {
                        (
                            anchor.algorithm.clone(),
                            anchor.digest.clone(),
                            anchor.scope,
                            anchor.locator.clone(),
                            anchor.expected_document_kind.clone(),
                            anchor.snapshot.clone(),
                        )
                    }),
            )
        })
        .collect()
}

fn normalize_reference_edges(
    asset: &GovernanceAsset,
    occurrences: &BTreeSet<ReferenceOccurrence>,
    targets: &BTreeMap<String, SemanticTarget>,
) -> Vec<GovernanceEdge> {
    normalize_reference_edges_with_policies(asset, occurrences, targets, REFERENCE_POLICIES)
}

fn normalize_reference_edges_with_policies(
    asset: &GovernanceAsset,
    occurrences: &BTreeSet<ReferenceOccurrence>,
    targets: &BTreeMap<String, SemanticTarget>,
    policies: &[ReferencePolicy],
) -> Vec<GovernanceEdge> {
    occurrences
        .iter()
        .filter(|occurrence| {
            reference_disposition(occurrence, policies)
                == Some(ReferenceDisposition::SemanticDependency)
        })
        .filter_map(|occurrence| {
            let content_anchor = match &occurrence.payload.anchor {
                Some(anchor) if matches!(anchor.algorithm.as_str(), "sha256" | "git") => {
                    Some(ContentAnchor {
                        algorithm: if anchor.algorithm == "sha256" {
                            "sha256"
                        } else {
                            "git"
                        },
                        digest: anchor.digest.clone(),
                        scope: anchor.scope,
                        locator: anchor.locator.clone(),
                        updated_at: anchor.updated_at.clone(),
                        updated_by: anchor.updated_by.clone(),
                        reason: anchor.reason.clone(),
                        snapshot: anchor.snapshot.clone(),
                    })
                }
                Some(_) => return None,
                None => None,
            };
            let relation = if content_anchor.is_some() {
                GovernanceEdgeKind::PinnedReference
            } else {
                GovernanceEdgeKind::SemanticReference
            };
            let expectation = reference_expectation(
                asset,
                &occurrence.raw_target,
                relation,
                occurrence
                    .payload
                    .anchor
                    .as_ref()
                    .and_then(|anchor| anchor.expected_document_kind.clone()),
            );
            let resolution = targets
                .get(&occurrence.raw_target)
                .map(|target| {
                    let mut candidates = vec![ReferenceEndpoint {
                        refinement_level: target.refinement_level,
                        reference_relation: target.reference_relation,
                        document_kind: target.document_kind.clone(),
                        lifecycle_status: target.status.clone(),
                        location: GovernanceLocation {
                            path: target.path.clone(),
                            line: None,
                        },
                    }];
                    candidates.extend(target.alternates.iter().map(|candidate| {
                        ReferenceEndpoint {
                            refinement_level: candidate.refinement_level,
                            reference_relation: candidate.reference_relation,
                            document_kind: candidate.document_kind.clone(),
                            lifecycle_status: candidate.status.clone(),
                            location: GovernanceLocation {
                                path: candidate.path.clone(),
                                line: None,
                            },
                        }
                    }));
                    candidates
                });
            Some(GovernanceEdge {
                source: asset.id.clone(),
                target: occurrence.raw_target.clone(),
                relation,
                source_location: occurrence.location.clone(),
                selector: occurrence.payload.selector.clone(),
                content_anchor,
                lifecycle: LifecycleProvenance {
                    source_status: asset.status.clone(),
                    target_status: targets
                        .get(&occurrence.raw_target)
                        .map(|target| target.status.clone()),
                },
                resolution: resolve_reference(&expectation, resolution.as_ref()),
                expectation,
            })
        })
        .collect()
}

fn reference_expectation(
    asset: &GovernanceAsset,
    target: &str,
    relation: GovernanceEdgeKind,
    expected_document_kind: Option<String>,
) -> ReferenceExpectation {
    let document_kinds = expected_document_kind.into_iter().collect::<Vec<_>>();
    let vertical = [
        (
            GovernanceEdgeKind::Formalizes,
            &asset.formalizes,
            RefinementLevel::Intent,
            ReferenceRelation::Body,
        ),
        (
            GovernanceEdgeKind::Realizes,
            &asset.realizes,
            RefinementLevel::Definition,
            ReferenceRelation::Body,
        ),
    ]
    .into_iter()
    .find(|(_, targets, _, _)| targets.iter().any(|candidate| candidate.id == target));
    if let Some((_, _, level, target_relation)) = vertical {
        return ReferenceExpectation::new(
            relation,
            vec![level],
            vec![target_relation],
            document_kinds,
        );
    }
    if asset.projects.iter().any(|candidate| candidate.id == target) {
        let levels = match asset.refinement_level {
            RefinementLevel::Intent => Vec::new(),
            RefinementLevel::Definition => vec![RefinementLevel::Intent],
            RefinementLevel::Implementation => vec![RefinementLevel::Definition],
        };
        return ReferenceExpectation::new(
            relation,
            levels,
            vec![ReferenceRelation::Library],
            document_kinds,
        );
    }
    let levels = match asset.reference_relation {
        ReferenceRelation::Body => vec![asset.refinement_level],
        ReferenceRelation::Library => match asset.refinement_level {
            RefinementLevel::Intent => vec![RefinementLevel::Intent],
            RefinementLevel::Definition => {
                vec![RefinementLevel::Intent, RefinementLevel::Definition]
            }
            RefinementLevel::Implementation => {
                vec![RefinementLevel::Definition, RefinementLevel::Implementation]
            }
        },
    };
    ReferenceExpectation::new(
        relation,
        levels,
        vec![ReferenceRelation::Library],
        document_kinds,
    )
}
