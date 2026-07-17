---
id: SPEC-003-C-015
status: proposed
---

# C-015 Identity Lifecycle and Authority Migration

Every governed asset, package domain, and package leaf has one stable identity
and one lifecycle status. Statuses have snapshot obligations:

| Category | Statuses | Authority meaning |
| --- | --- | --- |
| evolving | `draft`, `review`, `proposed` | not yet established as replacement authority |
| established | `baselined`, `current` | eligible authority and migration successor |
| terminal | `superseded`, `archived`, `abandoned` | must not be the target of a current governance edge |

A `superseded` identity must declare exactly one `supersededBy` stable ID. No
other status may declare that field. The successor must exist, differ from the
old identity, preserve `refinementLevel` and `referenceRelation`, and be
`baselined` or `current`. These rules prohibit self replacement, dangling
successors, cross-dimension replacement, and replacement chains whose next
authority is not yet established.

```yaml
id: TERM-OLD
status: superseded
supersededBy: TERM-NEW
```

Canonical and localized package representations preserve `supersededBy` just
as they preserve ID and status. The graph report exposes declared replacements
as the ordered `authorityMigrations` map. Any normalized edge still targeting a
`superseded`, `archived`, or `abandoned` identity reports `DH_GOVERNANCE_001` at
the consumer and, when available, names the replacement.

This is a current-state coherence contract. Git remains the evidence for when
historical transitions occurred. The checker does not rewrite consumers,
delete historical identities, or infer a successor from similar prose. The
governed lifecycle and authority-migration invariants pass only when these
snapshot obligations and all active identity checks pass without suppression.
Established successors independently satisfy the ownership, review-sunset, and
knowledge-redundancy contract in [C-019](ownership-review.md); predecessor
evidence is never inherited.
