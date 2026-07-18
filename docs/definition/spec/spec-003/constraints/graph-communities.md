---
id: SPEC-003-C-021
status: baselined
---

# C-021 Deterministic Graph Communities

Community discovery consumes only resolved typed edges from C-020. It converts
distinct directed endpoint pairs into an unweighted undirected graph, finds
bridges with deterministic depth-first traversal, removes those bridges, and
reports the remaining connected components as communities. A community ID is
`community:<lexicographically-smallest-member>`. The algorithm uses no random
seed; repeated references and parallel relation kinds do not change membership.

The graph report contains ordered community members and distinct directed edges
that cross detected communities. An optional `communityBaseline` maps identity
to expected community ID and produces ordered change records for changed,
missing, or newly detected members. Discovery and change reporting are
advisory by default.

`enforceCommunityBaseline: true` turns baseline changes into
`DH_TOPOLOGY_006` errors. A baseline without that explicit switch cannot block
CI. Fan, cycle, budget, and supernode policies remain independent.
