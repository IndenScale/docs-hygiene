---
id: GLOSSARY-001
version: 0.1.0
ul_registry: docs-hygiene
ul_version: 0.2.0
status: baselined
---

# Three-Layer Governance Glossary

## Source Semantics

This glossary projects `DH-THREE-LAYER-MODEL`, its three layer identities,
`DH-REFERENCE-LIBRARY`, `DH-GOVERNED-BODY`, and `DH-EVIDENCE-PLANE` from UL 0.2.0.

## Definition Identities

| Identity | Precise definition |
| --- | --- |
| `layer.intent` | Layer whose Body asserts desired outcomes and whose Library supplies product meaning. |
| `layer.definition` | Layer whose Body states falsifiable correctness and whose Library supplies precise specification terms. |
| `layer.implementation` | Layer whose Body realizes definitions and whose Library supplies reusable implementation primitives. |
| `role.body` | A concrete governed assertion within one layer. |
| `role.library` | A reusable reference consumed by Bodies in one layer. |
| `role.evidence` | A recorded observation that evaluates a versioned implementation against a definition and intent benefit. |
| `edge.references` | A Body-to-Library dependency within one layer. |
| `edge.formalizes` | A Definition Body declaring the Intent Body it makes falsifiable. |
| `edge.realizes` | An Implementation Body declaring the Definition Body it implements. |
| `edge.projects` | A downstream Library identity declaring its upstream semantic source. |
| `edge.verifies` | Evidence declaring the Definition and implementation version it evaluates. |

## Projection Rules

1. Every identity retains its UL source and version.
2. A Definition identity may narrow representation but cannot silently change meaning.
3. Relationship direction is stable: Body references Library; downstream formalizes,
   realizes, projects, or verifies upstream authority.
4. Missing intermediate edges remain missing even when a later artifact is linked directly.
