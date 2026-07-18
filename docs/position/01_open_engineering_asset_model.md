# Open Engineering Asset Model

Status: adopted

Scope: built-in software-documentation profile

## Position

Docs Hygiene keeps long-lived product language and intent inspectable without
turning every delivery form into a documentation layer. The project is the
governed subject, the selected project directory is the execution boundary,
and repository topology remains a project choice.

## Asset Roles

| Role | Authority | Durable directory required |
| --- | --- | --- |
| UL | Shared language and long-lived constraints | Yes |
| PRD | Product intent, boundaries, and requirements | Yes |
| Issue | Change scope, acceptance, coordination, and evidence | No |
| Artifact | Implementation or evidence such as code, tests, config, SDK content, or commits | No |

UL and PRD are open engineering documents. Issues may live in a local archive
or an external tracker. Artifacts are discovered through adapters or cited by
Issues. Definition, Implementation, Glossary, and SDK are not built-in axes or
fixed directories.

## Reference Relation

A PRD Body consumes stable UL Library identities through semantic `references`
edges. An Issue `addresses` PRD requirements, may `dependsOn` another Issue, and
is `evidencedBy` Artifacts. Critical mutable dependencies may be pinned or
snapshotted. Navigational Markdown links do not create these relations.

## Language Representation

Language codes such as `en`, `zh`, and `ja` identify representations.
`canonical` and `localized` are authority properties, not language values. One
semantic asset has one canonical representation and zero or more localized
representations that preserve identity and governance relations.

## Governance Graph

```text
UL ◀── references ── PRD ◀── addresses ── Issue ── evidencedBy ──▶ Artifact
                                      └── dependsOn ──▶ Issue
```

Only stable identity, explicit relations, lifecycle, and evidence enter the
graph. No edge implies a mirrored directory or progressive refinement level.

## Boundaries

Current checks validate configured Markdown structures, stable identity,
lifecycle, localized parity, semantic and pinned references, content anchors,
and graph policy. Issue and Artifact integrations are adapter boundaries. The
checker does not infer natural-language equivalence, decide product acceptance,
or prescribe where code, tests, configuration, or SDK content must live.
