---
title: Why Dead Link Checking Is Not Enough
description: Links may still open, but the meaning they reference has already changed. Document governance must move from path reachability to relationship validity.
date: 2026-07-18
topic: Assessing document trustworthiness
---

# Why Dead Link Checking Is Not Enough

A link returning 200 only means the target still exists. It does not mean the target is still the correct definition, nor that the referencing document has processed the upstream change.

This is precisely the blind spot of many document governance systems: they can detect a missing file, but they cannot see a stale knowledge dependency.

## Reachable Does Not Mean Correct

Suppose the gateway PRD links to a "rate limiting policy" document. Later, the team archives the old policy and creates a file with the same name in a new directory. To keep the link working, a maintainer updates the path.

The dead link check passes. But we still do not know:

- Whether the new file is the same semantic object as before;
- Whether it holds the authoritative definition;
- Whether the PRD expects a policy, a design, or an operations guide;
- Whether the PRD completed its review after the new policy changed.

A path only answers location; it cannot answer identity and relationship.

## Three Distinct Layers

Document references have at least three layers:

| Layer | Question it answers |
| --- | --- |
| Path navigation | Can the target be opened? |
| Semantic reference | Which stable identity does the current content consume? |
| State pinning | Which exact upstream state did the consumer review? |

A plain Markdown Link serves the first layer well and should not be forcibly interpreted as carrying all business relationships. When semantic governance is needed, the project should explicitly declare stable identities and References; when precise freshness is needed, Pins are added to critical dependencies.

## After an Upstream Change

When an authoritative definition changes, the system needs to distinguish:

1. Which consumers on the dependency graph might be affected;
2. Which consumers have a pinned upstream state that has deterministically gone stale;
3. Which stale consumers must enter review as required by policy.

If all three are conflated into "every link needs updating," the team is quickly overwhelmed by false positives. If only dead links are checked, real staleness passes silently.

## DH's Choice

DH preserves the distinction between navigation and semantics: Markdown Links handle path integrity, explicit semantic edges handle identity and dependency, and Pins provide deterministic freshness evidence.

It does not judge whether the old and new passages express the same business meaning. What it can determine is this: the upstream state a consumer previously reviewed has changed, and a human should reconfirm it.

The starting point of document governance is not making every link clickable — it is making the staleness of important relationships visible.
