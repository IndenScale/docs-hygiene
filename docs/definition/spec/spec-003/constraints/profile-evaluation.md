---
id: SPEC-003-C-001
status: proposed
---

# C-001 Profile Evaluation

Each dimension records applicability, whether it participates in the overall
grade, target maturity, observed maturity, and ordered evidence. A level is
observed only when every applicable invariant registered at that level and all
lower levels passes. Execution severity never changes this conformance result.

Inactive because inapplicable is excluded. Explicitly disabled, skipped, or
suppressed checks do not prove an invariant and must be reported as unverified.
The optional overall grade is the minimum observed result among applicable,
required dimensions. An explicit `notApplicable` dimension is excluded and
must retain its rationale in the report.
