# Cognitive Asset Governance Model

Status: adopted

Scope: Docs Hygiene narrative and product boundaries

## Position

Docs Hygiene engineers the governance of cognitive and narrative assets. It
treats documents and, in future review profiles, issues as governed assets whose
meaning, dependencies, and human judgments must remain inspectable as they
evolve.

Three responsibilities form the product-level model:

| Responsibility | Authority or mechanism | Governing question |
| --- | --- | --- |
| Semantic Authority | Library | What does this mean, and which definition is authoritative? |
| Change Propagation | Dependency | Who may be affected when an upstream asset changes? |
| Human Judgment | Decision | What did people choose, why, and when must it be reconsidered? |

These are responsibilities, not three symmetric graph entities. A `Reference`
is a kind of Dependency. A Library entry and a Decision record are governed
asset types.

## Common Substrate

The formal substrate separates nodes, edges, and review evidence:

```text
Asset
├── Library Entry
├── Decision Record
├── Document
└── Issue

Dependency
├── Reference
├── Pinned Reference
├── Derivation
└── Lifecycle Relation

Attestation
├── Agent Analysis
└── Human Confirmation
```

Stable identity, lifecycle, ownership, evidence, and representation metadata are
shared governance properties. The current CLI governs project documentation;
Issue Review and general Agent Attestation are product directions rather than
claims about shipped behavior.

## Semantic Authority

A Library provides canonical meaning for reusable concepts, entities, metrics,
rules, and design components. Consumers should reference an authority instead
of silently redefining it. AI may suggest an existing identity, identify an
ambiguous match or drift, and propose a new candidate; similarity alone cannot
create semantic authority or block CI.

Current shipped foundations include stable Library identities, semantic
references, claim authorities, advisory duplicate scanning, controlled excerpts,
and explicit authority migration.

## Change Propagation

A Dependency records that one governed asset consumes another. `Reference` is a
semantic Dependency; a Pin additionally locks a reviewed upstream state. A
change produces three distinct sets:

```text
potentialImpact  = structurally reachable consumers
invalidated      = pinned consumers whose locked upstream state changed
reviewSet        = invalidated consumers that policy requires someone to handle
```

An unpinned edge contributes only to `potentialImpact`: an upstream content
change does not automatically invalidate its consumer. Pinning a content hash
or equivalent reviewed state opts a dependency into deterministic freshness
failure. Projects therefore reduce false positives by Pinning only dependencies
whose exact upstream state must be reviewed. AI may further classify advisory
impact as `none`, `possible`, `material`, or `unknown`, but critical acceptance
remains attributable and auditable.

Architectural isolation is expressed by real intermediate contract assets. If
`A → A1 → B` replaces a direct `A → B` dependency, a B change first invalidates
A1. Propagation stops when A1 adapts without changing the contract consumed by
A. Docs Hygiene may analyze paths, communities, concentration, and possible
boundaries; it does not decide that an architect must introduce A1.

Current `transitiveImpact` is deterministic structural analysis and therefore
corresponds most closely to `potentialImpact`. Only a Pin or another explicit
freshness contract turns upstream drift into deterministic invalidation. Critical
Pins, scoped anchors, topology, and explicit exceptions provide shipped
foundations for selective review sets.

## Human Judgment

A Decision is a sparse record of a material human choice. A Policy is the
current rule established, amended, or retired by Decisions. Accepted Decision
content should retain its historical rationale; a changed choice creates a
successor instead of rewriting history.

Decision governance should cover alternatives, trade-offs, evidence, owner,
scope, review date, and lifecycle such as `proposed`, `accepted`, `superseded`,
or `retired`. Ordinary implementation choices, Pin refreshes, and Issue status
changes do not automatically become Decisions.

Current ownership review, audit records, document contracts, and lifecycle
migration are foundations. A first-class Decision asset and its Policy relations
are not yet shipped.

## Tool and Automation Boundary

Docs Hygiene owns deterministic asset discovery, dependency-graph analysis,
policy evaluation, diagnostics, and auditable update protocols. It does not own
Agent scheduling, unattended-service configuration, or architecture design.

An external Agent may consume versioned findings, analyze semantic impact, and
return attributed evidence. Project policy decides which Info findings may be
accepted automatically and which require people. The core tool must preserve the
actor, evidence, policy, and result rather than hiding probabilistic judgment
inside a deterministic checker.

## Refinement Level

Within the built-in software-documentation profile, refinement level positions
Intent, Definition, and Implementation assets. This is one profile's use of the
common substrate, not a fourth product-level responsibility.

## Reference Relation

Within that profile, Body and Library describe the role of an asset, while a
`Reference` is the Dependency edge by which a Body consumes Library authority.
The distinction preserves Library as the authority and Reference as a special
kind of Dependency.

## Language Representation

Canonical and localized forms are representations of one semantic asset. They
share identity and governance relations; localization does not create a second
authority or an independent Decision.

## Governance Graph

The product-level graph keeps asset, edge, and review evidence distinct:

```text
Library Entry ── Reference/Dependency ──▶ Consumer Asset
Decision Record ── establishes/amends ──▶ Policy
Attestation ── evaluates/confirms ──▶ Asset or Dependency state
```

The existing three-dimensional coordinates define how software documentation
instantiates part of this graph. See the
[Three-Dimensional Software Documentation Profile](01_three_dimensional_governance_model.md).

## Boundaries

The current product ships Library identity, typed documentation dependencies,
Pins and anchors, deterministic structural impact, topology analysis, ownership
review, audits, and lifecycle controls. First-class Decision assets, general
Agent Attestation, and Issue Review remain product directions. Docs Hygiene
provides dependency-graph evidence; it does not configure unattended Agents or
choose an architecture on a team's behalf.
