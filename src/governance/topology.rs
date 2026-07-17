use std::collections::{BTreeMap, BTreeSet};

use super::{GovernanceEdge, GovernanceNode};

pub(super) struct TopologyAnalysis {
    pub fan_in: BTreeMap<String, usize>,
    pub fan_out: BTreeMap<String, usize>,
    pub cycle_groups: Vec<Vec<String>>,
    pub isolated_nodes: usize,
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
        if !identities.contains(&edge.source) || !identities.contains(&edge.target) {
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
    TopologyAnalysis {
        fan_in,
        fan_out,
        cycle_groups,
        isolated_nodes,
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
