---
id: SPEC-003-C-020
status: baselined
---

# C-020 Typed Reference Resolution

Every normalized governance edge retains a serializable `expectation` and
`resolution`. The expectation contains the edge relation and allowed target
refinement levels, reference relations, and optional Document Kinds. The
resolution contains an explicit `resolved`, `unresolved`, `ambiguous`, or
`incompatible` outcome, typed endpoint candidates, locations, lifecycle state,
and ordered incompatibility categories.

One compatibility matrix classifies missing or ambiguous targets, refinement,
reference-relation and Document Kind mismatches, terminal lifecycle targets,
selector failures, and anchor failures. Vertical targets may declare an
expected `documentKind`; frontmatter anchors may declare
`expectedDocumentKind`. Absence means that the relation policy does not impose
a Document Kind, not that the resolved endpoint loses its actual Kind.

Diagnostics, profiles, impact analysis, Fan metrics, cycles, and communities
reuse this record. Unresolved, ambiguous, type-incompatible, and terminal
endpoints do not enter topology or impact. A resolved dependency with stale
selector or anchor evidence remains a dependency for impact, while its ordered
compatibility issues prevent the evidence from proving freshness.

Adding a relation extends the expectation policy rather than duplicating
endpoint comparison inside a collector or checker.
