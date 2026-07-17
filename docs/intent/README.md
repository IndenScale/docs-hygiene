# Intent Refinement Level

This directory contains the Intent Refinement Level. Its Library is the identity-governed
`ul/` recursive domain tree, with one stable term per Markdown leaf. Its governed
Bodies are recursive PRD Packages under `prd/`.

## Authority

- Every UL domain manifest defines its identity and direct member set.
- Each Markdown leaf under `ul/` defines exactly one stable product term.
- Each PRD manifest enumerates atomic roles, stories, requirements, and acceptance members.
- A PRD uses semantic Wiki Links for the UL terms it consumes.
- A baselined PRD must be formalized by a Definition Refinement Level Spec.
- Delivered capability remains authoritative in code, configuration, and tests.

## Lifecycle

Intent Bodies use `draft` → `review` → `baselined` → `superseded` → `archived`.
An abandoned proposal uses `abandoned` and does not become a normative baseline.

## Assets

- [Ubiquitous Language directory](ul/)
- [PRD-001 Three-Dimensional Contract Governance](prd/prd-001/index.md)
- [PRD-002 Governed Semantic Links and Editor Navigation](prd/prd-002/index.md) — Wiki Links delivered, editor navigation backlog
- [PRD-003 Progressive Rule Activation](prd/prd-003/index.md) — delivered
- [PRD-004 Multidimensional Documentation Governance](prd/prd-004/index.md) — in progress; profile, templates, normalized edges, scoped anchors, transitive impact, lifecycle, authority migration, and topology policy delivered
