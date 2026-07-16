# Overview

Docs Hygiene is a **Policy Engine** for repository documentation.

It keeps documentation tidy at the repository boundary. The tool checks the
files that must exist, the shape of numbered docs, language parity, length
budgets, and concept integrity.

Docs Hygiene does not replace general-purpose Markdown or prose linters. It
checks repository-aware links and document contracts directly, while external
tools remain available through **Adapter** configuration.

## Product Boundary

Docs Hygiene owns documentation hygiene rules that require repository context:

- required public entry files
- root entry docs with an allow-by-default posture
- docs bases with a deny-by-default posture
- numbered docs structure and index files
- language-representation parity between canonical and localized documents
- language threshold checks
- concept foreign keys
- dead semantic Wiki Links and repository-local Markdown Link targets
- governed YAML frontmatter contracts
- type-specific document profiles and recursive Package directory structure
- external adapter orchestration

It does not own general Markdown formatting, external URL crawling, spelling,
or prose style. Those stay in tools such as markdownlint, lychee, Vale, cspell,
or slop-lint. This boundary does not exclude repository-aware validation of
Markdown targets, Wiki Link identities, frontmatter, or document structure.
