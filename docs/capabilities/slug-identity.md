# Slug Identity Governance

Document naming patterns establish shape; `docs.slugSchemas` optionally adds a
stable, project-indexed slug identity contract for selected Document Kinds.

## Configuration

```yaml
docs:
  bases:
    - id: articles
      root: docs/articles
      localizedRoots:
        zh: docs/zh/articles
      patterns:
        - id: article
          regex: "^(?P<slug>[a-z0-9-]+)\\.md$"
          documentKind: article
  slugSchemas:
    - documentKind: article
      source:
        type: filename
        capture: slug
      pattern: "^[a-z][a-z0-9-]*$"
      minLength: 3
      maxLength: 64
      reserved: [admin, api]
      normalization: lowercaseKebab
      identityField: id
      aliasesField: aliases
      renamePolicy: stableIdentity
```

Each Document Kind can have only one Schema and one authoritative source:
`filename` uses a named capture from the matching filename pattern;
`frontmatter` reads a configured field; and `stableId` projects a configured
stable-ID field. `normalization` accepts `none`, `lowercase`, or
`lowercaseKebab`. `pattern`, `minLength`, `maxLength`, and `reserved` apply to
the normalized value.

## Identity Index

The checker creates one deterministic index per Document Kind. Primary slugs
and aliases share its namespace, so normalized duplicates, case-folding
collisions, reserved values, and alias conflicts cannot silently resolve to two
identities. Canonical and localized files with the same stable ID must expose
the same normalized slug. Their filenames may still differ when frontmatter or
the stable ID is the authoritative source.

## Rename Lifecycle

`renamePolicy: stableIdentity` requires `identityField`, separating governance
identity from path. During an explicit rename migration, `requireAlias` also
requires at least one former slug in `aliasesField`; return to `stableIdentity`
after consumers migrate. `allowPathBreak` explicitly opts out of stable path
identity. Aliases are validated and indexed with current slugs.

`DH_SLUG_001` is CI-blocking when the `docs.structure` rule is active. Its JSON
`data` contains `originalValue`, `normalizedValue`, `documentKind`, optional
`conflictPath`, and executable `remediation`. No slug checks run when
`docs.slugSchemas` is empty.
