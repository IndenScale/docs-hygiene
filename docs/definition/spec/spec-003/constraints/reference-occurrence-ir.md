---
id: SPEC-003-C-012
status: baselined
---

# C-012 Reference Occurrence IR

Reference declaration syntax and governance semantics are separated by the
versioned `docs-hygiene.reference-occurrence.v1` intermediate representation:

```json
{
  "schemaVersion": "docs-hygiene.reference-occurrence.v1",
  "rawTarget": "TERM-1",
  "syntax": "wikiLink",
  "context": "governedContent",
  "location": { "path": "docs/body.md", "line": 12 },
  "payload": {
    "selector": "term",
    "anchor": { "algorithm": "sha256", "digest": "..." }
  }
}
```

`syntax` and `context` are open string identifiers rather than closed enums.
A collector recognizes one declaration surface and emits occurrences without
choosing an edge relation. Policy then maps the `(syntax, context)` pair to a
disposition:

| Syntax | Context | Disposition |
| --- | --- | --- |
| `wikiLink` | `governedContent` | `semanticDependency` |
| `markdownLink` | `projectNavigation` | `navigationOnly` |
| `frontmatter` | `identityDeclaration` | `identityDeclaration` |
| `frontmatter` | `governedAnchor` | `semanticDependency` |

Wiki Link, Markdown Link, and Markdown frontmatter collectors share this IR.
The frontmatter collector emits the declared stable `id` as an identity
declaration and each explicit anchor as a governed dependency. Navigation-only
and identity-declaration occurrences are explicit policy outcomes and do not
enter the governance graph. Scoped anchor payloads follow
[C-013](scoped-content-anchors.md).

The sole reference-edge normalizer consumes occurrences and policy. A new
syntax can therefore connect through a collector and policy entry without a
syntax branch in normalization. Unknown `(syntax, context)` pairs produce no
semantic edge. Built-in anchor policy supports SHA-256 for file and block
scopes and Git object IDs for explicitly enabled commit scope under C-013.

This IR is an internal extension contract. Its schema version changes when
field meaning or compatibility changes. Existing file-anchor JSON remains
unchanged; scoped anchors add explicit fields under C-013. Normalized edges
continue to follow [C-003](edge-normalization.md); selector resolution follows
[C-011](selector-resolution.md).
