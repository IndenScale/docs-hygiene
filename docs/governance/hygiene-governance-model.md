# Documentation Hygiene Governance Model

This document defines the operational model baselined by
[PRD-004](../engineering/prd/prd-004/index.md). It organizes delivered capabilities
without forcing one linear maturity score to represent every concern. Current
executable behavior is defined by configuration, rules, tests, and
[Progressive Rule Activation](progressive-rule-activation.md).

It sits below the [Cognitive Asset Governance Model](../position/02_cognitive_asset_governance_model.md):
identity and SSOT operationalize Semantic Authority; dependency and topology
operationalize Change Propagation; ownership review, activation policy, and
audited updates provide foundations for Human Judgment. First-class Decision
assets are not claimed as shipped by this operational profile.

## Model

The operational hygiene profile has three independent axes:

1. A capability dimension states what governance surface is assessed.
2. A maturity level states how deeply that surface is governed.
3. An execution state states how one applicable rule is enforced now.

Keeping these axes separate avoids treating observe-only execution, dependency
precision, and graph scale as mandatory steps in one six-level ladder.

## Maturity Levels

| Level | Name | Guarantee |
| --- | --- | --- |
| G1 | Basic | Discover inputs and deterministically validate basic integrity. |
| G2 | Controlled | Make policy explicit and change detection repeatable. |
| G3 | Governed | Propagate impact and audit policy, budgets, and exceptions. |

The levels are cumulative within one applicable dimension. Observe-only is an
execution policy, not G0.

## Core Capability Dimensions

| Dimension | G1 Basic | G2 Controlled | G3 Governed |
| --- | --- | --- | --- |
| Structure and contracts | discovery, naming, links | document contracts and reusable templates | template versions and structural migration |
| Identity and SSOT | identity presence and duplicate detection | canonical sources, semantic references, representation parity | lifecycle and authority migration |
| Dependency and impact | resolvable references | typed edges, critical pins, stale-target detection | section or block selectors, local hashes, transitive impact |
| Topology and scale | node and edge metrics | Fan-In, Fan-Out, cycles, thresholds | graph budgets, public-concept exemptions, health trends |

A dimension can be not applicable. Scaffolding, editor integration, and
external Adapters are enabling tools rather than maturity dimensions.

## Unified Governance Edges

Semantic relationships normalize into one typed edge model:

```text
governance edge
  ├─ reference
  └─ pinned reference
```

These dependency edge kinds share target resolution, selectors, pins,
lifecycle, change impact, and topology policy. Standard Markdown links remain
navigational path-integrity inputs; they do not become semantic dependencies
merely because they are links.

Wiki Link, Markdown Link, and frontmatter declarations now share a versioned
reference-occurrence IR. Explicit syntax-and-context policy determines whether
an occurrence is a semantic dependency, navigation input, or identity
declaration before normalization; declaration syntax no longer chooses edge
semantics inside its parser.

Wiki Link and explicit anchor inputs normalize into this model. Issue adapters
may add `addresses`, `dependsOn`, and `evidencedBy` relations. The
profile serializes the ordered records, reverse transitive-impact sets, and
graph metrics. Heading selectors, impact, and Fan/cycle policy now consume the
same model; budgets and trends remain later layers.

ATX-block, file, and opt-in repo anchors now share the same edge model.
Block-local hashes, multi-anchor declarations, and deterministic transitive
propagation are delivered, so the repository dependency dimension now reaches
governed maturity.

Lifecycle status obligations and explicit `supersededBy` mappings now cover
assets, package domains, and leaves. Terminal identities cannot remain current
edge targets, and declared replacements appear in the profile graph. Historical
transition timing remains Git evidence rather than inferred state.

## Profiles and Overall Grades

A hygiene profile preserves target and observed maturity by dimension. The
configuration accepts the following schema:

```yaml
hygieneProfile:
  dimensions:
    structure: { target: governed, required: true }
    identity: { target: controlled, required: true }
    dependency: { target: controlled, required: true }
    topology:
      applicability: notApplicable
      rationale: No semantic dependency graph is governed.
```

If an overall grade is needed, it is the minimum result among applicable
required dimensions. The overall grade never replaces the dimensional result.

## Execution State

Rule execution continues to use `inactive`, `advisory`, `warning`, and `error`.
Project facts determine applicability and may recommend a maturity target;
explicit project policy remains authoritative for blocking behavior. Scale-only
heuristics do not silently promote a non-blocking baseline into an error.

## Delivery Boundary

[PRD-003](../engineering/prd/prd-003/index.md) and
[PRD-004](../engineering/prd/prd-004/index.md) define the profile evaluation and migration contract. Atomic invariant
registration, versioned profile output, N/A exclusion, legacy mapping,
suppression non-evidence, unified edges, heading selector resolution,
syntax-neutral reference collection,
scoped multi-anchor verification, transitive impact, Fan-In/Fan-Out and cycle
analysis, explicit topology thresholds and audited exceptions, responsibility,
review sunset, and knowledge redundancy are delivered. The current
four-level document-contract maturity and nine rule-family activation model
remain the executable compatibility baseline.
