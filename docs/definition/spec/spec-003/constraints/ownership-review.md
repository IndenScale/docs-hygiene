---
id: SPEC-003-C-019
status: baselined
---

# C-019 Ownership, Review Sunset, And Knowledge Redundancy

When ownership governance is configured, every `baselined` or `current`
governed asset, Package domain, and Package leaf declares one resolvable active
Owner, a current review deadline, and understanding confirmations from at least
two unique active person principals. Principal identities are project-local,
stable, offline policy records. Person and group identities have distinct
prefixes; groups list unique direct person members. An unexpanded group cannot
satisfy ownership and no group can count as individual understanding evidence.

Each confirmation records its person principal and date. Future, stale,
duplicate, unresolved, group, or inactive-person confirmations do not count.
The Owner may count only through their own valid person confirmation. Review
deadlines produce warning evidence inside the configured approach window and
error evidence after expiry.

Reset is an explicit exact-identity operation, dry-run by default. Apply writes
only the selected identity's advanced deadline and `lastReset` time, active
person actor, and reason, while atomically appending the same record to the
configured audit log. Ordinary content changes never renew evidence; invalid,
ambiguous, or non-advancing plans write nothing.

Evolving identities may prepare metadata without satisfying the gate. Terminal
identities are exempt from continuing ownership but retain C-015 lifecycle and
terminal-target obligations. Every established migration successor is checked
from its own declaration and cannot inherit the predecessor's confirmations.

Reports expose responsibility, current-review, and knowledge-redundancy
coverage, deadline risk, and per-identity bus factor. `DH_OWNERSHIP_001`,
`DH_REVIEW_001/002`, and `DH_KNOWLEDGE_001` are independent evidence for the
three delivered governed identity invariants. Legacy suppression makes the
matching invariant unverified and cannot prove responsibility or redundancy.
