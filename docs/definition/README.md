# Definition Layer

This directory contains the Definition Layer. Its Reference Library is the versioned
`glossary/` recursive domain tree, with one precise identity per Markdown leaf. Its
governed Bodies are recursive Spec Packages under `spec/` and Test Definitions.

## Authority

- Every Glossary domain manifest declares its direct member set.
- Each Markdown leaf under `glossary/` defines one precise identity and its UL source.
- Each Spec manifest enumerates atomic constraints and verification members.
- A Spec pins the Glossary version and formalizes a source PRD using those identities.
- Test cases, models, oracles, and verifiers are Definition Bodies.
- Test results and runtime observations belong to the separate Evidence plane.

## Assets

- [Three-Layer Governance Glossary directory](glossary/)
- [Three-Layer Governance Spec](spec/spec-001/index.md)
