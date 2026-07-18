---
title: How DH Works
description: How Docs Hygiene turns project facts and explicit policies into reproducible check results.
---

# How DH Works

DH does not read a piece of natural language and guess "whether it is correct." It only checks relationships that the project can explicitly express and repeatedly verify.

## Inputs

Each run reads from the specified project root:

- The governance policy in `docs-hygiene.yml`;
- Governed Markdown, frontmatter, and Manifest files;
- Stable identities and their canonical/localized representations;
- Semantic references, Pins, lifecycles, and ownership declarations;
- External tool results provided by Adapters.

The project directory is the execution boundary. A Git repository is merely the physical carrier; DH does not automatically traverse all projects in a monorepo.

## Evaluation Process

```text
Project facts
   ↓
Rule applicability
   ↓
Automatic activation suggestions + explicit project policy
   ↓
Deterministic checkers
   ↓
findings / profile / impact sets
```

The same inputs and policy should produce the same results. Work that requires probabilistic judgment can consume DH output, but it must not be hidden inside blocking rules.

## Three Outcomes of Change

Dependency analysis distinguishes three things that are easy to conflate:

```text
potentialImpact = downstream consumers reachable on the graph
invalidated     = the pinned upstream state has changed
reviewSet       = stale consumers that policy requires to be handled
```

Plain references can tell the team "who might be affected"; Pins and content anchors provide deterministic evidence that "the reviewed state has changed"; project policy then decides which stale consumers must enter review.

## A Finding Is Not a Business Conclusion

DH can determine:

- A stable identity has no corresponding authoritative source;
- A semantic reference cannot resolve to the expected type;
- Pinned content no longer matches the reviewed state;
- A localized representation lacks its corresponding canonical identity;
- A terminated asset is still being consumed by current documents.

DH cannot determine:

- Whether a product requirement is commercially sound;
- Whether two differently worded passages are semantically equivalent;
- Which architectural approach should be adopted;
- Whether the team should accept or reject a change.

Machines reliably expose breakage; humans are responsible for meaning and choices.
