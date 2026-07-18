# Overview

Docs Hygiene is a **Policy Engine** for cognitive assets expressed in project
documentation. It compiles explicit identities, dependencies, lifecycle, and
project policy into deterministic findings; it does not infer business truth
from prose.

## Governance Responsibilities

| Responsibility | Authority or mechanism | Governing question |
| --- | --- | --- |
| Semantic Authority | Library | What does this mean, and which definition is authoritative? |
| Change Propagation | Dependency | Who may be affected when an upstream asset changes? |
| Human Judgment | Decision | What did people choose, why, and when must it be reconsidered? |

Library entries and Decision records are governed assets. A Reference is one
kind of Dependency. See the
[Cognitive Asset Governance Model](../position/02_cognitive_asset_governance_model.md).

The current CLI ships stable Library identities, semantic and pinned
Dependencies, scoped freshness evidence, lifecycle, structural impact, topology,
ownership review, and audited updates. A first-class Decision asset, general
Agent Attestation, and Issue Review are product directions rather than current
behavior.

## Review and Automation Boundary

Dependency analysis distinguishes three concepts:

```text
potentialImpact  = structurally reachable consumers
invalidated      = pinned consumers whose locked upstream state changed
reviewSet        = invalidated consumers that policy requires someone to handle
```

Current `transitiveImpact` is structural `potentialImpact`; critical Pins and
scoped anchors provide deterministic invalidation evidence. Unpinned edges stay
advisory when upstream content changes, so projects can avoid false positives by
Pinning only dependencies that require exact-state review. An external AI may
analyze Info-level findings, but Docs Hygiene does not configure unattended
Agents or hide probabilistic judgment inside deterministic checks.

Architecture remains a human responsibility. DH may expose paths, communities,
concentration, and possible isolation boundaries. It does not decide to insert
an intermediate contract or rewrite the dependency graph.

## Open Engineering Profile

The built-in profile keeps UL and PRD as durable documents, while Issue owns
change-scoped acceptance and Artifact remains location-independent. It preserves
semantic references and canonical/localized parity without prescribing Definition,
Implementation, Glossary, or SDK directories. See the
[Open Engineering Asset Model](../position/01_open_engineering_asset_model.md).

## Product Boundary

Docs Hygiene owns project-context-aware rules for:

- required public entry files and deny-by-default docs bases;
- numbered structure, indexes, document contracts, and governed frontmatter;
- stable identities, Library authorities, semantic references, and lifecycle;
- canonical/localized representation parity;
- typed Dependency edges, Pins, scoped anchors, impact, and topology;
- explainable progressive activation, ownership review, and audited updates;
- external Adapter orchestration.

It does not own general Markdown formatting, external URL crawling, spelling,
or prose style. Those stay in tools such as markdownlint, lychee, Vale, cspell,
or slop-lint. It does not decide whether a narrative is true, schedule Agents,
or make architecture and product Decisions on a team's behalf.

The delivered [Documentation Hygiene Governance Model](../governance/hygiene-governance-model.md)
separates capability dimensions, maturity, and execution state within this
boundary.
