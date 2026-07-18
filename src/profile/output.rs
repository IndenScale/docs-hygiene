use anyhow::Result;

use super::{DimensionStatus, HygieneProfileReport, InvariantOutcome};

pub fn print_text_profile(report: &HygieneProfileReport) {
    let bound_profiles = report
        .document_templates
        .bindings
        .values()
        .map(Vec::len)
        .sum::<usize>();
    println!(
        "Document templates: {}, profiles bound: {}/{}, unused: {}, registry valid: {}, migration ready: {}.",
        report.document_templates.configured_templates,
        bound_profiles,
        report.document_templates.configured_profiles,
        report.document_templates.unused_templates.len(),
        report.document_templates.registry_valid,
        report.document_templates.proves_migration(),
    );
    println!(
        "Graph communities: {} bridge-connected communities, {} cross-community edges, {} baseline changes.",
        report.governance_graph.communities.len(),
        report.governance_graph.cross_community_edges.len(),
        report.governance_graph.community_changes.len(),
    );
    println!("Dimension    Applicable  Required  Target      Observed    Status");
    for dimension in &report.dimensions {
        println!(
            "{:<12} {:<11} {:<9} {:<11} {:<11} {}",
            dimension.dimension.label(),
            dimension.applicable,
            dimension.required,
            maturity_label(dimension.target),
            maturity_label(dimension.observed),
            dimension.status.label(),
        );
        for evidence in dimension.evidence.iter().filter(|evidence| {
            matches!(
                evidence.outcome,
                InvariantOutcome::Failed
                    | InvariantOutcome::Unverified
                    | InvariantOutcome::Excepted
            ) && dimension
                .target
                .is_none_or(|target| evidence.minimum_maturity <= target)
        }) {
            println!(
                "  - {} [{}]: {}",
                evidence.invariant,
                evidence.outcome.label(),
                evidence.reason
            );
        }
    }
    println!(
        "\nGovernance graph: {} nodes, {} edges, {} resolved, {} unresolved, {} ambiguous, {} incompatible, {} isolated; max Fan-In {}, max Fan-Out {}, {} cycle groups, max transitive impact {}.",
        report.governance_graph.metrics.nodes,
        report.governance_graph.metrics.edges,
        report.governance_graph.metrics.resolved_edges,
        report.governance_graph.metrics.unresolved_edges,
        report.governance_graph.metrics.ambiguous_edges,
        report.governance_graph.metrics.incompatible_edges,
        report.governance_graph.metrics.isolated_nodes,
        maximum_degree(&report.governance_graph.metrics.fan_in),
        maximum_degree(&report.governance_graph.metrics.fan_out),
        report.governance_graph.metrics.cycle_groups.len(),
        report
            .governance_graph
            .transitive_impact
            .values()
            .map(Vec::len)
            .max()
            .unwrap_or_default(),
    );
    if report.ownership.enabled {
        println!(
            "Ownership: responsibility {}%, review {}%, knowledge redundancy {}%; {} due soon, {} expired.",
            report.ownership.responsibility_coverage.percentage,
            report.ownership.review_coverage.percentage,
            report.ownership.knowledge_redundancy_coverage.percentage,
            report.ownership.reviews_due_soon,
            report.ownership.reviews_expired,
        );
    }
    println!(
        "Overall observed: {}; configured targets met: {}.",
        maturity_label(report.overall_observed),
        report.meets_targets
    );
}

fn maximum_degree(degrees: &std::collections::BTreeMap<String, usize>) -> usize {
    degrees.values().copied().max().unwrap_or_default()
}

pub fn print_json_profile(report: &HygieneProfileReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}

fn maturity_label(maturity: Option<crate::activation::HygieneMaturity>) -> &'static str {
    match maturity {
        Some(crate::activation::HygieneMaturity::Basic) => "basic",
        Some(crate::activation::HygieneMaturity::Controlled) => "controlled",
        Some(crate::activation::HygieneMaturity::Governed) => "governed",
        None => "-",
    }
}

impl DimensionStatus {
    fn label(self) -> &'static str {
        match self {
            Self::MeetsTarget => "meetsTarget",
            Self::BelowTarget => "belowTarget",
            Self::Observed => "observed",
            Self::Unverified => "unverified",
            Self::NotApplicable => "notApplicable",
        }
    }
}

impl InvariantOutcome {
    fn label(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Excepted => "excepted",
            Self::Failed => "failed",
            Self::Unverified => "unverified",
            Self::NotApplicable => "notApplicable",
        }
    }
}
