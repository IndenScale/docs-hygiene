---
title: Documentation
description: Start from your first check and gradually understand the identity, dependency, and governance model of Docs Hygiene.
---

# Docs Hygiene Documentation

Docs Hygiene helps teams discover broken, conflicting, or outdated relationships between project documents. It reads explicit policies from the project, checks stable identities, semantic references, dependency changes, lifecycles, and delivery traceability, and produces results that are reproducible both locally and in CI.

## Start Here

| What do you want to do | Read this |
| --- | --- |
| Run DH in a repository for the first time | [Getting Started](getting-started.md) |
| Understand where check results come from | [How DH Works](how-it-works.md) |
| Configure document scope and rules | [Project Configuration](configuration.md) |
| Add checks to the commit gate | [CI Integration](ci.md) |
| Understand identity, references, and Pin | [Identity and Dependency](concepts/identity-and-dependency.md) |
| Confirm what DH does and does not do | [Capabilities and Boundaries](capabilities.md) |

## Minimal Workflow

```bash
docs-hygiene scaffold .
docs-hygiene check .
docs-hygiene explain-rules .
```

On the first run, address `error` findings first, then decide whether to promote warnings to the gate with `--fail-on-warning`. DH does not automatically modify documents during normal checks; migrations, Pin updates, and review resets all provide explicit plan-and-apply steps.

::: info Current Status
DH is still at an early stage. Delivered behavior is defined by the code, configuration, tests, and rule documents in the repository; roadmap directions should not be interpreted as current capabilities.
:::

## Documentation and Engineering Assets

The documentation on this site is for teams adopting and using DH. The UL, PRD, issue archives, and architecture decisions in the repository are DH's own product engineering assets, recording terminology authority, requirements, acceptance criteria, and implementation evidence. The two do not share the same reading path.
