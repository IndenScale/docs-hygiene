# Governance Graph

Docs Hygiene builds a deterministic graph from configured manifests and
semantic Wiki Links. It validates explicit structure; it does not infer
business meaning from prose or repository location.

## Asset Sources

```yaml
governance:
  manifests:
    - docs/engineering/ul/manifest.yml
    - docs/engineering/prd/prd-001/manifest.yml
```

Localized documents represent the same asset and are not separate declarations.
Issue and Artifact relations enter through adapters or Issue evidence.

## Asset Contract

Every declaration provides a stable `id`, `referenceRelation`, and lifecycle
`status`. UL terms use `library`; PRD packages use `body`. Removed fields such
as `refinementLevel`, `formalizes`, `realizes`, and `projects` are rejected.

```yaml
id: PRD-001
referenceRelation: body
status: baselined
```

Document-level versions, manifest-level semantic references, and leaf-level
source metadata are rejected. Git records history; governed content records
semantic dependencies.

## Recursive Package Trees

UL Library and PRD Body directories declare non-empty direct `members`. Child
domains own a `manifest.yml` and recursively enumerate direct children.
Markdown leaves carry stable `id` and lifecycle `status`. Localized trees
preserve canonical path, identity, membership, and reference signatures.
Issues and Artifacts do not require mirrored package trees.

## Semantic References

A governed PRD Body uses Wiki Links to UL Library identities. Issue adapters may
expose `addresses`, `dependsOn`, and `evidencedBy` relations without prescribing
where Issues or Artifacts live.

- `[[DH-LIBRARY]]` resolves a semantic identity.
- `[[DH-LIBRARY|Library]]` adds a display label.
- `[[DH-LIBRARY#library]]` selects a canonical heading.
- `[[DH-LIBRARY#library@sha256:<hash>|Library]]` also pins reviewed content.

Canonical and localized packages preserve target, selector, and anchor parity.

## Normalized Edge Record

Semantic and pinned references normalize to ordered edge records containing
source and target identity, relation, source location, optional selector and
anchor, lifecycle provenance, endpoint expectation, candidates, and an explicit
`resolved`, `unresolved`, `ambiguous`, or `incompatible` result. Markdown Links
remain navigation-only unless policy assigns semantic meaning.

All syntaxes first enter the versioned `docs-hygiene.reference-occurrence.v1`
IR. Syntax/context policy classifies each occurrence, so adding a collector does
not silently change semantic behavior. See
[FEATURE-020](../zh/issues/features/20_reference-occurrence-ir.md).

## Selectors, Pins, and Snapshots

Heading selectors resolve against canonical ATX headings. Content anchors can
pin file, block, or complete tracked repository state. Critical dependency
policy selects which edges require pins, algorithms, minimum scope, and audit
age. Portable snapshots preserve signed offline file/block evidence without
making Git the semantic authority. See [Critical Dependency Pins](../capabilities/critical-dependency-pins.md)
and [Portable Commit Snapshots](../capabilities/portable-snapshots.md).

## Lifecycle and Impact

Terminal identities cannot remain edge targets. A declared current successor
preserves reference relation and enables deterministic authority migration.
Reverse traversal reports direct and transitive consumers of resolved semantic
edges; unresolved endpoints do not propagate.

## Topology Policy

The graph reports distinct-neighbor Fan-In/Fan-Out, cycle groups, deterministic
communities, cross-community edges, and reverse impact. Explicit thresholds,
cycle policy, community baselines, and audited node/direction exceptions turn
selected results into diagnostics. See [FEATURE-023](../zh/issues/features/23_graph-metrics-and-cycles.md),
[FEATURE-024](../zh/issues/features/24_graph-communities.md), and
[FEATURE-025](../zh/issues/features/25_fan-budgets-and-exceptions.md).

## Boundaries

The current graph validates configured document identities, references,
lifecycle, anchors, package membership, localization parity, impact, and
topology. External Issue coverage and general Artifact discovery remain adapter
boundaries. The graph does not prove natural-language equivalence or product
acceptance.
