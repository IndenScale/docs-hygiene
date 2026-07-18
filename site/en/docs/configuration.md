---
title: Project Configuration
description: Configuration entry points, document scope, language representations, and rule enforcement policies in docs-hygiene.yml.
---

# Project Configuration

DH reads `docs-hygiene.yml` from the root of the project being checked by default. You can also use `--config` to point to a different policy file.

## Minimal Configuration

```yaml
entryDocs:
  required:
    - README.md
    - CHANGELOG.md

docs:
  bases:
    - id: guides
      root: docs
      requireContinuousNumbering: false
      maxLines: 300
      patterns:
        - id: guide-page
          regex: "^[a-z][a-z0-9-]*\\.md$"
          documentKind: guide
          numbered: false
```

This configuration declares public entry files and a governed document area. DH only applies the corresponding contracts to content explicitly brought into the governance scope.

## Document Areas

A `docs.bases` entry typically includes:

- `id`: a stable configuration identifier;
- `root`: the document directory relative to the project root;
- `localizedRoots`: localized representations of the same content;
- `patterns`: allowed file names and Document Kinds;
- `maxLines`: a file size budget;
- `ignore`: subtrees excluded from the current contract.

## Language Representations

```yaml
languageRepresentations:
  canonical: en
  localized: [zh]
  requireDocumentParity: true
  requireNumberParity: true
```

`canonical` and `localized` express an authority relationship, not aliases for language codes. A single semantic asset has one canonical representation and zero or more localized representations; they share the same stable identity and governance relationships.

## Explicit Rule Control

Projects can use three modes for stable rule families:

- `auto`: derived from project facts;
- `required`: the project explicitly requires enforcement;
- `disabled`: the project explicitly turns it off.

The actual enforcement state of a rule is still `inactive`, `advisory`, `warning`, or `error`. Use `explain-rules` to see the final result after facts, automatic determination, and explicit overrides.

## Multi-Dimensional Profile

```yaml
hygieneProfile:
  dimensions:
    structure:
      target: governed
      required: true
    identity:
      target: controlled
      required: true
    dependency:
      target: controlled
      required: true
    topology:
      applicability: notApplicable
      rationale: The current project does not govern a semantic dependency graph.
```

Each capability dimension declares its target independently. A dimension can also be explicitly marked as not applicable, but a rationale must be provided.

## Full Reference

For all current fields and compatibility requirements, see the [Configuration Guide](https://github.com/IndenScale/docs-hygiene/blob/main/docs/zh/guide/configuration.md) in the repository.
