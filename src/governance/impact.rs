use std::collections::{BTreeMap, BTreeSet};

use super::{GovernanceEdge, GovernanceNode, ReferenceResolutionOutcome};

pub(super) fn analyze_transitive_impact(
    nodes: &[GovernanceNode],
    edges: &[GovernanceEdge],
) -> BTreeMap<String, Vec<String>> {
    let identities = nodes
        .iter()
        .map(|node| node.identity.clone())
        .collect::<BTreeSet<_>>();
    let mut incoming = identities
        .iter()
        .map(|identity| (identity.clone(), BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for edge in edges {
        if edge.resolution.outcome == ReferenceResolutionOutcome::Resolved
            && identities.contains(&edge.source)
            && identities.contains(&edge.target)
        {
            incoming
                .get_mut(&edge.target)
                .expect("resolved target identity was checked")
                .insert(edge.source.clone());
        }
    }

    let mut impact = BTreeMap::new();
    for changed in &identities {
        let mut pending = incoming[changed].clone();
        let mut affected = BTreeSet::new();
        while let Some(identity) = pending.pop_first() {
            if !affected.insert(identity.clone()) {
                continue;
            }
            pending.extend(incoming[&identity].iter().cloned());
        }
        affected.remove(changed);
        if !affected.is_empty() {
            impact.insert(changed.clone(), affected.into_iter().collect());
        }
    }
    impact
}
