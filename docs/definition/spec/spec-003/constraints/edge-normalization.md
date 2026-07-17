---
id: SPEC-003-C-003
status: baselined
---

# C-003 Edge Normalization

Semantic references, pinned references, derivations, and projections normalize
to an ordered edge record containing source identity, target identity, relation
kind, source location, optional selector, optional content anchor, and lifecycle
provenance. Resolution, staleness, impact, and topology consume this record.

Markdown navigation links remain path-integrity inputs and never become semantic
edges without an explicit semantic relation.

Reference declaration surfaces first produce the versioned, syntax-neutral
occurrence IR defined by [C-012](reference-occurrence-ir.md). Explicit policy,
not collector syntax, determines whether an occurrence reaches this edge model.

When present, a heading selector is validated against the canonical target and
retained on the normalized edge according to [C-011](selector-resolution.md).
