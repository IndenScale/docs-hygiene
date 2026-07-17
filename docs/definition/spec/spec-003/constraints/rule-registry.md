---
id: SPEC-003-C-002
status: proposed
---

# C-002 Rule Registry

One ordered registry owns every stable rule identifier, capability dimension,
minimum maturity, applicability evidence, default execution policy, checker,
diagnostic codes, remediation, and exception behavior. Configuration validation,
dispatch, profile evaluation, explanation output, and generated rule
documentation consume this registry.

The original eight activation families remain compatibility groups, not
sufficiently atomic maturity evidence. The independent `governance.topology`
family is added without renaming or reordering those IDs. A family that spans
dimensions or levels must expose atomic registered invariants while preserving
its public family ID and existing diagnostic codes during migration.
