---
id: SPEC-003-C-009
status: proposed
---

# C-009 Document Template Registry

`documentContracts.templates` is an ordered registry of reusable contract
fragments. Every template and profile has a stable identity containing only
ASCII letters, digits, `.`, `_`, and `-`. A profile binds to at most one
template through `template`.

Resolution is deterministic. Template sections, fields, and placeholder
patterns are placed before profile-local additions. Profile-local scalar values
override template scalar values; otherwise the template value and then the
compatibility default apply. Duplicate section or field identities in the
resolved profile are invalid rather than implicit overrides.

`DH_TEMPLATE_001` reports invalid identities, duplicate declarations, unknown
bindings, invalid expressions, and conflicting resolved members.
`DH_TEMPLATE_002` reports templates with no profile binding. Legacy inline
profiles remain valid and continue to execute, but do not prove reusable
template coverage.

The `structure.reusable-templates` invariant is controlled only when the
registry is valid, at least one template and profile exist, every profile is
bound, and every template is used. The versioned profile report exposes counts,
bindings, untemplated profiles, unused templates, and registry validity.
Suppression remains non-evidence.

Stable binding is not template lifecycle governance. Revision windows, profile
pins, and atomic migration are the separate governed
`structure.template-migration` invariant defined by [C-010](template-lifecycle.md).
