# Governance Graph

Docs Hygiene builds a deterministic graph from explicitly configured, versioned
asset manifests. It validates relationship structure and reachability; it does
not infer business meaning from prose.

## Asset Sources

`governance.manifests` lists YAML files or Markdown files with YAML frontmatter.
Localized documents are representations of the same asset and should not be
listed as separate declarations.

```yaml
governance:
  manifests:
    - docs/intent/ul/manifest.yml
    - docs/intent/prd/prd-001/manifest.yml
    - docs/definition/glossary/manifest.yml
    - docs/definition/spec/spec-001/manifest.yml
    - sdk-manifest.yml
    - implementation-manifest.yml
  requireCompleteVerticalDerivation: true
```

## Asset Contract

Every declaration provides a stable `id`, semantic `version`, `layer`, `role`,
and lifecycle `status`. Supported layers are `intent`, `definition`, and
`implementation`; supported roles are `body` and `library`.

```yaml
id: SPEC-001
version: 1.0.0
layer: definition
role: body
status: baselined
references: { id: GLOSSARY-001, version: 1.0.0 }
formalizes: { id: PRD-001, version: 1.1.0 }
```

Each relationship accepts one target mapping or a list. Targets always include
an exact identity and semantic version.

## Recursive Package Trees

Directory Bodies and Libraries declare a non-empty `members` list. A member is one
direct Markdown leaf or child domain. Every child domain has its own `manifest.yml`,
declares `kind: domain`, and recursively enumerates only direct children. Tree depth
is unrestricted. Absolute paths, traversal, deep declarations, missing manifests,
duplicate identities, and undeclared children are errors.

Library leaves are stable terms and Body leaves are atomic roles, stories,
requirements, constraints, or verification statements. Every leaf has YAML
frontmatter containing `id`, semantic `version`, and lifecycle `status`. Localized
trees preserve every canonical path and identity. Library failures produce
`DH_LIBRARY_001`; Body Package failures produce `DH_BODY_001`.

UL and Glossary are recursive Library trees rather than monolithic term lists. PRD
and Spec are directory Body Packages rather than monolithic documents. Bodies use
normal Markdown links for individual terms while governance edges pin package versions.

## Horizontal References

Every Body declares at least one `references` edge to a Library in the same
layer:

- Intent Body to UL;
- Definition Body to Glossary;
- Implementation Body to SDK.

A missing target, a Body target, or a target in another layer is an error. A
Library cannot use `references` as a substitute for `projects`.

## Vertical Derivation

Vertical edges point from a downstream asset to adjacent upstream authority:

- Definition Body `formalizes` Intent Body;
- Implementation Body `realizes` Definition Body;
- Definition Library `projects` Intent Library;
- Implementation Library `projects` Definition Library.

Skipping a layer, reversing an edge, using the wrong role, or resolving an
unknown `id@version` is an error. A horizontal edge cannot satisfy a missing
vertical edge.

## Reverse Completeness

When `requireCompleteVerticalDerivation` is enabled, every `baselined` or
`current` non-Implementation asset must have an adjacent downstream derivation.
This catches a PRD without a Spec, a Spec without an implementation, a UL
without a Glossary, and a Glossary without an SDK. Draft, review, proposed, and
other non-baselined assets can remain without downstream derivation.

## Boundaries

The current graph validates asset-level identity, version, role, layer, edge type,
reachability, recursive Package membership, and localization identity parity. It does not yet validate item-level requirement
coverage, term-level projection completeness, symbol-level semantic mappings,
or natural-language contradictions.
