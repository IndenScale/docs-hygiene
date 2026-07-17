---
id: SPEC-003-C-008
status: proposed
---

# C-008 Topology Policy

Fan-In and Fan-Out count distinct resolved governed neighbor identities. Repeated
references and parallel relation kinds between the same identities do not
inflate degree. Directed cycle groups are ordered strongly connected components;
a self-loop is a one-identity cycle group.

Topology enforcement is opt-in. `maxFanIn`, `maxFanOut`, and `forbidCycles`
activate the independent `governance.topology` family only when explicitly
configured. Graph presence and scale signals alone never create blocking
topology policy.

Topology reaches controlled maturity only when deterministic Fan and cycle
analysis passes and an active explicit topology policy has no visible or
suppressed violation. Suppression remains non-evidence.
