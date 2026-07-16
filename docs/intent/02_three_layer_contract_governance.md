---
id: PRD-001
version: 1.0.0
type: product-requirement
status: baselined
ul_registry: docs-hygiene
ul_version: 0.2.0
---

# PRD-001 Three-Layer Contract Governance

## Problem

AI coding agents amplify implementation faster than teams can validate shared
business meaning. Governing only PRD formatting or terminology cannot detect
breaks among requirements, formal definitions, and implementation. Docs Hygiene
must establish Intent, Definition, and Implementation as classifiable, projected,
and traceable contracts without treating document existence as implementation evidence.

## Users and Benefits

| User | Need | Governed benefit |
| --- | --- | --- |
| Product owner | Review changes to intent meaning and benefit before baseline | `BENEFIT-EARLY-DRIFT-DETECTION` |
| Engineer or coding agent | Find definitions and implementation constraints for a requirement | `BENEFIT-REPLAYABLE-INTENT` |
| Reviewer | Trace a requirement to falsifiable definitions and runtime evidence | `BENEFIT-LAYERED-TRACEABILITY` |
| Documentation maintainer | Rebuild the same result locally and in CI | `RESULT-POLICY-PASSED` |

## Semantic Dependencies

```yaml
ul:
  registry: docs-hygiene
  version: 0.2.0
  references:
    - DH-THREE-LAYER-MODEL
    - DH-INTENT-LAYER
    - DH-DEFINITION-LAYER
    - DH-IMPLEMENTATION-LAYER
    - DH-REFERENCE-LIBRARY
    - DH-GOVERNED-BODY
    - DH-EVIDENCE-PLANE
    - DH-SEMANTIC-CONTRACT
    - DH-TRACEABILITY-CONTRACT
    - BENEFIT-EARLY-DRIFT-DETECTION
    - BENEFIT-REPLAYABLE-INTENT
    - BENEFIT-LAYERED-TRACEABILITY
  local_concepts: []
  change_proposals: []
```

## Requirements

### FR-001 Three-layer asset classification

Policy must classify assets by responsibility rather than extension. The Intent
Body is the PRD and its Library is the UL. The Definition Body is the Spec or Test
Definition and its Library is the Glossary. The Implementation Body is Code or
Configuration and its Library is the SDK; the SDK contains shared types, schemas,
interfaces, and rules. Every governed asset must have a stable identity, layer,
role, and lifecycle.

### FR-002 Same-layer Library references

Every Body must declare its same-layer Library dependency: `PRD → UL`,
`Spec/Test → Glossary`, and `Code/Configuration → SDK`. A baselined asset
must pin dependency versions rather than resolve a floating latest version.

### FR-003 Reference projection

Policy must govern the `UL → Glossary → SDK` projection. A downstream
Library identity must record its source identity and semantic version. Narrowing,
splitting, merging, and incompatible changes require explicit proposals.

### FR-004 Body traceability

Policy must govern `PRD → Spec/Test Definition → Code/Configuration`. A Spec must
cover the requirements, invariants, benefits, and acceptance criteria of its source
PRD. An implementation must declare the Spec it realizes. A direct link to a test
result must not conceal a missing intermediate layer.

### FR-005 Evidence plane

Test cases, models, oracles, and verifiers belong to Definition. Test results,
acceptance records, runtime observations, and metric values belong to Evidence.
Evidence must identify the definition, implementation version, and Intent benefit
it verifies. A definition is not a passing result, and technical success alone does
not prove a user benefit.

### FR-006 Lifecycle and review queue

Drafts may contain declared proposals. A `baselined` asset must use valid pinned
references and resolve or explicitly defer open proposals. Local concepts, semantic
changes, broken traces, and expiring deferrals must create rebuildable review items.

### FR-007 Deterministic and assisted checks

Missing IDs, invalid versions, type errors, broken relationships, and incomplete
manifests may block. LLM-assisted similarity or contradiction analysis may create a
warning or review item, but it must not decide business meaning for the team.

## Non-goals

- Generate PRDs, Specs, technical designs, or task lists.
- Replace SDD workflows, test frameworks, or coding-agent planning.
- Treat directory names or extensions as the only layer classifier.
- Claim that a document, test definition, or metric proves current behavior.
- Require one packaging form for every Implementation Library.

## Acceptance Criteria

1. Given a PRD references an unknown or floating UL, when the baseline gate runs,
   then a stable blocking diagnostic is produced.
2. Given a Spec lacks a Glossary reference or PRD coverage, when Definition is
   checked, then the missing identity and trace edge are independently visible.
3. Given Code or Configuration declares neither its Spec nor SDK dependency, when
   Implementation is checked, then file existence cannot prove delivery.
4. Given a Glossary or SDK symbol has no upstream semantic version, when Reference
   projection is checked, then drift is blocked or enters explicit review.
5. Given a Test Definition has no result, when Evidence is checked, then its state
   is missing proof rather than passed.
6. Given Evidence proves technical behavior but not user benefit, when the end-to-end
   trace is checked, then the missing edge in `PRD → Spec/Test → Code/Configuration
   → Evidence → Benefit` is visible.
7. Given assisted analysis cannot decide a suspected contradiction deterministically,
   when output is produced, then it becomes a review item rather than a business decision.

## Delivery Status

The three-layer asset model and this PRD are baselined. Document contracts govern UL
and PRD under `docs/intent` and Glossary and Spec under `docs/definition`. Implementation
remains at the repository root: the public Rust library is the SDK, and
`implementation-manifest.yml` declares the CLI/Configuration Body relationships to its
Spec and SDK. Generic manifest validation, projection graphs, and end-to-end trace
diagnostics remain future work and must not be described as shipped.
