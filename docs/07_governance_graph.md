# Governance Graph

Docs Hygiene builds a deterministic graph from explicitly configured asset
manifests and semantic Wiki Links in Body content. It validates relationship structure and reachability; it does
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

Every declaration provides a stable `id`, `refinementLevel`, `referenceRelation`,
and lifecycle `status`. Supported refinement levels are
`intent`, `definition`, and `implementation`; supported reference relations are
`body` and `library`. Language representation is derived from the canonical root
or its configured `localizedRoots`, so localized files do not repeat the asset declaration.
Document-level `version` fields are rejected. Git history records past states;
review-sensitive dependencies may use content-hash anchors instead.
Manifest-level `references` and leaf-level `source` metadata are also rejected;
semantic dependencies belong in governed content as Wiki Links.

```yaml
id: SPEC-001
refinementLevel: definition
referenceRelation: body
status: baselined
formalizes: PRD-001
```

Vertical relationships accept one stable target ID or a list. Horizontal Library
references live in Body content rather than Manifest metadata.

## Recursive Package Trees

Directory Bodies and Libraries declare a non-empty `members` list. A member is one
direct Markdown leaf or child domain. Every child domain has its own `manifest.yml`,
declares `kind: domain`, and recursively enumerates only direct children. Tree depth
is unrestricted. Absolute paths, traversal, deep declarations, missing manifests,
duplicate identities, and undeclared children are errors.

Library leaves are stable terms and Body leaves are atomic roles, stories,
requirements, constraints, or verification statements. Every leaf has YAML
frontmatter containing `id` and lifecycle `status`. Localized
trees preserve every canonical path and identity. Library failures produce
`DH_LIBRARY_001`; Body Package failures produce `DH_BODY_001`.

An Implementation Body may group project-root-relative file members by kind, such
as `code` or `configuration`. When declared, these paths must be safe, unique,
and resolve to existing files.

UL and Glossary are recursive Library trees rather than monolithic term lists. PRD
and Spec are directory Body Packages rather than monolithic documents.

## Horizontal References

Every Body contains at least one semantic Wiki Link to a Library identity at the
same refinement level:

- Intent Body to UL;
- Definition Body to Glossary;
- Implementation Body to SDK.

A Definition or Implementation Library also contains at least one Wiki Link to
an adjacent upstream Library identity. These content links replace per-leaf
`source` frontmatter while the package-level `projects` edge remains explicit.

`[[DH-LIBRARY]]` resolves by semantic ID. `[[DH-LIBRARY|Library]]` adds a display
label. `[[DH-LIBRARY@sha256:<hash>|Library]]` additionally anchors the canonical
target bytes. A missing target, an invalid refinement direction, or a stale hash
is an error. Canonical and localized packages preserve the same Wiki Link targets
and hash anchors.

## Vertical Derivation

Vertical edges point from a downstream asset to adjacent upstream authority:

- Definition Body `formalizes` Intent Body;
- Implementation Body `realizes` Definition Body;
- Definition Library `projects` Intent Library;
- Implementation Library `projects` Definition Library.

Skipping a refinement level, reversing an edge, using the wrong reference relation, or resolving an
unknown ID is an error. A horizontal edge cannot satisfy a missing
vertical edge.

## Reverse Completeness

When `requireCompleteVerticalDerivation` is enabled, every `baselined` or
`current` non-Implementation asset must have an adjacent downstream derivation.
This catches a PRD without a Spec, a Spec without an implementation, a UL
without a Glossary, and a Glossary without an SDK. Draft, review, proposed, and
other non-baselined assets can remain without downstream derivation.

## Boundaries

The current graph validates asset-level identity, refinement level, reference
relation, edge type, reachability, semantic Wiki Links, optional content-hash
anchors, recursive Package membership, and language-representation identity parity.
Git history remains the authority for historical document states. It does not yet validate item-level requirement
coverage, term-level projection completeness, symbol-level semantic mappings,
or natural-language contradictions.
