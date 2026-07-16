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

## Multilingual Boundary

Localized headings map to the same semantic section ID, so structural contracts do not require identical visible titles. Existing language-representation parity checks still govern counterpart presence. Detecting stale translated content needs a source revision or content hash and is intentionally outside this first contract increment.
