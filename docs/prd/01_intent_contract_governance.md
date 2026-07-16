---
id: PRD-001
type: product-requirement
status: proposed
ul_registry: docs-hygiene
ul_version: 0.1.0
---

# PRD-001 Intent Contract Governance

## Problem

AI coding agents amplify implementation faster than teams can verify shared
business meaning. Repository documents have formatting and link tools, but lack
deterministic governance for semantic identities, requirement dependencies, and
the path from intent to validation evidence. A structurally valid PRD can still
introduce an anonymous concept fork or cite a definition whose meaning later
drifts.

## Users and Benefits

| User | Need | Governed benefit |
| --- | --- | --- |
| Product owner | Review changes to business meaning before baseline | `BENEFIT-EARLY-DRIFT-DETECTION` |
| Engineer or coding agent | Resolve the exact concepts and invariants a change must preserve | `BENEFIT-REPLAYABLE-INTENT` |
| Reviewer | Distinguish deterministic defects from questions requiring accountable judgment | `RESULT-REVIEW-REQUIRED` |
| Documentation maintainer | Apply one reproducible policy locally and in CI | `RESULT-POLICY-PASSED` |

## Semantic Dependencies

```yaml
ul:
  registry: docs-hygiene
  version: 0.1.0
  references:
    - DH-PRODUCT
    - DH-INTENT-CONTROL-PLANE
    - DH-SEMANTIC-CONTRACT
    - DH-TRACEABILITY-CONTRACT
    - DH-COGNITIVE-DEBT
    - DH-REVIEW-ITEM
    - BENEFIT-EARLY-DRIFT-DETECTION
    - BENEFIT-REPLAYABLE-INTENT
  local_concepts: []
  change_proposals: []
```

## Requirements

### FR-001 Versioned semantic registry

The policy must load stable, typed semantic IDs from a configured registry and
must distinguish current capability from product direction. A baselined PRD
must resolve a fixed registry version rather than a floating latest definition.

### FR-002 Explicit PRD semantic manifest

A governed PRD must enumerate shared UL references, local concepts, and semantic
change proposals in a machine-readable manifest. Deterministic validation must
not depend on extracting every possible domain term from unrestricted prose.

### FR-003 Lifecycle-aware baseline gate

Draft and review records may contain declared proposals. A record entering
`baselined` must have valid fixed references and no unresolved proposal without
an accountable owner, decision state, and expiry. An `abandoned` record remains
historical context and does not become normative intent.

### FR-004 Reproducible semantic review queue

The policy must derive review items for local concepts, semantic changes,
similar names, repeated local use, and expiring deferrals. The queue must be a
reproducible artifact; meeting state and decisions remain versioned in the
repository.

### FR-005 Intent traceability

The policy must support typed relationships from UL concepts through PRD
requirements and acceptance criteria to executable or recorded validation
evidence. A requirement existing in Markdown does not prove delivery.

### FR-006 Deterministic and assisted checks

Missing IDs, invalid versions, type mismatches, and incomplete manifests may be
blocking diagnostics. LLM-assisted similarity or contradiction analysis may
create warnings or review items, but must not decide business meaning or block
a baseline without a deterministic policy selected by the repository.

## Non-goals

- Generate PRDs, technical designs, or implementation task lists.
- Replace Spec-Driven Development workflows or coding-agent planning.
- Treat every noun in natural-language prose as a governed concept.
- Claim that documentation alone proves current implementation behavior.
- Store meeting decisions in a hidden service that cannot be rebuilt from the
  repository.

## Acceptance Criteria

1. Given a PRD references an unknown UL ID, when the repository is checked,
   then a stable blocking diagnostic identifies the manifest entry.
2. Given a PRD enters `baselined` with a floating UL version, when the baseline
   gate runs, then the gate fails without rewriting the reference automatically.
3. Given a PRD declares a local concept, when review output is generated, then
   the item includes its definition, owner, relations, source PRD, and expiry.
4. Given optional semantic analysis suspects a contradiction, when the result
   cannot be established deterministically, then it is emitted as a review item
   rather than an automatic business decision.
5. Given a requirement claims delivery without linked validation evidence, when
   traceability is checked, then the missing proof is visible and the PRD text
   is not treated as execution truth.

## Delivery Status

This PRD is `proposed`. The current engine provides document contracts and
filename-backed concept foreign keys. The semantic registry, manifest gate,
review queue, and intent traceability rules described here are not implemented.
