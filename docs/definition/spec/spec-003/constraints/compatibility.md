---
id: SPEC-003-C-004
status: baselined
---

# C-004 Compatibility

The first profile schema is versioned independently from diagnostic and
activation schemas. Existing rule IDs, diagnostic codes, `required` and
`disabled` behavior, and checker severities remain unchanged in that version.

Legacy contract maturity maps only to the structure target: `seed` to basic,
`growing` and `maintained` to controlled, and `governed` to governed. It never
sets identity, dependency, or topology targets. An explicit new structure target
that conflicts with the legacy value produces an actionable migration error;
there is no silent precedence.
