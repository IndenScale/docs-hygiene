# Three-Dimensional Governance Model

Status: adopted

Scope: Docs Hygiene product model

## Position

In the age of AI coding, project documentation is the SSOT for project intent
and decisions. Code and configuration are implementation facts, but governed
documentation is what stably explains why the project exists, who it serves,
what counts as correct, and which decisions ground the implementation. Docs
Hygiene does not generate those decisions. It turns the relationships that must
continue to hold among them into verifiable invariants.

The project is the governed subject, the directory is the execution boundary,
and a Git repository is only a physical container. One governance scope may
cover an entire repository or one project directory inside a monorepo. The
current CLI loads policy and resolves assets from one explicitly selected
project root per run; it does not auto-discover or orchestrate other projects.

Docs Hygiene locates every governed project asset along three independent
dimensions:

1. Refinement level preserves progressive refinement from intent to implementation.
2. Reference relation preserves semantic dependencies between project assertions and shared definitions.
3. Language representation preserves the same asset from working language to distribution languages.

The coordinate identifies an asset representation. Typed graph edges preserve reference and derivation relationships across those coordinates.
These invariants expose deviation early and reduce the comprehension cost of
repeatedly establishing terminology, document identity, authoritative
representations, and implementation grounds.

## Refinement Level

Refinement is the progressive reduction of ambiguity and implementation freedom. It does not imply that Intent is defective or incomplete.

| Refinement level | Body | Library | Governing question |
| --- | --- | --- | --- |
| Intent | PRD | UL | Why, for whom, and toward what outcome? |
| Definition | Spec and test definition | Glossary | What precisely counts as correct? |
| Implementation | Code and configuration | SDK | Which executable form realizes the definition? |

The Body derivation chain is `PRD → Spec/Test → Code/Configuration`. The Library projection chain is `UL → Glossary → SDK`. Each step reduces ambiguity, adds binding constraints, and narrows the remaining decision space.

## Reference Relation

A Body expresses a concrete project assertion. A Library maintains reusable terms, types, or rules that supply shared meaning. A semantic Wiki Link in Body content establishes a `references` edge to a Library identity at the same refinement level:

- `PRD → UL`;
- `Spec/Test → Glossary`;
- `Code/Configuration → SDK`.

`Reference` is reserved for the edge or act of referencing; the governed shared asset is called a Library.

## Language Representation

Language codes such as `en`, `zh`, and `ja` are values on the language-representation dimension. `canonical` and `localized` are authority properties, not language values.

One semantic asset has one canonical representation and zero or more localized representations. Localized representations preserve canonical path, identity, lifecycle, package structure, and governance edges. They are not independent graph nodes or competing sources of meaning.

```text
PRD-001
├── en  canonical
└── zh  localized
```

## Governance Graph

Asset coordinates and graph relationships are separate concepts:

```text
coordinate = (refinementLevel, referenceRelation, languageRepresentation)
edges      = references | formalizes | realizes | projects
```

- `references` is derived from a semantic Wiki Link connecting a Body to a Library identity at the same refinement level;
- `formalizes` connects a Definition Body to an Intent Body;
- `realizes` connects an Implementation Body to a Definition Body;
- `projects` connects a downstream Library to its adjacent upstream Library.

Governance is based on semantic authority rather than file extension. YAML can express intent policy, a definition schema, or runtime configuration; its refinement level depends on the assertion it carries.

## Boundaries

Current checks validate project-root-local Markdown targets, governed frontmatter, asset identity, lifecycle, the three-dimensional classification, type-specific package structure, language parity, Wiki Link references, optional content-hash anchors, and graph reachability. External URL reachability and general prose quality remain integration concerns. The checker does not infer natural-language equivalence, translation freshness, item-level coverage, or semantic contradictions.
