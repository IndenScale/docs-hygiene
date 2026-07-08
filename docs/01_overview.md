# Overview

Docs Hygiene is a **Policy Engine** for repository documentation.

It keeps documentation tidy at the repository boundary. The tool checks the
files that must exist, the shape of numbered docs, language parity, length
budgets, and concept integrity.

Docs Hygiene does not replace Markdown or prose linters. It delegates syntax,
style, and link checking to external tools through **Adapter** configuration.

## Product Boundary

Docs Hygiene owns documentation hygiene rules that require repository context:

- required public entry files
- root entry docs with an allow-by-default posture
- docs bases with a deny-by-default posture
- numbered docs structure and index files
- i18n parity between root docs and localized docs
- language threshold checks
- concept foreign keys
- external adapter orchestration

It does not own Markdown formatting, broken link crawling, spelling, or prose
style. Those should stay in tools such as markdownlint, lychee, Vale, cspell,
or slop-lint.
