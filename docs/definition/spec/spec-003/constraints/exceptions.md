---
id: SPEC-003-C-005
status: proposed
---

# C-005 Exceptions

An exception records rule ID, bounded paths, rationale, owner, and optional
expiry. Matching failures remain visible as excepted evidence rather than being
deleted. An exception cannot prove that an invariant passed. Missing rationale
or owner is invalid for a governed target; expired exceptions fail validation.

Legacy suppressions remain compatible but are reported as unaudited exceptions
and therefore cannot establish observed maturity for the suppressed invariant.
