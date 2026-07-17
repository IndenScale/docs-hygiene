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
label. `[[DH-LIBRARY#library]]` selects the canonical `# Library` heading.
`[[DH-LIBRARY#library@sha256:<hash>|Library]]` additionally anchors the complete
canonical target bytes. A missing target or selector, an invalid refinement
direction, or a stale hash is an error. Canonical and localized packages preserve
the same Wiki Link targets, selectors, and hash anchors.

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

## Normalized Edge Record

[PRD-004](intent/prd/prd-004/index.md) now normalizes semantic references,
pinned references, derivations, and projections into one ordered dependency
edge record. Each record carries source and target identity, relation kind,
source location, optional selector and content anchor, and lifecycle provenance.
The profile reports ordered nodes and edges plus basic resolution, relation, and
isolation metrics. Existing resolution, staleness, and reverse-completeness
checks consume these records. Standard Markdown Links remain navigational and
never enter the semantic graph without an explicit semantic relation.
For non-Markdown implementation members, semantic Wiki Links are recognized
only on standalone comment lines; string literals and fixtures are not edges.

Declaration syntax is separated from that edge contract by the versioned
`docs-hygiene.reference-occurrence.v1` IR. Wiki Link, Markdown Link, and
frontmatter collectors all emit this shape. An explicit `(syntax, context)`
policy classifies them as semantic dependency, navigation only, or identity
declaration before the syntax-independent normalizer runs. This makes Markdown's
non-semantic default an inspectable policy while preserving the public edge JSON.
See [SPEC-003 C-012](definition/spec/spec-003/constraints/reference-occurrence-ir.md).

## Heading Selectors

Selectors use lowercase ASCII heading slugs, for example
`[[DH-HYGIENE-PROFILE#documentation-hygiene-profile]]`. Runs of punctuation,
formatting, and whitespace in an ATX heading collapse to one hyphen. The edge
retains the selector and `DH_SELECTOR_001` points to the source line when the
canonical target has no matching heading. A selector does not change an optional
SHA-256 anchor from whole-file to block scope. See
[SPEC-003 C-011](definition/spec/spec-003/constraints/selector-resolution.md).

## Scoped and Multiple Anchors

Markdown frontmatter may declare an `anchors` sequence. Each item names one
governed target and an explicit `file`, `block`, or `commit` scope. File scope
hashes the complete target; block scope requires a heading-slug `locator` and
hashes only that ATX section. Multiple items become independently ordered pinned
edges and report failures at their own declaration lines.

Commit scope uses `algorithm: git` and a full commit OID. It is disabled by
default and requires `governance.contentAnchors.verifyGitCommits: true`; the
checker then compares the current target with the same path's blob at that
commit. Existing inline selector-plus-SHA-256 links remain whole-file anchors.
See [SPEC-003 C-013](definition/spec/spec-003/constraints/scoped-content-anchors.md). Exported or cross-repository SHA-256 file/block anchors may instead carry typed `snapshot` provenance backed by registered local payloads; see [Portable Commit Snapshots](17_portable_snapshots.md) and [SPEC-003 C-017](definition/spec/spec-003/constraints/portable-commit-snapshot.md).

## Transitive Impact

The graph reports `transitiveImpact` by reversing every resolved semantic edge:
for each changed authority identity, it lists all direct and indirect dependent
identities in sorted order. Parallel edges are deduplicated, cycles terminate
without including the changed identity itself, and unresolved endpoints do not
propagate. This is deterministic analysis, not a budget or notification policy.
See [SPEC-003 C-014](definition/spec/spec-003/constraints/transitive-impact.md).

## Lifecycle and Authority Migration

Assets, package domains, and leaves may retain an old stable identity with `status: superseded` and name its established replacement through `supersededBy`.
The successor preserves refinement level and reference relation and is `baselined` or `current`; other statuses cannot declare one.
Superseded, archived, and abandoned identities cannot remain edge targets. Diagnostics identify stale consumers and suggest declared replacements; ordered declarations appear as graph `authorityMigrations`.
See [SPEC-003 C-015](definition/spec/spec-003/constraints/identity-lifecycle.md).

## Topology Policy

The graph reports Fan-In and Fan-Out as counts of distinct governed neighbor
identities, so repeated links or parallel edge kinds do not inflate degree.
Directed cycle groups are deterministic strongly connected components; a
self-loop is a one-node cycle group.

Topology enforcement is opt-in:

```yaml
governance:
  topology:
    maxFanIn: 8
    maxFanOut: 12
    forbidCycles: true
```

Configured limits activate the independent `governance.topology` rule family.
Graph presence and repository scale alone never activate blocking topology
policy. `DH_TOPOLOGY_001` reports Fan threshold violations and
`DH_TOPOLOGY_002` reports forbidden cycle groups; audited, exact-node directional exceptions follow [SPEC-003 C-018](definition/spec/spec-003/constraints/supernode-exceptions.md).
