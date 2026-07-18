# Terminology Migration

The open engineering asset model removes the fixed refinement axis.

| Removed model | Current model |
| --- | --- |
| Intent refinement level | UL and PRD durable document structures |
| Definition / Spec / Glossary layer | PRD constraints plus Issue-scoped acceptance |
| Implementation / SDK layer | Location-independent Artifact evidence |
| `refinementLevel` | removed |
| `formalizes`, `realizes`, `projects` | removed |
| `requireCompleteVerticalDerivation` | removed |

Removed fields are rejected rather than ignored. Move reusable meaning into UL,
durable product requirements into PRD, and change-scoped constraints, acceptance,
coordination, and delivery evidence into an Issue. Code, tests, configuration,
SDK content, generated output, and commits may remain wherever the project owns them.

Other earlier configuration renames remain unchanged: filename-pattern `role`
became `documentKind`; language configuration uses `languageRepresentations`;
and repository-scale thresholds use project-scale names.
