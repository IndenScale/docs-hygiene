# ADR-0001: Open Engineering Asset Model

Status: adopted

## Context

Docs Hygiene currently fixes one software-documentation profile into a global
three-level ontology: Intent, Definition, and Implementation. It also fixes two
parallel Library projections, UL to Glossary to SDK, and requires adjacent
`formalizes`, `realizes`, and `projects` edges. In practice, Glossary duplicates
UL authority, Spec contains change-scoped engineering contracts, and
Implementation manifests enumerate volatile repository paths. SDK, code,
configuration, tests, and other implementation evidence do not share one
portable directory shape.

The repository already keeps Feature Issues with capability boundaries,
acceptance criteria, dependencies, delivery state, and evidence. Those records
are a better owner for change-scoped engineering contracts than a permanent
Definition refinement level.

## Decision

Docs Hygiene will not prescribe Definition or Implementation refinement levels,
a Glossary projection, an SDK projection, or fixed implementation directories.
The built-in open-engineering profile has four roles:

- UL is the stable semantic authority and may contain precise definitions,
  aliases, boundaries, and normative claims.
- PRD is the stable product-intent authority for problems, outcomes,
  requirements, boundaries, non-goals, and lifecycle.
- Issue is the delivery authority for one change: scope, design constraints,
  decisions, dependencies, atomic acceptance criteria, status, and evidence.
- Artifact is a location-independent evidence target such as code, test,
  configuration, schema, command, commit, URL, or SDK surface.

UL and PRD may use governed directories in the built-in profile. Issue storage
and Artifact location are adapter concerns and are never inferred from one
required directory. A local Markdown Issue archive is one adapter, not the
ontology.

## Relations

The built-in profile retains semantic references, Pins, lifecycle migration,
ownership, impact, and topology. It removes `refinementLevel`, `formalizes`,
`realizes`, `projects`, and complete adjacent-level derivation. Delivery uses
location-independent relations:

```text
PRD   --references--> UL
Issue --addresses---> PRD or one PRD member
Issue --references--> UL
Issue --dependsOn----> Issue
Issue --evidencedBy--> Artifact
```

Issue and Artifact relations may be supplied by repository files or external
adapters. They do not make Artifact paths semantic authorities.

## Authority and Retention

Closing Issues must not hide current product behavior. User-observable behavior
is maintained in Guide, Governance, Capability, and Migration documentation;
stable meaning is maintained in UL; stable product intent is maintained in PRD;
executable truth remains in schemas, tests, configuration, and code.

A delivered Issue must retain a stable identity, frozen or audited acceptance,
resolvable evidence, and durable history. An external mutable Issue system must
provide a Pin, export, or portable snapshot before it can prove delivery.

## Migration

Existing Definition assets are migrated by claim rather than deleted wholesale:

- Glossary meaning and normative claims merge into UL.
- Stable product outcomes and boundaries merge into PRD.
- Change-scoped algorithms, decisions, acceptance, and delivery evidence move
  to the Issue that delivered the change.
- User-observable contracts remain in public product documentation.
- Executable invariants remain in schemas and tests.

The three implementation manifests and SDK manifest are removed after their
useful evidence is attached to Issues or already proved by tests. Existing
Definition/Implementation configuration and edge fields become rejected legacy
configuration rather than a silently accepted no-op.

## Consequences

The core model is smaller and applies beyond software projects without forcing a
waterfall directory shape. PRD stays stable instead of absorbing implementation
detail, while Issue becomes a governed delivery contract. Projects that want a
Spec, Glossary, SDK, or implementation manifest may define custom Document Kinds
and adapters, but those structures receive no built-in refinement semantics.

This is a breaking product-model change. Governance graph schemas, diagnostics,
activation evidence, examples, dogfood policy, tests, and position documents
must migrate together. The migration is complete only when no unique current
claim remains exclusively in the removed Definition assets.
