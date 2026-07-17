#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct DependencyKey<'a> {
    source: &'a str,
    target: &'a str,
    relation: CriticalDependencyRelation,
}

fn check_critical_dependencies(
    root: &Path,
    config: &Config,
    graph: &GovernanceGraph,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if config.governance.critical_dependencies.is_empty() {
        return;
    }
    let mut policy_ids = BTreeSet::new();
    for policy in &config.governance.critical_dependencies {
        validate_critical_policy(policy, &mut policy_ids, diagnostics);
    }
    let dependencies = governed_dependencies(graph);
    for (key, edge) in dependencies {
        let Some(source) = graph.node(key.source) else {
            continue;
        };
        let Some(target) = graph.node(key.target) else {
            continue;
        };
        for policy in &config.governance.critical_dependencies {
            if !critical_policy_matches(policy, key, edge, source, target) {
                continue;
            }
            check_critical_pin(root, config, graph, policy, key, edge, target, diagnostics);
        }
    }
}

fn validate_critical_policy<'a>(
    policy: &'a CriticalDependencyPolicyConfig,
    ids: &mut BTreeSet<&'a str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !valid_claim_identity(&policy.id) || !ids.insert(&policy.id) {
        diagnostics.push(Diagnostic::new(
            "DH_PIN_006",
            Severity::Error,
            "docs-hygiene.yml",
            format!(
                "Critical dependency policy identity '{}' is invalid or duplicated.",
                policy.id
            ),
        ));
    }
    if policy.require.algorithms.is_empty()
        || policy
            .require
            .algorithms
            .iter()
            .any(|algorithm| !matches!(algorithm.as_str(), "sha256" | "git"))
    {
        diagnostics.push(Diagnostic::new(
            "DH_PIN_006",
            Severity::Error,
            "docs-hygiene.yml",
            format!(
                "Critical dependency policy '{}' requires a non-empty algorithms list containing only 'sha256' or 'git'.",
                policy.id
            ),
        ));
    }
    if policy.require.max_age_days == Some(0) {
        diagnostics.push(Diagnostic::new(
            "DH_PIN_006",
            Severity::Error,
            "docs-hygiene.yml",
            format!(
                "Critical dependency policy '{}' maxAgeDays must be greater than zero.",
                policy.id
            ),
        ));
    }
    if (policy.require.minimum_scope == ContentAnchorScope::Block
        || policy.require.forbid_whole_file)
        && !policy
            .require
            .algorithms
            .iter()
            .any(|algorithm| algorithm == "sha256")
    {
        diagnostics.push(Diagnostic::new(
            "DH_PIN_006",
            Severity::Error,
            "docs-hygiene.yml",
            format!(
                "Critical dependency policy '{}' requires block scope but does not allow 'sha256'.",
                policy.id
            ),
        ));
    }
    for pattern in policy
        .matcher
        .source_paths
        .iter()
        .chain(&policy.matcher.target_paths)
    {
        if let Err(error) = Glob::new(pattern) {
            diagnostics.push(Diagnostic::new(
                "DH_PIN_006",
                Severity::Error,
                "docs-hygiene.yml",
                format!(
                    "Critical dependency policy '{}' has invalid path glob '{}': {error}.",
                    policy.id, pattern
                ),
            ));
        }
    }
}

fn governed_dependencies(
    graph: &GovernanceGraph,
) -> BTreeMap<DependencyKey<'_>, &GovernanceEdge> {
    let mut dependencies = BTreeMap::new();
    for edge in &graph.edges {
        let relation = CriticalDependencyRelation::from_edge_kind(edge.relation);
        let key = DependencyKey {
            source: &edge.source,
            target: &edge.target,
            relation,
        };
        let replace = dependencies
            .get(&key)
            .is_none_or(|current: &&GovernanceEdge| {
                current.relation != GovernanceEdgeKind::PinnedReference
                    && edge.relation == GovernanceEdgeKind::PinnedReference
            });
        if replace {
            dependencies.insert(key, edge);
        }
    }
    dependencies
}

fn critical_policy_matches(
    policy: &CriticalDependencyPolicyConfig,
    key: DependencyKey<'_>,
    edge: &GovernanceEdge,
    source: &GovernanceNode,
    target: &GovernanceNode,
) -> bool {
    let matcher = &policy.matcher;
    list_matches(&matcher.source_kinds, source.reference_relation)
        && list_matches(&matcher.target_kinds, target.reference_relation)
        && list_matches(&matcher.relations, key.relation)
        && string_list_matches(&matcher.source_ids, key.source)
        && string_list_matches(&matcher.target_ids, key.target)
        && path_list_matches(&matcher.source_paths, &edge.source_location.path)
        && path_list_matches(&matcher.target_paths, &target.location.path)
}

fn list_matches<T: PartialEq>(configured: &[T], actual: T) -> bool {
    configured.is_empty() || configured.contains(&actual)
}

fn string_list_matches(configured: &[String], actual: &str) -> bool {
    configured.is_empty() || configured.iter().any(|value| value == actual)
}

fn path_list_matches(configured: &[String], actual: &str) -> bool {
    if configured.is_empty() {
        return true;
    }
    let mut builder = GlobSetBuilder::new();
    for pattern in configured {
        let Ok(pattern) = Glob::new(pattern) else {
            return false;
        };
        builder.add(pattern);
    }
    builder
        .build()
        .is_ok_and(|patterns| patterns.is_match(Path::new(actual)))
}

#[allow(clippy::too_many_arguments)]
fn check_critical_pin(
    root: &Path,
    config: &Config,
    graph: &GovernanceGraph,
    policy: &CriticalDependencyPolicyConfig,
    key: DependencyKey<'_>,
    edge: &GovernanceEdge,
    target: &GovernanceNode,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let pins = graph
        .edges
        .iter()
        .filter(|candidate| {
            candidate.source == key.source
                && candidate.target == key.target
                && candidate.relation == GovernanceEdgeKind::PinnedReference
        })
        .collect::<Vec<_>>();
    if pins.is_empty() {
        let declaration_corrupt = diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DH_REFERENCE_001"
                && diagnostic.path == edge.source_location.path
                && (diagnostic.message.contains("anchor")
                    || diagnostic.message.contains("Invalid semantic Wiki Link"))
        });
        push_pin_diagnostic(
            if declaration_corrupt {
                "DH_PIN_006"
            } else {
                "DH_PIN_001"
            },
            policy,
            key,
            edge,
            target,
            graph,
            diagnostics,
            if declaration_corrupt {
                "critical dependency anchor declaration is corrupt".to_owned()
            } else {
                "critical dependency has no content anchor".to_owned()
            },
        );
        return;
    }
    let algorithms = pins
        .iter()
        .filter(|pin| {
            pin.content_anchor.as_ref().is_some_and(|anchor| {
                policy
                    .require
                    .algorithms
                    .iter()
                    .any(|allowed| allowed == anchor.algorithm)
            })
        })
        .copied()
        .collect::<Vec<_>>();
    if algorithms.is_empty() {
        push_pin_diagnostic(
            "DH_PIN_003",
            policy,
            key,
            edge,
            target,
            graph,
            diagnostics,
            format!(
                "critical dependency uses a disallowed anchor algorithm; allowed: {}",
                policy.require.algorithms.join(", ")
            ),
        );
        return;
    }
    let scopes = algorithms
        .iter()
        .filter(|pin| {
            pin.content_anchor.as_ref().is_some_and(|anchor| {
                anchor.scope.meets_minimum(policy.require.minimum_scope)
                    && !(policy.require.forbid_whole_file
                        && anchor.scope.covers_whole_file())
            })
        })
        .copied()
        .collect::<Vec<_>>();
    if scopes.is_empty() {
        push_pin_diagnostic(
            "DH_PIN_002",
            policy,
            key,
            edge,
            target,
            graph,
            diagnostics,
            format!(
                "critical dependency anchor scope is below {:?} or violates forbidWholeFile",
                policy.require.minimum_scope
            ),
        );
        return;
    }
    let mut current = Vec::new();
    let mut stale = Vec::new();
    let mut invalid = Vec::new();
    for pin in scopes {
        match critical_pin_state(root, config, pin, target) {
            PinState::Current => current.push(pin),
            PinState::Stale { actual } => stale.push((pin, actual)),
            PinState::Invalid(reason) => invalid.push(reason),
        }
    }
    if current.is_empty() {
        if let Some((pin, actual)) = stale.first() {
            let expected = pin
                .content_anchor
                .as_ref()
                .map(|anchor| anchor.digest.as_str())
                .unwrap_or_default();
            push_pin_diagnostic(
                "DH_PIN_004",
                policy,
                key,
                edge,
                target,
                graph,
                diagnostics,
                format!(
                    "critical dependency content changed: expected digest '{expected}', actual '{actual}'"
                ),
            );
        } else {
            push_pin_diagnostic(
                "DH_PIN_006",
                policy,
                key,
                edge,
                target,
                graph,
                diagnostics,
                format!(
                    "critical dependency anchor declaration cannot be verified: {}",
                    invalid.join("; ")
                ),
            );
        }
        return;
    }
    if let Some(max_age) = policy.require.max_age_days
        && !current.iter().any(|pin| pin_is_fresh(pin, max_age))
    {
        push_pin_diagnostic(
            "DH_PIN_005",
            policy,
            key,
            edge,
            target,
            graph,
            diagnostics,
            format!(
                "critical dependency pin is missing updatedAt/updatedBy/reason audit metadata or is older than {max_age} days"
            ),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn push_pin_diagnostic(
    code: &'static str,
    policy: &CriticalDependencyPolicyConfig,
    key: DependencyKey<'_>,
    edge: &GovernanceEdge,
    target: &GovernanceNode,
    graph: &GovernanceGraph,
    diagnostics: &mut Vec<Diagnostic>,
    reason: String,
) {
    let impact = graph
        .transitive_impact
        .get(key.target)
        .cloned()
        .unwrap_or_default();
    let mut diagnostic = Diagnostic::new(
        code,
        Severity::Error,
        edge.source_location.path.clone(),
        format!(
            "Critical dependency policy '{}': {reason}. Direct dependent: '{}'; reverse transitive impact: [{}].",
            policy.id,
            key.source,
            impact.join(", ")
        ),
    )
    .with_related(RelatedInformation::new(
        target.location.path.clone(),
        "Critical dependency target is here.",
    ));
    if let Some(line) = edge.source_location.line {
        diagnostic = diagnostic.at_line(line);
    }
    diagnostics.push(diagnostic);
}
