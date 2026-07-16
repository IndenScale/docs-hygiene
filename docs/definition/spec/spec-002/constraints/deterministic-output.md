---
id: SPEC-002-C-006
status: baselined
---

# C-006 Deterministic Output

`explain-rules` emits decisions in stable rule-ID order. JSON uses schema
`docs-hygiene.rule-activation.v1`; path traversal order cannot affect facts,
evidence, or output ordering.
