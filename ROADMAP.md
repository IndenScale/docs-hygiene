# Roadmap

This roadmap distinguishes current capability from planned work and backlog
product requirements. Code, configuration, tests, and generated diagnostics
remain the authority for delivered behavior.

## Current Baseline

- Governed documents use standard Markdown links for project-root navigation and
  semantic Wiki Links for Library identities.
- Horizontal references are derived from Body content; vertical derivation and
  projection remain explicit Manifest relationships.
- Document contracts, concept foreign keys, project structure, and the
  existing CLI diagnostics are the currently delivered policy surface.
- Project-root-local Markdown Link and image targets are checked for existence;
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
- Progressive rule activation derives nine stable rule-family decisions from
  deterministic project facts, supports per-rule auto/required/disabled policy,
  and exposes evidence through versioned `explain-rules` text and JSON output.
- The first PRD-004 implementation slice centralizes compatibility-family
  metadata in an ordered registry without changing current diagnostics or modes.
- The second PRD-004 slice registers atomic invariants and emits the versioned
  multidimensional `profile` report with optional target gating.
- The third PRD-004 slice normalizes semantic references, pins, derivations, and
  projections into ordered governance edges and reports basic graph metrics.
- The fourth PRD-004 slice reports distinct-neighbor Fan-In/Fan-Out and cycle
  groups, with opt-in thresholds under the independent topology rule family.
- The fifth PRD-004 slice adds reusable document-contract templates,
  deterministic merge and validation, complete binding evidence, and repository
  migration from repeated inline policy.
- The sixth PRD-004 slice adds template revision compatibility windows, exact
  profile pins, read-only migration planning, and atomic compatible migration;
  repository structure governance now reaches governed maturity.
- The seventh PRD-004 slice resolves lowercase heading selectors against
  canonical ATX headings, preserves localized signatures, and retains selector
  evidence on normalized governance edges.
- The eighth PRD-004 slice separates declaration syntax from edge semantics with
  a versioned reference-occurrence IR, three collectors, explicit policy, and a
  syntax-independent normalizer while preserving the public edge contract.
- The ninth PRD-004 slice adds deterministic file and ATX-block SHA-256 anchors,
  frontmatter multi-anchor declarations, per-item diagnostics, and explicitly
  opt-in local Git commit verification without changing existing inline pins.
- The tenth PRD-004 slice reports deterministic reverse transitive impact across
  every resolved semantic edge kind, including deduplication and cycle-safe
  propagation, without coupling analysis to budgets or notification policy.
- The eleventh PRD-004 slice enforces lifecycle status obligations across assets
  and package identities, validates `supersededBy` successors, rejects stale
  terminal targets, and reports ordered authority-migration evidence.
- Subsequent delivered slices add kind-scoped slug identity, typed Document Kind
  schemas and scaffolding, explicit Library claim governance, critical Pin policy,
  portable offline commit snapshots, audited supernode exceptions, and degree trends.
- The current final slice adds offline principal identities, established-identity
  Owner/review/two-person obligations, deterministic coverage and bus-factor
  evidence, and atomic audited review resets, completing the current
  [PRD-004](docs/intent/prd/prd-004/index.md) / [SPEC-003 C-019](docs/definition/spec/spec-003/constraints/ownership-review.md)
  baseline.

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
- Semantic references, pins, derivations, and projections may share dependency
  governance without turning navigational Markdown Links into semantic edges.
- Document versions are not part of the governance identity model. Git records
  history, while optional content hashes anchor review-sensitive dependencies.
- Commit anchors are opt-in physical audit evidence: they compare canonical
  target bytes with a local Git commit but never replace stable semantic IDs or
  make Git history the governance authority.
- LSP or editor integration is not required by the current delivery baseline.
- Work items and feature tickets live in the repository as governed documents;
  external issue trackers are not part of the project baseline.
