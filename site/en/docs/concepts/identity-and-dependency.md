---
title: Identity and Dependency
description: Understanding the relationship between stable identities, semantic references, Pins, and change propagation.
---

# Identity and Dependency

DH treats project documents as content with stable identities and explicit relationships, not as a set of files that can only be found by path.

## A Path Is Not an Identity

A path answers "where is this file stored right now"; an identity answers "who is this object." When a requirement moves from `drafts/` to `product/`, or when a localization adds another file, the semantic object may still be the same one.

A stable identity makes these operations checkable migrations rather than silent broken links.

## Navigation Links and Semantic References

A standard Markdown Link is for navigation:

```markdown
[Configuration Guide](../guide/configuration.md)
```

A semantic reference declares "the current asset consumes the meaning of another stable identity":

```markdown
[[DH-GOVERNANCE-EDGE|Governance Edge]]
```

The former only requires the target to be reachable; the latter additionally requires identity, expected type, lifecycle, and governance relationship compatibility. DH does not infer business dependencies just because a link exists.

## Reference and Pin

A Reference expresses a dependency relationship; a Pin expresses "the upstream state I reviewed is exactly this one."

```text
Consuming asset ── Reference ──▶ Authoritative asset
Consuming asset ── Pin ────────▶ Reviewed upstream state
```

After an upstream change:

- References are used to compute potential impact;
- Pins are used to determine whether a consumer has gone stale;
- Project policy decides whether a review is required.

Not every reference needs a Pin. Only critical dependencies that require precise freshness evidence should be pinned; otherwise, unnecessary maintenance cost is created.

## Lifecycle

An identity does not disappear just because the content stops being effective. States such as `superseded`, `archived`, or `abandoned` preserve history while preventing terminated identities from continuing to serve as the current authority. A replacement relationship should explicitly point to the new stable identity.

## Localized Representations

Canonical and localized are different language representations of the same semantic asset. They share identity, lifecycle, and governance relationships; localization does not create a second authority.
