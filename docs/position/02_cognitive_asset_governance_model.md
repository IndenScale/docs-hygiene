# Cognitive Asset Governance Model

Status: adopted

Scope: Docs Hygiene narrative and product boundaries

## Position

Docs Hygiene engineers the governance of cognitive assets whose meaning,
dependencies, evidence, and human judgments must remain inspectable as they
evolve. Semantic Authority, Change Propagation, and Human Judgment are product
responsibilities, not symmetric directory layers.

## Asset Roles

The common substrate distinguishes governed Assets, typed Dependencies, and
Attestations. UL supplies semantic authority; PRD records durable product
intent; Issue records a scoped change and its judgment; Artifact supplies
implementation or evidence. A role describes authority, not storage location.

## Reference Relation

A Reference is a semantic Dependency on an authority. A Pin locks a reviewed
upstream state. Issue relations connect work to PRD requirements and Artifact
evidence. A change may produce potential impact, deterministic invalidation for
pinned consumers, and a policy-selected review set.

## Language Representation

Canonical and localized forms represent one semantic asset. They share stable
identity and governance relations; localization does not create a second
authority or an independent decision.

## Governance Graph

```text
Authority Asset ── Reference/Dependency ──▶ Consumer Asset
Issue ── addresses ──▶ Requirement
Issue ── evidencedBy ──▶ Artifact
Attestation ── evaluates/confirms ──▶ Asset or Dependency state
```

Stable identity, lifecycle, ownership, evidence, and representation metadata
are shared governance properties. Pins, structural impact, topology, ownership,
audits, and lifecycle controls operate on this substrate.

## Boundaries

Docs Hygiene owns deterministic discovery, dependency analysis, policy
evaluation, diagnostics, and auditable update protocols. It does not schedule
agents, infer semantic truth from prose, choose an architecture, or prescribe
repository topology. See the [Open Engineering Asset Model](01_open_engineering_asset_model.md)
for the built-in software-documentation profile.
