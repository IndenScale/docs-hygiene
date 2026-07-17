---
id: SPEC-003-C-014
status: baselined
---

# C-014 Transitive Impact

Every normalized semantic edge points from a dependent source to an authority
target. A change to one governed identity affects every other governed identity
reachable through the reverse of those edges, including indirect dependents.
Semantic references, pinned references, derivations, and projections use the
same propagation algorithm.

The graph report exposes `transitiveImpact` as an ordered map from changed
identity to a sorted, deduplicated list of affected identities. Identities with
an empty impact set are omitted. The changed identity itself is excluded;
visited-set traversal terminates cycles deterministically. Repeated and parallel
edges do not duplicate results, and unresolved edge endpoints do not propagate
impact.

This analysis reports evidence rather than policy. It does not infer file
changes, rank severity, enforce an impact budget, or notify owners. Budget and
exception enforcement remain separate topology and workflow capabilities.
