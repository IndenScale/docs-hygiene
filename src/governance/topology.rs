use std::collections::{BTreeMap, BTreeSet};

use super::{
    GovernanceBoundaryEdge, GovernanceCommunity, GovernanceEdge, GovernanceNode,
    ReferenceResolutionOutcome,
};

pub(super) struct TopologyAnalysis {
    pub fan_in: BTreeMap<String, usize>,
    pub fan_out: BTreeMap<String, usize>,
    pub cycle_groups: Vec<Vec<String>>,
    pub isolated_nodes: usize,
    pub communities: Vec<GovernanceCommunity>,
    pub cross_community_edges: Vec<GovernanceBoundaryEdge>,
}

pub(super) fn analyze_topology(
    nodes: &[GovernanceNode],
    edges: &[GovernanceEdge],
) -> TopologyAnalysis {
    let identities = nodes
        .iter()
        .map(|node| node.identity.clone())
        .collect::<BTreeSet<_>>();
    let mut outgoing = identities
        .iter()
        .map(|identity| (identity.clone(), BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    let mut incoming = outgoing.clone();
    for edge in edges {
        if edge.resolution.outcome != ReferenceResolutionOutcome::Resolved
            || !identities.contains(&edge.source)
            || !identities.contains(&edge.target)
        {
            continue;
        }
        outgoing
            .get_mut(&edge.source)
            .expect("source identity was checked")
            .insert(edge.target.clone());
        incoming
            .get_mut(&edge.target)
            .expect("target identity was checked")
            .insert(edge.source.clone());
    }
    let fan_in = incoming
        .iter()
        .map(|(identity, neighbors)| (identity.clone(), neighbors.len()))
        .collect();
    let fan_out = outgoing
        .iter()
        .map(|(identity, neighbors)| (identity.clone(), neighbors.len()))
        .collect();
    let isolated_nodes = identities
        .iter()
        .filter(|identity| incoming[*identity].is_empty() && outgoing[*identity].is_empty())
        .count();
    let cycle_groups = strongly_connected_cycles(&outgoing);
    let (communities, cross_community_edges) = bridge_connected_communities(&outgoing, edges);
    TopologyAnalysis {
        fan_in,
        fan_out,
        cycle_groups,
        isolated_nodes,
        communities,
        cross_community_edges,
    }
}

fn bridge_connected_communities(
    outgoing: &BTreeMap<String, BTreeSet<String>>,
    edges: &[GovernanceEdge],
) -> (Vec<GovernanceCommunity>, Vec<GovernanceBoundaryEdge>) {
    let mut undirected = outgoing
        .keys()
        .map(|identity| (identity.clone(), BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for edge in edges {
        if edge.resolution.outcome != ReferenceResolutionOutcome::Resolved
            || edge.source == edge.target
            || !undirected.contains_key(&edge.source)
            || !undirected.contains_key(&edge.target)
        {
            continue;
        }
        undirected
            .get_mut(&edge.source)
            .expect("resolved source is indexed")
            .insert(edge.target.clone());
        undirected
            .get_mut(&edge.target)
            .expect("resolved target is indexed")
            .insert(edge.source.clone());
    }

    let bridges = undirected_bridges(&undirected);
    let mut visited = BTreeSet::new();
    let mut communities = Vec::new();
    let mut membership = BTreeMap::new();
    for identity in undirected.keys() {
        if visited.contains(identity) {
            continue;
        }
        let mut pending = BTreeSet::from([identity.clone()]);
        let mut members = Vec::new();
        while let Some(member) = pending.pop_first() {
            if !visited.insert(member.clone()) {
                continue;
            }
            members.push(member.clone());
            for neighbor in &undirected[&member] {
                if !bridges.contains(&ordered_pair(&member, neighbor)) {
                    pending.insert(neighbor.clone());
                }
            }
        }
        members.sort();
        let id = format!("community:{}", members[0]);
        for member in &members {
            membership.insert(member.clone(), id.clone());
        }
        communities.push(GovernanceCommunity { id, members });
    }
    communities.sort();

    let cross_community_edges = edges
        .iter()
        .filter(|edge| edge.resolution.outcome == ReferenceResolutionOutcome::Resolved)
        .filter(|edge| {
            membership
                .get(&edge.source)
                .zip(membership.get(&edge.target))
                .is_some_and(|(source, target)| source != target)
        })
        .map(|edge| GovernanceBoundaryEdge {
            source: edge.source.clone(),
            target: edge.target.clone(),
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    (communities, cross_community_edges)
}

fn undirected_bridges(
    neighbors: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<(String, String)> {
    let mut search = BridgeSearch {
        neighbors,
        next_index: 0,
        discovery: BTreeMap::new(),
        low: BTreeMap::new(),
        bridges: BTreeSet::new(),
    };
    for identity in neighbors.keys() {
        if !search.discovery.contains_key(identity) {
            search.visit(identity, None);
        }
    }
    search.bridges
}

fn ordered_pair(left: &str, right: &str) -> (String, String) {
    if left <= right {
        (left.to_owned(), right.to_owned())
    } else {
        (right.to_owned(), left.to_owned())
    }
}

struct BridgeSearch<'a> {
    neighbors: &'a BTreeMap<String, BTreeSet<String>>,
    next_index: usize,
    discovery: BTreeMap<String, usize>,
    low: BTreeMap<String, usize>,
    bridges: BTreeSet<(String, String)>,
}

impl BridgeSearch<'_> {
    fn visit(&mut self, identity: &str, parent: Option<&str>) {
        let index = self.next_index;
        self.next_index += 1;
        self.discovery.insert(identity.to_owned(), index);
        self.low.insert(identity.to_owned(), index);
        for neighbor in &self.neighbors[identity] {
            if Some(neighbor.as_str()) == parent {
                continue;
            }
            if !self.discovery.contains_key(neighbor) {
                self.visit(neighbor, Some(identity));
                let neighbor_low = self.low[neighbor];
                self.low
                    .entry(identity.to_owned())
                    .and_modify(|low| *low = (*low).min(neighbor_low));
                if neighbor_low > index {
                    self.bridges.insert(ordered_pair(identity, neighbor));
                }
            } else {
                let neighbor_discovery = self.discovery[neighbor];
                self.low
                    .entry(identity.to_owned())
                    .and_modify(|low| *low = (*low).min(neighbor_discovery));
            }
        }
    }
}

fn strongly_connected_cycles(outgoing: &BTreeMap<String, BTreeSet<String>>) -> Vec<Vec<String>> {
    let mut search = StronglyConnectedSearch::new(outgoing);
    for identity in outgoing.keys() {
        if !search.indices.contains_key(identity) {
            search.visit(identity);
        }
    }
    search.cycles.sort();
    search.cycles
}

struct StronglyConnectedSearch<'a> {
    outgoing: &'a BTreeMap<String, BTreeSet<String>>,
    next_index: usize,
    indices: BTreeMap<String, usize>,
    low_links: BTreeMap<String, usize>,
    stack: Vec<String>,
    on_stack: BTreeSet<String>,
    cycles: Vec<Vec<String>>,
}

impl<'a> StronglyConnectedSearch<'a> {
    fn new(outgoing: &'a BTreeMap<String, BTreeSet<String>>) -> Self {
        Self {
            outgoing,
            next_index: 0,
            indices: BTreeMap::new(),
            low_links: BTreeMap::new(),
            stack: Vec::new(),
            on_stack: BTreeSet::new(),
            cycles: Vec::new(),
        }
    }

    fn visit(&mut self, identity: &str) {
        let index = self.next_index;
        self.next_index += 1;
        self.indices.insert(identity.to_owned(), index);
        self.low_links.insert(identity.to_owned(), index);
        self.stack.push(identity.to_owned());
        self.on_stack.insert(identity.to_owned());

        for target in &self.outgoing[identity] {
            if !self.indices.contains_key(target) {
                self.visit(target);
                let target_low = self.low_links[target];
                self.low_links
                    .entry(identity.to_owned())
                    .and_modify(|low| *low = (*low).min(target_low));
            } else if self.on_stack.contains(target) {
                let target_index = self.indices[target];
                self.low_links
                    .entry(identity.to_owned())
                    .and_modify(|low| *low = (*low).min(target_index));
            }
        }

        if self.low_links[identity] != self.indices[identity] {
            return;
        }
        let mut component = Vec::new();
        loop {
            let member = self.stack.pop().expect("root must remain on Tarjan stack");
            self.on_stack.remove(&member);
            let complete = member == identity;
            component.push(member);
            if complete {
                break;
            }
        }
        component.sort();
        let self_loop =
            component.len() == 1 && self.outgoing[&component[0]].contains(&component[0]);
        if component.len() > 1 || self_loop {
            self.cycles.push(component);
        }
    }
}
