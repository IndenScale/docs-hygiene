# Product Requirements

This directory contains baselinable product-intent records. Requirements state
what Docs Hygiene should govern and how value can be observed. They do not
serve as implementation task lists.

## Authority

- Product language comes from the [UL registry](../references/01_ubiquitous_language.md).
- Current capability comes from code, configuration, tests, and rule docs.
- A `proposed` PRD records approved discussion scope, not delivered behavior.

## Lifecycle

`draft` → `review` → `baselined` → `superseded` → `archived`

An abandoned proposal uses `abandoned`; it is not forced to become a normative
semantic baseline. A baselined PRD must pin its UL registry version and resolve
or explicitly defer every local concept and semantic change proposal.

## Records

- [PRD-001 Intent Contract Governance](01_intent_contract_governance.md)
