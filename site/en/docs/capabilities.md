---
title: Capabilities and Boundaries
description: What Docs Hygiene can deterministically check today, and what is explicitly left to other tools and human judgment.
---

# Capabilities and Boundaries

DH is responsible for governance requirements that need project context and can be explicitly expressed and repeatedly verified.

## Current Capabilities

### Structure and Contracts

- Public entry files, document areas, and allowed file types;
- Document Kinds, frontmatter schemas, and scaffolding;
- Reusable templates, revision compatibility, and atomic migrations;
- canonical/localized path, identity, and structural parity.

### Identity and Authority

- Stable slugs and rename policies;
- Library identities and unique authoritative claims;
- Duplicate definition candidates and controlled excerpts;
- Lifecycle obligations and explicit authority migration.

### Dependency and Change

- Typed semantic references and target resolution;
- file, block, and repo content anchors;
- Critical dependency Pins and audited updates;
- Reverse impact, transitive impact, and staleness evidence.

### Graph and Accountability

- Fan-In, Fan-Out, cycles, and community boundaries;
- Exceptions with budgets, deadlines, and audit evidence;
- Owners, review sunsets, coverage, and knowledge redundancy;
- Per-dimension maturity targets and enforcement states.

## What DH Does Not Do

DH is not responsible for:

- Judging whether a piece of natural language matches business truth;
- Inferring whether two passages are semantically equivalent;
- Choosing product or architectural approaches for the team;
- Automatically dispatching Agents to modify the project;
- General Markdown formatting, spelling, style, or external URL crawling.

These boundaries keep blocking findings explainable and reproducible. Teams can use AI or manual review on top of DH output, but should preserve the distinction between probabilistic analysis and deterministic checks.

## Directions Not Yet Delivered

First-class Decision assets, general Agent Attestation, Issue Review, and richer external Issue/Artifact Adapters remain product directions. They should not be interpreted as current CLI behavior merely because they appear in the product model or roadmap.
