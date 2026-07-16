# Roadmap

This roadmap distinguishes current capability from planned work and backlog
product requirements. Code, configuration, tests, and generated diagnostics
remain the authority for delivered behavior.

## Current Baseline

- Governed documents use standard Markdown links for repository navigation and
  editor preview.
- Stable semantic identities and typed governance relationships are declared in
  structured metadata rather than inferred from navigation links.
- Document contracts, concept foreign keys, repository structure, and the
  existing CLI diagnostics are the currently delivered policy surface.
- Native governance manifests validate horizontal same-layer references and
  vertical adjacent-layer Body derivation and Library projection.
- UL and Glossary are mandatory recursive Library trees; each domain manifest
  declares direct children and each term has its own Markdown leaf.
- PRD and Spec are recursive directory Body Packages whose manifests enumerate
  atomic roles, stories, requirements, constraints, acceptance, and verification.

## Planned

- [PRD-001 Three-Layer Contract Governance](docs/intent/prd/prd-001/index.md)
  will extend the delivered asset graph with item-level requirement coverage,
  symbol-level semantic projection and end-to-end traces from intent to implementation.

## Backlog

- [PRD-002 Governed Semantic Links and Editor Navigation](docs/intent/prd/prd-002/index.md)
  records optional semantic-link authoring, preview, and IDE navigation as
  backlog work. It is not part of the current CLI or editor contract.

## Decision Boundaries

- A Markdown link provides navigation; it does not establish a typed governance
  relationship or prove semantic consistency.
- Wiki Link syntax is not a current repository convention or required input.
- LSP or editor integration is not required by the current delivery baseline.
