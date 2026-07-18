# Document Kind Registry

The `documentKinds` registry makes document creation and validation consume one
contract. Each Kind binds an existing docs base and filename pattern to an
existing Document Profile. The Profile continues to own paths, semantic section
IDs, open-body behavior, template identity, and template revision; the Kind adds
typed frontmatter and deterministic scaffold policy.

## Registry

```yaml
documentKinds:
  - id: article
    base: articles
    pattern: article
    profile: article-profile
    scaffold:
      filename: "{slug}.md"
      title: "{identity}"
      sectionHeadings:
        context: { en: Context, zh: 上下文 }
        decision: { en: Decision, zh: 决策 }
    frontmatter:
      revision: 2
      compatibleFrom: 1
      revisionField: schemaRevision
      allowUnknownFields: false
      fields:
        - id: id
          type: string
          required: true
          source: identity
          format: "^ARTICLE-[0-9]+$"
        - id: slug
          type: string
          required: true
          source: slug
        - id: locale
          type: string
          required: true
          source: locale
          values: [en, zh]
        - id: status
          type: string
          required: true
          values: [draft, current, superseded]
          default: draft
        - id: priority
          type: integer
          default: 1
        - id: tags
          type: stringList
          default: [docs]
        - id: supersededBy
          type: string
      conditions:
        - when: { field: status, equals: superseded }
          required: [supersededBy]
```

The linked filename pattern must declare the same `documentKind`; the linked
Profile must be the first matching owner of every governed file. Profile
`requiredSections` remain open: additional body sections are valid. Frontmatter
openness is independent and explicit through `allowUnknownFields`.

When `docs.slugSchemas` declares the same `documentKind`, the registry also
binds that slug Schema. Its source capture or field and stable identity field
must exist in the Kind contract, and scaffold input must satisfy its normalized
pattern, length, reserved-name, and rename-policy constraints before any write.

## Typed Frontmatter

Field types are `string`, `integer`, `number`, `boolean`, and `stringList`.
`values` declares an enum, `format` is a string regular expression, `required`
controls presence, and `default` supplies scaffold input. `source` accepts
`input`, `identity`, `slug`, or `locale`. Conditions can require or forbid fields
when another field equals a value. `invariants` compare two fields with `equals`
or `notEquals`.

`DH_KIND_001` reports invalid registry bindings. `DH_KIND_002` reports missing,
compatible-stale, or incompatible Schema revisions. `DH_FRONTMATTER_001` reports
field and cross-field violations. The `structure.kind-schema` profile invariant
is applicable only when at least one Kind is configured.

## Kind-aware Scaffold

```bash
docs-hygiene scaffold . \
  --kind article \
  --identity ARTICLE-42 \
  --slug cache-policy \
  --locale en \
  --field priority=2
```

The generator resolves the Kind's base, locale root, filename pattern, Profile,
Template, frontmatter Schema, and section IDs before writing. `--target` may
override the project-relative destination directory only when the resulting path
still matches the Profile. `--dry-run` prints the path and content. Existing
files are rejected unless `--force` is explicit; invalid input never creates a
partial file.

## Atomic Migration

Schema revision windows and Template revision windows are checked together:

```bash
docs-hygiene migrate-kinds . --check --format json
docs-hygiene migrate-kinds .
```

The versioned report is `docs-hygiene.kind-migration.v1`. Compatible document
Schema revisions and compatible Profile Template pins advance together. A
malformed document, invalid field, future revision, or revision below
`compatibleFrom` blocks every document and policy write. The existing
`migrate-templates` command remains available for projects without Kind schemas.
