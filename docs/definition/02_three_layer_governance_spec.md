---
id: SPEC-001
version: 0.1.0
status: baselined
prd: PRD-001
glossary: GLOSSARY-001
glossary_version: 0.1.0
---

# Three-Layer Governance Contract

## Intent Source

This Spec formalizes PRD-001 FR-001 through FR-007 and uses GLOSSARY-001.

## Inputs

A governed asset provides `id`, `layer`, `role`, `status`, and versioned relationship
entries. Relationship entries use `edge.references`, `edge.formalizes`, `edge.realizes`,
`edge.projects`, or `edge.verifies` and identify a stable target plus version.

## Constraints

1. `(layer.intent, role.body)` accepts PRD; `(layer.intent, role.library)` accepts UL.
2. `(layer.definition, role.body)` accepts Spec or Test Definition;
   `(layer.definition, role.library)` accepts Glossary.
3. `(layer.implementation, role.body)` accepts Code or Configuration;
   `(layer.implementation, role.library)` accepts the SDK that packages reusable primitives.
4. Every Body has an `edge.references` target in the same layer.
5. Every downstream Body has exactly one or more typed upstream trace edges.
6. Glossary and SDK identities use `edge.projects` to preserve semantic provenance.
7. `role.evidence` is not a Body or Library and must identify a Definition target,
   implementation version, result status, and Intent benefit.
8. `baselined` assets reject floating versions and unresolved undeclared proposals.

## Verification

The verifier must independently report invalid classification, missing same-layer
references, missing Body trace edges, missing Library projections, and missing Evidence.
One valid edge cannot satisfy a different missing edge. Current document-contract tests
verify the repository shape; manifest and graph diagnostics remain future implementation.
