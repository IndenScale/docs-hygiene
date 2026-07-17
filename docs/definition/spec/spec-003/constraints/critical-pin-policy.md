---
id: SPEC-003-C-016
status: baselined
---

# C-016 Critical Dependency Pin Policy

A critical dependency policy selects normalized edges by source and target
reference-relation Kind, normalized relation, source or target path glob, and
stable identity sets. Empty matcher dimensions are wildcards; unmatched edges
remain valid without anchors.

Requirements independently constrain allowed algorithms, minimum scope,
whole-file scope, and maximum audit age. Scope strength is ordered
`file < commit < block`. `DH_PIN_001` through `DH_PIN_006` distinguish missing,
insufficient-scope, disallowed-algorithm, changed-content, expired-audit, and
invalid-declaration states. Diagnostics expose the direct dependent and target
reverse impact.

Frontmatter anchors may carry `updatedAt`, `updatedBy`, and `reason`. A maximum
age requires all three. The `update-pins` workflow is read-only unless
`--apply` is explicit, supports policy and target subsets, validates the whole
selected plan before any write, and writes both updated declarations and
the configured JSONL audit log as one rollback-protected atomic batch. `check`
never accepts a new digest.

The versioned plan is `docs-hygiene.pin-update.v1`. Each change contains policy,
source, target, normalized relation, source and target paths, algorithm, scope,
selector, old and new digest, date, actor, and reason.
