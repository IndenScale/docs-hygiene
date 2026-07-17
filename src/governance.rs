use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

// Governance Library: [[SDK-001]]

mod impact;
mod topology;

use impact::analyze_transitive_impact;
use topology::analyze_topology;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RefinementLevel {
    Intent,
    Definition,
    Implementation,
}

impl RefinementLevel {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Intent => "intent",
            Self::Definition => "definition",
            Self::Implementation => "implementation",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ReferenceRelation {
    Body,
    Library,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LifecycleStatus {
    Draft,
    Review,
    Proposed,
    Baselined,
    Current,
    Superseded,
    Archived,
    Abandoned,
}

impl LifecycleStatus {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "draft" => Some(Self::Draft),
            "review" => Some(Self::Review),
            "proposed" => Some(Self::Proposed),
            "baselined" => Some(Self::Baselined),
            "current" => Some(Self::Current),
            "superseded" => Some(Self::Superseded),
            "archived" => Some(Self::Archived),
            "abandoned" => Some(Self::Abandoned),
            _ => None,
        }
    }

    pub(crate) fn is_established(self) -> bool {
        matches!(self, Self::Baselined | Self::Current)
    }

    pub(crate) fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived | Self::Abandoned)
    }

    pub(crate) fn requires_successor(self) -> bool {
        self == Self::Superseded
    }
}

impl ReferenceRelation {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Body => "body",
            Self::Library => "library",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GovernanceEdgeKind {
    SemanticReference,
    PinnedReference,
    Formalizes,
    Realizes,
    Projects,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceLocation {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentAnchor {
    pub algorithm: &'static str,
    pub digest: String,
    #[serde(skip_serializing_if = "ContentAnchorScope::is_file")]
    pub scope: ContentAnchorScope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<SnapshotProvenance>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SnapshotProvenance {
    pub manifest: String,
    pub repository: String,
    pub commit: String,
    pub path: String,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentAnchorScope {
    Block,
    #[default]
    File,
    Commit,
}

impl ContentAnchorScope {
    pub(crate) fn is_file(&self) -> bool {
        *self == Self::File
    }

    /// Returns whether this scope satisfies a configured minimum pin scope.
    ///
    /// Scope strength is a governance policy, not declaration order: a commit
    /// identifies a whole-file revision, while a block isolates the narrowest
    /// reviewed content boundary.
    pub(crate) fn meets_minimum(self, minimum: Self) -> bool {
        self.strength() >= minimum.strength()
    }

    pub(crate) fn covers_whole_file(self) -> bool {
        matches!(self, Self::File | Self::Commit)
    }

    fn strength(self) -> u8 {
        match self {
            Self::File => 0,
            Self::Commit => 1,
            Self::Block => 2,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleProvenance {
    pub source_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status: Option<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceEdge {
    pub source: String,
    pub target: String,
    pub relation: GovernanceEdgeKind,
    pub source_location: GovernanceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_anchor: Option<ContentAnchor>,
    pub lifecycle: LifecycleProvenance,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceNode {
    pub identity: String,
    pub refinement_level: RefinementLevel,
    pub reference_relation: ReferenceRelation,
    pub lifecycle_status: String,
    pub location: GovernanceLocation,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceGraphMetrics {
    pub nodes: usize,
    pub edges: usize,
    pub resolved_edges: usize,
    pub unresolved_edges: usize,
    pub isolated_nodes: usize,
    pub relation_counts: BTreeMap<GovernanceEdgeKind, usize>,
    pub fan_in: BTreeMap<String, usize>,
    pub fan_out: BTreeMap<String, usize>,
    pub cycle_groups: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceGraph {
    pub nodes: Vec<GovernanceNode>,
    pub edges: Vec<GovernanceEdge>,
    pub metrics: GovernanceGraphMetrics,
    pub transitive_impact: BTreeMap<String, Vec<String>>,
    pub authority_migrations: BTreeMap<String, String>,
}

impl GovernanceGraph {
    pub fn new(mut nodes: Vec<GovernanceNode>, mut edges: Vec<GovernanceEdge>) -> Self {
        nodes.sort();
        nodes.dedup_by(|left, right| left.identity == right.identity);
        edges.sort();
        edges.dedup();

        let identities = nodes
            .iter()
            .map(|node| node.identity.as_str())
            .collect::<BTreeSet<_>>();
        let mut relation_counts = BTreeMap::new();
        let mut resolved_edges = 0;
        for edge in &edges {
            *relation_counts.entry(edge.relation).or_default() += 1;
            if identities.contains(edge.target.as_str()) {
                resolved_edges += 1;
            }
        }
        let metrics = GovernanceGraphMetrics {
            nodes: nodes.len(),
            edges: edges.len(),
            resolved_edges,
            unresolved_edges: edges.len().saturating_sub(resolved_edges),
            isolated_nodes: 0,
            relation_counts,
            ..GovernanceGraphMetrics::default()
        };
        let topology = analyze_topology(&nodes, &edges);
        let transitive_impact = analyze_transitive_impact(&nodes, &edges);
        let metrics = GovernanceGraphMetrics {
            fan_in: topology.fan_in,
            fan_out: topology.fan_out,
            cycle_groups: topology.cycle_groups,
            isolated_nodes: topology.isolated_nodes,
            ..metrics
        };
        Self {
            nodes,
            edges,
            metrics,
            transitive_impact,
            authority_migrations: BTreeMap::new(),
        }
    }

    pub fn node(&self, identity: &str) -> Option<&GovernanceNode> {
        self.nodes.iter().find(|node| node.identity == identity)
    }

    pub fn edges_from(
        &self,
        source: &str,
        relation: GovernanceEdgeKind,
    ) -> impl Iterator<Item = &GovernanceEdge> {
        self.edges
            .iter()
            .filter(move |edge| edge.source == source && edge.relation == relation)
    }

    pub fn edges_to(
        &self,
        target: &str,
        relation: GovernanceEdgeKind,
    ) -> impl Iterator<Item = &GovernanceEdge> {
        self.edges
            .iter()
            .filter(move |edge| edge.target == target && edge.relation == relation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(identity: &str) -> GovernanceNode {
        GovernanceNode {
            identity: identity.to_owned(),
            refinement_level: RefinementLevel::Intent,
            reference_relation: ReferenceRelation::Body,
            lifecycle_status: "current".to_owned(),
            location: GovernanceLocation {
                path: format!("{identity}.yml"),
                line: None,
            },
        }
    }

    fn edge(source: &str, target: &str) -> GovernanceEdge {
        GovernanceEdge {
            source: source.to_owned(),
            target: target.to_owned(),
            relation: GovernanceEdgeKind::Formalizes,
            source_location: GovernanceLocation {
                path: format!("{source}.yml"),
                line: None,
            },
            selector: None,
            content_anchor: None,
            lifecycle: LifecycleProvenance {
                source_status: "current".to_owned(),
                target_status: Some("current".to_owned()),
            },
        }
    }

    #[test]
    fn graph_is_ordered_deduplicated_and_measured() {
        let graph = GovernanceGraph::new(
            vec![node("B"), node("A"), node("A")],
            vec![edge("A", "B"), edge("A", "B"), edge("B", "MISSING")],
        );

        assert_eq!(
            graph
                .nodes
                .iter()
                .map(|node| node.identity.as_str())
                .collect::<Vec<_>>(),
            vec!["A", "B"]
        );
        assert_eq!(graph.metrics.nodes, 2);
        assert_eq!(graph.metrics.edges, 2);
        assert_eq!(graph.metrics.resolved_edges, 1);
        assert_eq!(graph.metrics.unresolved_edges, 1);
        assert_eq!(graph.metrics.isolated_nodes, 0);
        assert_eq!(graph.metrics.fan_out["A"], 1);
        assert_eq!(graph.metrics.fan_in["B"], 1);
        assert!(graph.metrics.cycle_groups.is_empty());
    }

    #[test]
    fn graph_reports_deterministic_reverse_transitive_impact() {
        let graph = GovernanceGraph::new(
            vec![node("A"), node("B"), node("C"), node("D")],
            vec![edge("A", "B"), edge("B", "C"), edge("D", "C")],
        );

        assert_eq!(graph.transitive_impact["B"], vec!["A"]);
        assert_eq!(graph.transitive_impact["C"], vec!["A", "B", "D"]);
        assert!(!graph.transitive_impact.contains_key("A"));

        let cyclic = GovernanceGraph::new(
            vec![node("A"), node("B"), node("C")],
            vec![edge("A", "B"), edge("B", "A"), edge("C", "A")],
        );
        assert_eq!(cyclic.transitive_impact["A"], vec!["B", "C"]);
        assert_eq!(cyclic.transitive_impact["B"], vec!["A", "C"]);
    }

    #[test]
    fn graph_reports_deterministic_cycle_groups_and_distinct_neighbor_fan() {
        let graph = GovernanceGraph::new(
            vec![node("C"), node("B"), node("A")],
            vec![
                edge("A", "B"),
                edge("A", "B"),
                edge("B", "A"),
                edge("C", "C"),
            ],
        );

        assert_eq!(graph.metrics.fan_out["A"], 1);
        assert_eq!(graph.metrics.fan_in["B"], 1);
        assert_eq!(
            graph.metrics.cycle_groups,
            vec![vec!["A".to_owned(), "B".to_owned()], vec!["C".to_owned()]]
        );
    }

    #[test]
    fn lifecycle_policy_centralizes_valid_established_and_terminal_states() {
        let valid = [
            "draft",
            "review",
            "proposed",
            "baselined",
            "current",
            "superseded",
            "archived",
            "abandoned",
        ];
        assert!(
            valid
                .into_iter()
                .all(|value| LifecycleStatus::parse(value).is_some())
        );
        assert!(LifecycleStatus::parse("unknown").is_none());
        assert!(LifecycleStatus::Baselined.is_established());
        assert!(LifecycleStatus::Current.is_established());
        assert!(LifecycleStatus::Superseded.is_terminal());
        assert!(LifecycleStatus::Archived.is_terminal());
        assert!(LifecycleStatus::Abandoned.is_terminal());
        assert!(LifecycleStatus::Superseded.requires_successor());
    }
}
