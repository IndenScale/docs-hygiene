# Open Engineering

This directory contains the two durable document structures used by open
engineering: the Ubiquitous Language (`ul/`) and Product Requirements (`prd/`).

## Authority

- Every UL domain manifest defines its identity and direct member set.
- Each Markdown leaf under `ul/` defines exactly one stable product term.
- Each PRD manifest enumerates its stable product-intent members.
- A PRD uses semantic Wiki Links for the UL terms it consumes.
- An Issue addresses a PRD and owns change-scoped acceptance, coordination, and delivery evidence.
- Code, tests, configuration, SDK content, commits, and generated outputs are location-independent Artifacts.

## Lifecycle

PRD Bodies use `draft` → `review` → `baselined` → `superseded` → `archived`.
An abandoned proposal uses `abandoned` and does not become a normative baseline.

## Assets

- [Ubiquitous Language directory](ul/)
- [PRD-001 Open Engineering Asset Governance](prd/prd-001/index.md)
- [PRD-002 Governed Semantic Links and Editor Navigation](prd/prd-002/index.md) — Wiki Links delivered, editor navigation backlog
- [PRD-003 Progressive Rule Activation](prd/prd-003/index.md) — delivered
- [PRD-004 Multidimensional Documentation Governance](prd/prd-004/index.md) — delivered and baselined
