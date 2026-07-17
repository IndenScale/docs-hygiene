# Document Contracts

Docs Hygiene infers document intent from project-root conventions instead of requiring every Markdown file to declare its own type. A profile matches both a project-root-relative path glob and a file-name regular expression. Profiles are evaluated in configuration order, and the first match owns the document.

## Decision

Document governance follows four rules:

1. Path and file-name conventions classify project entry documents, section indexes, CHANGELOGs, ROADMAPs, and ADRs.
2. A profile defines only required fields and semantic sections. Additional sections remain valid.
3. The declared project maturity controls whether an incomplete profile contract is advisory or blocking. Project-scale signals may recommend a higher profile maturity but never change it automatically.
4. One semantic section can accept localized heading aliases. Translation freshness remains a separate concern from structural parity.

This keeps standard entry documents readable by GitHub and package tooling, avoids frontmatter used only as a type tag, and lets early repositories adopt governance incrementally.

## Classification

Each profile has `match.paths` and `match.filenames`. When both are present, both must match. More specific profiles should appear before general profiles.

```yaml
documentContracts:
  profiles:
    - id: adr
      match:
        paths: ["docs/**/decisions/*.md"]
        filenames: ["^\\d{4}-[a-z0-9-]+\\.md$"]
```

## Open Contracts

`requiredSections` uses stable semantic IDs with one or more accepted headings. The checker requires those sections and optionally their order, while leaving every other section untouched. `requiredFields` supplies a regular expression for visible metadata or repeated conventions that do not need frontmatter.

```yaml
requiredSections:
  - id: context
    headings: [Context, 上下文]
  - id: decision
    headings: [Decision, 决策]
requiredFields:
  - id: status
    pattern: "(?m)^Status:"
orderedSections: true
```

## Reusable Template Registry

`documentContracts.templates` extracts shared contract policy from individual
profiles. A profile names one template and may append its own sections, fields,
or placeholder patterns. Template list members are resolved before profile
members; profile scalar values override template scalars. Duplicate resolved
section or field IDs are configuration errors, not implicit overrides.

```yaml
documentContracts:
  templates:
    - id: maintained-open-contract
      revision: 1
      compatibleFrom: 1
      enforceFrom: maintained
      placeholdersAllowedUntil: growing
      orderedSections: true
  profiles:
    - id: project-readme
      template: maintained-open-contract
      templateRevision: 1
      match:
        paths: [README.md, README_ZH.md]
```

The profile report exposes template bindings and incomplete coverage. Controlled
reuse requires a valid registry, every profile bound, and every template used.
Inline profiles remain compatible but do not prove reuse. Governed templates
add positive integer revision windows and exact profile pins. Use
`docs-hygiene migrate-templates --check` to detect missing or compatible stale
pins, and `docs-hygiene migrate-templates` to advance them atomically. An
incompatible pin blocks every write. See
[SPEC-003 C-010](definition/spec/spec-003/constraints/template-lifecycle.md).

## Typed Document Kinds

An opt-in `documentKinds` entry binds one docs base and filename pattern to one
Profile. This makes naming, applicable paths, semantic sections, Template pins,
typed frontmatter, and generated skeletons one registry contract. Existing
Profile-only projects remain compatible; their `requiredFields` continue to
operate as whole-document regular expressions. See
[Document Kind Registry](14_document_kinds.md).

## Maturity And Placeholders

The maturity order is `seed`, `growing`, `maintained`, and `governed`. A profile's `enforceFrom` selects the first level where missing requirements become errors. Before that level they remain warnings.

Configured placeholder expressions make an admitted gap visible. A placeholder is informational through `placeholdersAllowedUntil` and becomes an error after that maturity. Recommendations can use project-root lines, bytes, and the number of managed documents. Every configured threshold in one recommendation must be met.

```yaml
maturity:
  declared: growing
  recommendations:
    - level: maintained
      minProjectLines: 10000
      minManagedDocuments: 20
```

The recommendation is diagnostic only. A project explicitly raises `declared`
before stronger profile gates take effect. General rule-family applicability is
derived separately through [Progressive Rule Activation](10_progressive_rule_activation.md),
so projects do not need to select one global maturity for every governance rule.

These four names remain the delivered Document Contract compatibility model.
[PRD-004](intent/prd/prd-004/index.md) proposes a separate three-level maturity
applied per capability dimension; it does not retroactively change this
configuration contract.

## Multilingual Boundary

Localized headings map to the same semantic section ID, so structural contracts do not require identical visible titles. Existing language-representation parity checks still govern counterpart presence. Detecting stale translated content needs a source revision or content hash and is intentionally outside this first contract increment.
