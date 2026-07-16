# Roadmap

This roadmap distinguishes current capability from planned work and backlog
product requirements. Code, configuration, tests, and generated diagnostics
remain the authority for delivered behavior.

## Current Baseline

- Governed documents use standard Markdown links for repository navigation and
  semantic Wiki Links for Library identities.
- Horizontal references are derived from Body content; vertical derivation and
  projection remain explicit Manifest relationships.
- Document contracts, concept foreign keys, repository structure, and the
  existing CLI diagnostics are the currently delivered policy surface.
- Repository-local Markdown Link and image targets are checked for existence;
  external URL reachability remains outside the deterministic local baseline.
- Native governance checks validate same-refinement-level Wiki Links, optional
  SHA-256 content anchors, and vertical adjacent-refinement-level Body derivation
  and Library projection.
- UL and Glossary are mandatory recursive Library trees; each domain manifest
  declares direct children and each term has its own Markdown leaf.
- PRD and Spec are recursive directory Body Packages whose manifests enumerate
  atomic roles, stories, requirements, constraints, acceptance, and verification.
- Source files are governed by a 500-line warning and a 1,000-line CI error;
  the core checker is split into responsibility-focused private units and the
  current repository passes without warnings or exceptions.

## Planned

- [PRD-001 Three-Layer Contract Governance](docs/intent/prd/prd-001/index.md)
  will extend the delivered asset graph with item-level requirement coverage,
  symbol-level semantic projection and end-to-end traces from intent to implementation.

## Backlog

- [PRD-002 Governed Semantic Links and Editor Navigation](docs/intent/prd/prd-002/index.md)
  retains preview and IDE navigation as backlog work. Semantic Wiki Link validation
  is part of the current CLI contract.

## Decision Boundaries

- A Markdown link provides path navigation; a Wiki Link establishes a semantic
  identity reference but does not prove natural-language consistency.
- Document versions are not part of the governance identity model. Git records
  history, while optional content hashes anchor review-sensitive dependencies.
- LSP or editor integration is not required by the current delivery baseline.
