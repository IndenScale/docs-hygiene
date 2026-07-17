---
id: SPEC-003-C-018
status: baselined
---

# C-018 Audited Supernode Exceptions

A supernode exception binds one unique exception identity to one exact governed
node and `fanIn` or `fanOut` direction. It declares an exception budget above
the corresponding global limit, non-empty rationale, owner and approver, an
expiry date, and ordered dated degree history. No wildcard matcher exists.

An exception applies only while metadata is valid, the node exceeds the global
limit, current degree does not exceed the exception budget, and at least one
non-future history observation exists. It relaxes only that node/direction.
Expiry, invalid metadata, missing history, or exception-budget excess restores
the ordinary threshold failure. A node back within the global limit makes the
declaration idle and produces cleanup evidence.

`topologyExceptions` reports current degree, both budgets, signed remaining
capacity, latest observation, trend delta, lifecycle status, audit metadata,
and reverse transitive impact. Stable `DH_TOPOLOGY_003` through
`DH_TOPOLOGY_005` distinguish invalid/expired, idle, and history failures.

An applied exception is non-blocking but the profile marks
`topology.thresholds` as `excepted`, never Passed. Legacy suppression remains
unverified evidence under [C-005](exceptions.md). Budget, audited-exception, and
trend mechanisms provide delivered governed-level invariant implementations;
an active exception still prevents the lower threshold invariant from proving
controlled maturity.
