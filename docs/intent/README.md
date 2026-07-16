# Intent Refinement Level

This directory contains the Intent Refinement Level. Its Library is the versioned
`ul/` recursive domain tree, with one stable term per Markdown leaf. Its governed
Bodies are recursive PRD Packages under `prd/`.

## Authority

- Every UL domain manifest defines its identity, version, and direct member set.
- Each Markdown leaf under `ul/` defines exactly one stable product term.
- Each PRD manifest enumerates atomic roles, stories, requirements, and acceptance members.
- A PRD pins the UL version and links to the terms it consumes.
- A baselined PRD must be formalized by a Definition Refinement Level Spec.
- Delivered capability remains authoritative in code, configuration, and tests.

## Lifecycle

Intent Bodies use `draft` → `review` → `baselined` → `superseded` → `archived`.
An abandoned proposal uses `abandoned` and does not become a normative baseline.

## Assets

- [Ubiquitous Language directory](ul/)
- [PRD-001 Three-Dimensional Contract Governance](prd/prd-001/index.md)
- [PRD-002 Governed Semantic Links and Editor Navigation](prd/prd-002/index.md) — Backlog
