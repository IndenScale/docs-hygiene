---
id: SPEC-002-C-005
status: baselined
---

# C-005 Checker Enforcement

Inactive families do not emit diagnostics or invoke external processes.
Advisory and warning decisions cap derived diagnostics at Info and Warning;
error decisions retain each checker's configured severity semantics.
