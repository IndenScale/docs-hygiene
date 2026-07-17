fn check_topology_policy(
    config: &Config,
    graph: &GovernanceGraph,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<TopologyExceptionEvidence> {
    let (mut exceptions, applied) = evaluate_supernode_exceptions(config, graph, diagnostics);
    if let Some(limit) = config.governance.topology.max_fan_in {
        check_fan_threshold(
            TopologyDirection::FanIn,
            "Fan-In",
            "maxFanIn",
            limit,
            &graph.metrics.fan_in,
            graph,
            &applied,
            diagnostics,
        );
    }
    if let Some(limit) = config.governance.topology.max_fan_out {
        check_fan_threshold(
            TopologyDirection::FanOut,
            "Fan-Out",
            "maxFanOut",
            limit,
            &graph.metrics.fan_out,
            graph,
            &applied,
            diagnostics,
        );
    }
    if config.governance.topology.forbid_cycles {
        for cycle in &graph.metrics.cycle_groups {
            let Some(first) = cycle.first() else {
                continue;
            };
            let path = graph
                .node(first)
                .map(|node| node.location.path.clone())
                .unwrap_or_else(|| ".".to_owned());
            let mut diagnostic = Diagnostic::new(
                "DH_TOPOLOGY_002",
                Severity::Error,
                path,
                format!(
                    "Governance cycle violates forbidCycles: {}.",
                    cycle.join(" -> ")
                ),
            );
            for identity in cycle.iter().skip(1) {
                if let Some(node) = graph.node(identity) {
                    diagnostic = diagnostic.with_related(RelatedInformation::new(
                        node.location.path.clone(),
                        format!("Cycle member '{identity}' is declared here."),
                    ));
                }
            }
            diagnostics.push(diagnostic);
        }
    }
    exceptions.sort_by(|left, right| {
        (&left.id, &left.node, left.direction).cmp(&(&right.id, &right.node, right.direction))
    });
    exceptions
}

fn evaluate_supernode_exceptions(
    config: &Config,
    graph: &GovernanceGraph,
    diagnostics: &mut Vec<Diagnostic>,
) -> (
    Vec<TopologyExceptionEvidence>,
    BTreeMap<(String, TopologyDirection), usize>,
) {
    let mut evidence = Vec::new();
    let mut applied = BTreeMap::new();
    let mut ids = BTreeSet::new();
    let mut targets = BTreeSet::new();
    for exception in &config.governance.topology.exceptions {
        let node = graph.node(&exception.node);
        let direction = exception.direction;
        let global_budget = direction.and_then(|direction| topology_limit(config, direction));
        let current_degree = direction.and_then(|direction| topology_degree(graph, &exception.node, direction));
        let duplicate_id = !ids.insert(exception.id.clone());
        let duplicate_target = direction
            .is_some_and(|direction| !targets.insert((exception.node.clone(), direction)));
        let expiry = parse_iso_date(&exception.expires);
        let expired = expiry
            .zip(today_utc())
            .is_some_and(|(expiry, today)| today > expiry);
        let history = validate_exception_history(exception);
        let base_valid = valid_claim_identity(&exception.id)
            && !exception.node.is_empty()
            && node.is_some()
            && direction.is_some()
            && global_budget.is_some()
            && global_budget.is_some_and(|limit| exception.budget > limit)
            && !exception.reason.trim().is_empty()
            && !exception.owner.trim().is_empty()
            && !exception.approved_by.trim().is_empty()
            && expiry.is_some()
            && !duplicate_id
            && !duplicate_target;
        let mut status = TopologyExceptionStatus::Invalid;
        if !base_valid {
            push_exception_diagnostic(
                "DH_TOPOLOGY_003",
                Severity::Error,
                exception,
                node,
                "requires unique stable id and node/direction, an existing node, a budget above the global limit, reason, owner, approvedBy, and valid expiry",
                diagnostics,
            );
        } else if expired {
            status = TopologyExceptionStatus::Expired;
            push_exception_diagnostic(
                "DH_TOPOLOGY_003",
                Severity::Error,
                exception,
                node,
                "is expired; renew expiry and approval metadata explicitly",
                diagnostics,
            );
        } else if history.is_err() {
            push_exception_diagnostic(
                "DH_TOPOLOGY_005",
                Severity::Error,
                exception,
                node,
                "has invalid, unordered, or future degree history",
                diagnostics,
            );
        } else if current_degree.zip(global_budget).is_some_and(|(degree, limit)| degree <= limit) {
            status = TopologyExceptionStatus::Idle;
            push_exception_diagnostic(
                "DH_TOPOLOGY_004",
                Severity::Warning,
                exception,
                node,
                "is idle because the node no longer exceeds its global direction limit; remove the exception",
                diagnostics,
            );
        } else if history.as_ref().is_ok_and(Option::is_none) {
            push_exception_diagnostic(
                "DH_TOPOLOGY_005",
                Severity::Error,
                exception,
                node,
                "requires at least one dated degree observation before it can except an active violation",
                diagnostics,
            );
        } else if current_degree.is_some_and(|degree| degree > exception.budget) {
            status = TopologyExceptionStatus::Exceeded;
        } else if let (Some(direction), Some(degree)) = (direction, current_degree) {
            status = TopologyExceptionStatus::Applied;
            applied.insert((exception.node.clone(), direction), exception.budget);
            debug_assert!(degree <= exception.budget);
        }
        let latest = history.ok().flatten();
        evidence.push(TopologyExceptionEvidence {
            id: exception.id.clone(),
            node: exception.node.clone(),
            direction,
            current_degree,
            global_budget,
            exception_budget: exception.budget,
            remaining: current_degree.map(|degree| signed_difference(exception.budget, degree)),
            reason: exception.reason.clone(),
            owner: exception.owner.clone(),
            approved_by: exception.approved_by.clone(),
            expires: exception.expires.clone(),
            latest_observed_at: latest.map(|observation| observation.observed_at.clone()),
            latest_observed_degree: latest.map(|observation| observation.degree),
            trend_delta: latest.zip(current_degree).map(|(observation, degree)| {
                signed_difference(degree, observation.degree)
            }),
            transitive_impact: graph
                .transitive_impact
                .get(&exception.node)
                .cloned()
                .unwrap_or_default(),
            status,
        });
    }
    (evidence, applied)
}

fn validate_exception_history(
    exception: &SupernodeExceptionConfig,
) -> std::result::Result<Option<&SupernodeDegreeObservationConfig>, ()> {
    let today = today_utc();
    let mut previous = None;
    for observation in &exception.history {
        let Some(date) = parse_iso_date(&observation.observed_at) else {
            return Err(());
        };
        if previous.is_some_and(|previous| date <= previous)
            || today.is_some_and(|today| date > today)
        {
            return Err(());
        }
        previous = Some(date);
    }
    Ok(exception.history.last())
}

fn topology_limit(config: &Config, direction: TopologyDirection) -> Option<usize> {
    match direction {
        TopologyDirection::FanIn => config.governance.topology.max_fan_in,
        TopologyDirection::FanOut => config.governance.topology.max_fan_out,
    }
}

fn topology_degree(
    graph: &GovernanceGraph,
    node: &str,
    direction: TopologyDirection,
) -> Option<usize> {
    match direction {
        TopologyDirection::FanIn => graph.metrics.fan_in.get(node).copied(),
        TopologyDirection::FanOut => graph.metrics.fan_out.get(node).copied(),
    }
}

fn signed_difference(left: usize, right: usize) -> i64 {
    i64::try_from(left)
        .unwrap_or(i64::MAX)
        .saturating_sub(i64::try_from(right).unwrap_or(i64::MAX))
}

fn push_exception_diagnostic(
    code: &'static str,
    severity: Severity,
    exception: &SupernodeExceptionConfig,
    node: Option<&GovernanceNode>,
    reason: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    diagnostics.push(Diagnostic::new(
        code,
        severity,
        node.map(|node| node.location.path.clone())
            .unwrap_or_else(|| "docs-hygiene.yml".to_owned()),
        format!("Supernode exception '{}': {reason}.", exception.id),
    ));
}

#[allow(clippy::too_many_arguments)]
fn check_fan_threshold(
    direction: TopologyDirection,
    label: &str,
    policy_name: &str,
    limit: usize,
    degrees: &BTreeMap<String, usize>,
    graph: &GovernanceGraph,
    applied: &BTreeMap<(String, TopologyDirection), usize>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (identity, degree) in degrees.iter().filter(|(_, degree)| **degree > limit) {
        if applied
            .get(&(identity.clone(), direction))
            .is_some_and(|budget| degree <= budget)
        {
            continue;
        }
        let path = graph
            .node(identity)
            .map(|node| node.location.path.clone())
            .unwrap_or_else(|| ".".to_owned());
        diagnostics.push(Diagnostic::new(
            "DH_TOPOLOGY_001",
            Severity::Error,
            path,
            format!(
                "Governed identity '{identity}' has {label} {degree}, exceeding configured {policy_name} {limit}."
            ),
        ));
    }
}
