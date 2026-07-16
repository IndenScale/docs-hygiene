# Product References

This directory is the stable reference layer for Docs Hygiene product intent.
Requirements consume these definitions instead of redefining them locally.

## Authority

- [Ubiquitous Language](01_ubiquitous_language.md) defines the product terms,
  actions, invariants, results, and benefits shared by documentation, PRDs,
  tests, and future adapters.
- Implemented behavior remains authoritative in the CLI, configuration model,
  tests, and rule documentation under `docs/`.
- A planned term in the UL describes product intent, not shipped capability.

## Change Rule

New product concepts must first be defined in the UL or declared as a local
concept or change proposal in a PRD. A requirement may refine a shared concept,
but it must not silently give that concept a competing meaning.
