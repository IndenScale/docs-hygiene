---
title: A File Path Is Not a Document Identity
description: Why semantic identity must be separated from storage location when files are moved, renamed, or localized.
date: 2026-07-18
topic: Stable identity
---

# A File Path Is Not a Document Identity

Engineering teams often treat a path as a document identity: `docs/api/rate-limit.md` *is* "the rate limiting policy." This convention is simple — until the file starts moving, getting renamed, being split, or gaining another language representation.

A path describes a storage location; an identity describes a semantic object. The two often coincide in small projects, but they should not be permanently bound.

## A Routine Directory Cleanup

A maintainer moves a file from:

```text
docs/api/rate-limit.md
```

to:

```text
docs/platform/traffic/rate-limiting.md
```

If the path is the identity, this cleanup is equivalent to deleting one object and creating another. Old references, review records, lifecycle, and localization relationships all need extra conventions to recover.

If the document has a stable identity `UL-RATE-LIMIT`, the path change is simply a migration of where that identity is represented.

## What a Stable Identity Enables

Once identity is separated from path, the team can deterministically check:

- Whether old and new files accidentally declare the same identity;
- Whether a localized file corresponds to the same canonical identity;
- Whether a rename preserves its alias and migration policy;
- Whether a terminated identity is still referenced by current content;
- Whether a replacement asset has explicitly taken over the authority relationship.

These capabilities do not require turning Markdown into a database. Files can still be read, moved, and version-managed as usual; key objects simply no longer have "who they are" accidentally determined by their path.

## Do Not Number Every File

Stable identities have a cost and should not be mechanically applied to every document. Temporary notes, generated output, and ordinary navigation pages do not necessarily need long-term identities.

The content that most needs stable identities is content that other assets depend on, that needs cross-language representation, that has a lifecycle, or that carries an authoritative definition. Examples include shared terminology, long-term requirements, and architecture decisions.

## Git Is Not a Semantic Identity Either

A Git commit is well suited to proving the content state at a point in time, but it is not suited to naming a business object. A stable ID says "who it is"; Git, hashes, and snapshots say "which state I reviewed."

Separating these two concerns allows document changes to both preserve history and maintain current authority.
