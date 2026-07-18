# Document Ownership, Review Sunset, And Knowledge Redundancy

Ownership governance turns an established identity into an accountable,
time-bounded knowledge asset. It is opt-in: configuring `governance.ownership`
activates the policy for every `baselined` or `current` governed asset, Package
domain, and Package leaf.

## Offline Principal Directory

```yaml
governance:
  ownership:
    enabled: true
    confirmationMaxAgeDays: 365
    reviewWarningDays: 30
    resetAuditLog: .docs-hygiene/review-resets.jsonl
    principals:
      - id: person:alice
        kind: person
        status: active
      - id: person:bob
        kind: person
        status: active
      - id: group:platform-docs
        kind: group
        status: active
        members: [person:alice, person:bob]
```

Principal IDs are stable references, not copied display names. A person ID
starts with `person:` and has no members. A group ID starts with `group:` and
lists unique, directly resolvable person members. `inactive` people immediately
stop satisfying ownership and understanding evidence.

A group may own a document only when it is active and expands to an active
person. Understanding confirmations remain individual: a group never counts as
one or two people.

## Identity Metadata

Every established identity declares metadata in its own YAML Manifest or
Markdown frontmatter:

```yaml
ownership:
  owner: person:alice
  understoodBy:
    - principal: person:alice
      confirmedAt: 2026-07-01
    - principal: person:bob
      confirmedAt: 2026-07-02
review:
  reviewBy: 2026-10-01
  lastReset:
    at: 2026-07-17
    by: person:bob
    reason: quarterly semantic review
```

`owner` resolves to one active person or expanded group. `understoodBy`
requires at least two unique active person principals whose `confirmedAt`
dates are not future-dated or older than `confirmationMaxAgeDays`. The Owner
may be one of the two people but cannot prove both seats.

`reviewBy` is a valid current or future date. During `reviewWarningDays` before
the deadline, the checker emits a warning. After the deadline it emits an
error. Optional `lastReset` evidence must contain a non-future date, an active
person actor, a non-empty reason, and an `at` date no later than `reviewBy`.

`draft`, `review`, and `proposed` identities may prepare metadata without being
gated. `archived`, `abandoned`, and `superseded` identities have no continuing
ownership obligation; existing lifecycle and terminal-target rules still
apply. A `baselined` or `current` migration successor is checked independently
and cannot inherit stale confirmations from the old authority.

## Explicit Review Reset

Reset is a manual semantic-review action. It is dry-run by default:

```bash
docs-hygiene reset-review TERM-RETRY \
  --actor person:bob \
  --reason "quarterly semantic review" \
  --review-by 2027-01-31
```

The plan reports exactly one target and does not write. Add `--apply` to update
only that identity's `review.reviewBy` and `review.lastReset`, and append the
same typed record to `resetAuditLog` atomically. The actor must be one unique
active person, the new deadline must be in the future and advance any valid
existing deadline, and ambiguous or invalid targets produce zero writes.

Ordinary document edits never renew review evidence. JSON output uses
`docs-hygiene.review-reset.v1`.

## Diagnostics And Profile Evidence

- `DH_OWNERSHIP_001`: invalid principal directory or missing/unresolvable Owner;
- `DH_REVIEW_001`: missing, invalid, or expired review evidence;
- `DH_REVIEW_002`: review deadline is within the configured warning window;
- `DH_KNOWLEDGE_001`: fewer than two current active-person confirmations.

The standard report and `docs-hygiene.profile.v1` expose responsibility,
review, and knowledge-redundancy coverage as covered/total/percentage values,
due-soon and expired counts, and per-identity bus factor. Profile invariants
`identity.responsibility`, `identity.review-sunset`, and
`identity.knowledge-redundancy` require unsuppressed checker evidence;
suppression remains `unverified`, never Passed.

The delivery contract is [FEATURE-026](../zh/issues/features/26_ownership-and-community.md).
