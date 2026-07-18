use std::collections::BTreeMap;

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
    Repo,
}

impl ContentAnchorScope {
    pub(crate) fn is_file(&self) -> bool {
        *self == Self::File
    }

    /// Returns whether this scope satisfies a configured minimum pin scope.
    ///
    /// Scope strength is a governance policy, not declaration order: a repo
    /// anchor proves one complete tracked repository state, while a block
    /// isolates the narrowest reviewed content boundary.
    pub(crate) fn meets_minimum(self, minimum: Self) -> bool {
        self.strength() >= minimum.strength()
    }

    pub(crate) fn covers_whole_file(self) -> bool {
        matches!(self, Self::File | Self::Repo)
    }

    fn strength(self) -> u8 {
        match self {
            Self::File => 0,
            Self::Repo => 1,
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
pub struct ReferenceEndpointExpectation {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub refinement_levels: Vec<RefinementLevel>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reference_relations: Vec<ReferenceRelation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub document_kinds: Vec<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceExpectation {
    pub relation: GovernanceEdgeKind,
    pub endpoint: ReferenceEndpointExpectation,
}

impl ReferenceExpectation {
    pub(crate) fn new(
        relation: GovernanceEdgeKind,
        mut refinement_levels: Vec<RefinementLevel>,
        mut reference_relations: Vec<ReferenceRelation>,
        mut document_kinds: Vec<String>,
    ) -> Self {
        refinement_levels.sort();
        refinement_levels.dedup();
        reference_relations.sort();
        reference_relations.dedup();
        document_kinds.sort();
        document_kinds.dedup();
        Self {
            relation,
            endpoint: ReferenceEndpointExpectation {
                refinement_levels,
                reference_relations,
                document_kinds,
            },
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceEndpoint {
    pub refinement_level: RefinementLevel,
    pub reference_relation: ReferenceRelation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_kind: Option<String>,
    pub lifecycle_status: String,
    pub location: GovernanceLocation,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReferenceResolutionOutcome {
    Resolved,
    Unresolved,
    Ambiguous,
    Incompatible,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReferenceCompatibilityIssue {
    MissingTarget,
    AmbiguousTarget,
    RefinementLevel,
    ReferenceRelation,
    DocumentKind,
    Lifecycle,
    Selector,
    Anchor,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceResolution {
    pub outcome: ReferenceResolutionOutcome,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<ReferenceEndpoint>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub incompatibilities: Vec<ReferenceCompatibilityIssue>,
}

impl ReferenceResolution {
    pub(crate) fn unresolved() -> Self {
        Self {
            outcome: ReferenceResolutionOutcome::Unresolved,
            endpoints: Vec::new(),
            incompatibilities: vec![ReferenceCompatibilityIssue::MissingTarget],
        }
    }

    pub(crate) fn add_incompatibility(&mut self, issue: ReferenceCompatibilityIssue) {
        if !self.incompatibilities.contains(&issue) {
            self.incompatibilities.push(issue);
            self.incompatibilities.sort();
        }
        if self.outcome == ReferenceResolutionOutcome::Resolved
            && !matches!(
                issue,
                ReferenceCompatibilityIssue::Selector | ReferenceCompatibilityIssue::Anchor
            )
        {
            self.outcome = ReferenceResolutionOutcome::Incompatible;
        }
    }
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
    pub expectation: ReferenceExpectation,
    pub resolution: ReferenceResolution,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceNode {
    pub identity: String,
    pub refinement_level: RefinementLevel,
    pub reference_relation: ReferenceRelation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_kind: Option<String>,
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
    pub ambiguous_edges: usize,
    pub incompatible_edges: usize,
    pub isolated_nodes: usize,
    pub relation_counts: BTreeMap<GovernanceEdgeKind, usize>,
    pub fan_in: BTreeMap<String, usize>,
    pub fan_out: BTreeMap<String, usize>,
    pub cycle_groups: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceCommunity {
    pub id: String,
    pub members: Vec<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceBoundaryEdge {
    pub source: String,
    pub target: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceCommunityChange {
    pub identity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_community: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_community: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceGraph {
    pub nodes: Vec<GovernanceNode>,
    pub edges: Vec<GovernanceEdge>,
    pub metrics: GovernanceGraphMetrics,
    pub transitive_impact: BTreeMap<String, Vec<String>>,
    pub authority_migrations: BTreeMap<String, String>,
    pub communities: Vec<GovernanceCommunity>,
    pub cross_community_edges: Vec<GovernanceBoundaryEdge>,
    pub community_changes: Vec<GovernanceCommunityChange>,
}

impl GovernanceGraph {
    pub fn new(mut nodes: Vec<GovernanceNode>, mut edges: Vec<GovernanceEdge>) -> Self {
        nodes.sort();
        nodes.dedup();
        let candidates = nodes.iter().fold(
            BTreeMap::<String, Vec<ReferenceEndpoint>>::new(),
            |mut candidates, node| {
                candidates
                    .entry(node.identity.clone())
                    .or_default()
                    .push(ReferenceEndpoint::from(node));
                candidates
            },
        );
        for edge in &mut edges {
            let supplemental = edge
                .resolution
                .incompatibilities
                .iter()
                .copied()
                .filter(|issue| {
                    matches!(
                        issue,
                        ReferenceCompatibilityIssue::Selector | ReferenceCompatibilityIssue::Anchor
                    )
                })
                .collect::<Vec<_>>();
            edge.resolution = resolve_reference(&edge.expectation, candidates.get(&edge.target));
            for issue in supplemental {
                edge.resolution.add_incompatibility(issue);
            }
            edge.lifecycle.target_status = edge
                .resolution
                .endpoints
                .first()
                .map(|endpoint| endpoint.lifecycle_status.clone());
        }
        edges.sort();
        edges.dedup();
        nodes.dedup_by(|left, right| left.identity == right.identity);

        let mut relation_counts = BTreeMap::new();
        for edge in &edges {
            *relation_counts.entry(edge.relation).or_default() += 1;
        }
        let resolution_count = |outcome| {
            edges
                .iter()
                .filter(|edge| edge.resolution.outcome == outcome)
                .count()
        };
        let metrics = GovernanceGraphMetrics {
            nodes: nodes.len(),
            edges: edges.len(),
            resolved_edges: resolution_count(ReferenceResolutionOutcome::Resolved),
            unresolved_edges: resolution_count(ReferenceResolutionOutcome::Unresolved),
            ambiguous_edges: resolution_count(ReferenceResolutionOutcome::Ambiguous),
            incompatible_edges: resolution_count(ReferenceResolutionOutcome::Incompatible),
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
            communities: topology.communities,
            cross_community_edges: topology.cross_community_edges,
            community_changes: Vec::new(),
        }
    }

    pub(crate) fn compare_community_baseline(&mut self, baseline: &BTreeMap<String, String>) {
        if baseline.is_empty() {
            self.community_changes.clear();
            return;
        }
        let actual = self
            .communities
            .iter()
            .flat_map(|community| {
                community
                    .members
                    .iter()
                    .map(|member| (member.clone(), community.id.clone()))
            })
            .collect::<BTreeMap<_, _>>();
        self.community_changes = baseline
            .keys()
            .chain(actual.keys())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .filter_map(|identity| {
                let expected = baseline.get(identity);
                let observed = actual.get(identity);
                (expected != observed).then(|| GovernanceCommunityChange {
                    identity: identity.clone(),
                    expected_community: expected.cloned(),
                    actual_community: observed.cloned(),
                })
            })
            .collect();
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

impl From<&GovernanceNode> for ReferenceEndpoint {
    fn from(node: &GovernanceNode) -> Self {
        Self {
            refinement_level: node.refinement_level,
            reference_relation: node.reference_relation,
            document_kind: node.document_kind.clone(),
            lifecycle_status: node.lifecycle_status.clone(),
            location: node.location.clone(),
        }
    }
}

pub(crate) fn resolve_reference(
    expectation: &ReferenceExpectation,
    candidates: Option<&Vec<ReferenceEndpoint>>,
) -> ReferenceResolution {
    let Some(candidates) = candidates.filter(|candidates| !candidates.is_empty()) else {
        return ReferenceResolution::unresolved();
    };
    if candidates.len() > 1 {
        return ReferenceResolution {
            outcome: ReferenceResolutionOutcome::Ambiguous,
            endpoints: candidates.clone(),
            incompatibilities: vec![ReferenceCompatibilityIssue::AmbiguousTarget],
        };
    }
    let endpoint = &candidates[0];
    let mut incompatibilities = Vec::new();
    if !expectation.endpoint.refinement_levels.is_empty()
        && !expectation
            .endpoint
            .refinement_levels
            .contains(&endpoint.refinement_level)
    {
        incompatibilities.push(ReferenceCompatibilityIssue::RefinementLevel);
    }
    if !expectation.endpoint.reference_relations.is_empty()
        && !expectation
            .endpoint
            .reference_relations
            .contains(&endpoint.reference_relation)
    {
        incompatibilities.push(ReferenceCompatibilityIssue::ReferenceRelation);
    }
    if !expectation.endpoint.document_kinds.is_empty()
        && endpoint
            .document_kind
            .as_ref()
            .is_none_or(|kind| !expectation.endpoint.document_kinds.contains(kind))
    {
        incompatibilities.push(ReferenceCompatibilityIssue::DocumentKind);
    }
    if LifecycleStatus::parse(&endpoint.lifecycle_status).is_some_and(LifecycleStatus::is_terminal)
    {
        incompatibilities.push(ReferenceCompatibilityIssue::Lifecycle);
    }
    ReferenceResolution {
        outcome: if incompatibilities.is_empty() {
            ReferenceResolutionOutcome::Resolved
        } else {
            ReferenceResolutionOutcome::Incompatible
        },
        endpoints: candidates.clone(),
        incompatibilities,
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
            document_kind: None,
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
            expectation: ReferenceExpectation::new(
                GovernanceEdgeKind::Formalizes,
                vec![RefinementLevel::Intent],
                vec![ReferenceRelation::Body],
                Vec::new(),
            ),
            resolution: ReferenceResolution::unresolved(),
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

    #[test]
    fn resolution_classifies_ambiguous_and_incompatible_endpoints_before_topology() {
        let mut duplicate = node("B");
        duplicate.location.path = "other-B.yml".to_owned();
        let ambiguous =
            GovernanceGraph::new(vec![node("A"), node("B"), duplicate], vec![edge("A", "B")]);
        assert_eq!(ambiguous.metrics.ambiguous_edges, 1);
        assert_eq!(
            ambiguous.edges[0].resolution.outcome,
            ReferenceResolutionOutcome::Ambiguous
        );
        assert_eq!(ambiguous.metrics.fan_out["A"], 0);

        let mut wrong_kind = node("B");
        wrong_kind.document_kind = Some("article".to_owned());
        let mut typed_edge = edge("A", "B");
        typed_edge.expectation.endpoint.document_kinds = vec!["term".to_owned()];
        let incompatible = GovernanceGraph::new(vec![node("A"), wrong_kind], vec![typed_edge]);
        assert_eq!(incompatible.metrics.incompatible_edges, 1);
        assert_eq!(
            incompatible.edges[0].resolution.incompatibilities,
            vec![ReferenceCompatibilityIssue::DocumentKind]
        );
        assert_eq!(incompatible.metrics.fan_out["A"], 0);
        assert!(!incompatible.transitive_impact.contains_key("B"));
    }

    #[test]
    fn graph_discovers_deterministic_bridge_communities_and_boundary_changes() {
        let mut parallel_relation = edge("A", "B");
        parallel_relation.relation = GovernanceEdgeKind::PinnedReference;
        parallel_relation.expectation.relation = GovernanceEdgeKind::PinnedReference;
        let edges = vec![
            edge("A", "B"),
            parallel_relation,
            edge("B", "C"),
            edge("C", "A"),
            edge("C", "D"),
            edge("D", "E"),
            edge("E", "F"),
            edge("F", "D"),
            edge("A", "B"),
        ];
        let mut graph = GovernanceGraph::new(
            ["A", "B", "C", "D", "E", "F"]
                .into_iter()
                .map(node)
                .collect(),
            edges,
        );
        assert_eq!(
            graph.communities,
            vec![
                GovernanceCommunity {
                    id: "community:A".to_owned(),
                    members: vec!["A".to_owned(), "B".to_owned(), "C".to_owned()],
                },
                GovernanceCommunity {
                    id: "community:D".to_owned(),
                    members: vec!["D".to_owned(), "E".to_owned(), "F".to_owned()],
                },
            ]
        );
        assert_eq!(
            graph.cross_community_edges,
            vec![GovernanceBoundaryEdge {
                source: "C".to_owned(),
                target: "D".to_owned(),
            }]
        );
        let mut baseline = ["A", "B", "C"]
            .into_iter()
            .map(|identity| (identity.to_owned(), "community:A".to_owned()))
            .chain(
                ["D", "E", "F"]
                    .into_iter()
                    .map(|identity| (identity.to_owned(), "community:D".to_owned())),
            )
            .collect::<BTreeMap<_, _>>();
        graph.compare_community_baseline(&baseline);
        assert!(graph.community_changes.is_empty());
        baseline.insert("F".to_owned(), "community:A".to_owned());
        graph.compare_community_baseline(&baseline);
        assert_eq!(graph.community_changes.len(), 1);
        assert_eq!(graph.community_changes[0].identity, "F");
    }
}
