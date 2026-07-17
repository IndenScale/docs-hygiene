fn check_topology_policy(
    config: &Config,
    graph: &GovernanceGraph,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(limit) = config.governance.topology.max_fan_in {
        check_fan_threshold("Fan-In", "maxFanIn", limit, &graph.metrics.fan_in, graph, diagnostics);
    }
    if let Some(limit) = config.governance.topology.max_fan_out {
        check_fan_threshold(
            "Fan-Out",
            "maxFanOut",
            limit,
            &graph.metrics.fan_out,
            graph,
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
}

fn check_fan_threshold(
    label: &str,
    policy_name: &str,
    limit: usize,
    degrees: &BTreeMap<String, usize>,
    graph: &GovernanceGraph,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (identity, degree) in degrees.iter().filter(|(_, degree)| **degree > limit) {
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
