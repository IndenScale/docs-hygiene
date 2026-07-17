# Configuration

Docs Hygiene reads `docs-hygiene.yml` from the checked project root unless `--config` points to another file. Breaking field renames are listed in [Terminology Migration](08_terminology_migration.md).

## Required Files

`entryDocs` declares project-root entry documents. The project root uses
an allow-by-default posture because project-level AI tooling can introduce
files such as `AGENTS.md`, `CLAUDE.md`, or `GEMINI.md`.

```yaml
entryDocs:
  required:
    - README.md
    - README_ZH.md
    - CHANGELOG.md
    - LICENSE
  optional:
    - AGENTS.md
    - CLAUDE.md
    - GEMINI.md
```

## Docs Structure

`docs.bases` controls documentation contract zones. A docs base uses a
deny-by-default posture for Markdown files: every checked `.md` file must match
one of the configured patterns or be excluded by global `ignore.paths` or the
base's own `ignore` list.

```yaml
docs:
  bases:
    - id: main
      root: docs
      requireContinuousNumbering: true
      maxLines: 500
      ignore:
        - docs/adr/**
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          documentKind: numbered
          numbered: true
        - id: index
          regex: "^INDEX\\.md$"
          documentKind: index
          numbered: false
```

`INDEX.md` is legal in this configuration but does not participate in
continuous numbering.

The older `docs.root` and `docs.filenamePattern` fields are still accepted as a
single-base shorthand.

## Multiple Bases

Documentation areas can use different naming rules. When a parent base contains
a child base, exclude the child root through the parent's `ignore` list so each
file has one governing base.

## Language Representations

`languageRepresentations` defines the canonical language and localized representations.

```yaml
languageRepresentations:
  canonical: en
  localized: [zh]
  requireDocumentParity: true
  requireNumberParity: true
```

By default, a base rooted at `docs` discovers `docs/zh` as its `zh` subtree.
When a semantic base and the locale hierarchy are orthogonal, use
`localizedRoots` to pair them explicitly:

```yaml
docs:
  bases:
    - id: intent
      root: docs/intent
      localizedRoots:
        zh: docs/zh/intent
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          numbered: true
```

The localized root is checked with the same filename patterns, numbering,
line budget, and document contracts as the canonical-language directory.

For this layout, `docs/01_overview.md` should have
`docs/zh/01_overview.md`.

## Language Representation CRUD

Language policy can be edited through CLI commands instead of manual YAML
patches.

```bash
docs-hygiene lang list
docs-hygiene lang add ja --min-cjk-ratio 0.10
docs-hygiene lang add en --canonical --max-cjk-ratio 0.05
docs-hygiene lang set-threshold ja --max-cjk-ratio 0.90
docs-hygiene lang remove ja
```

Every command accepts `--config` when the policy file is not
`docs-hygiene.yml`.

## Language Thresholds

`language` defines lightweight CJK ratio thresholds. Code fences are ignored.

```yaml
language:
  en:
    maxCjkRatio: 0.05
  zh:
    minCjkRatio: 0.15
```

## Document Contracts

`documentContracts.profiles` infers a document type from path and file name; `documentContracts.templates` supplies reusable contract fragments through stable profile bindings. The first matching profile applies. See [Document Contracts](06_document_contracts.md) for merge, maturity, and migration boundaries.

`documentContracts.maturity.declared` remains a severity floor for configured document profiles. Project-scale recommendations emit information; general rule applicability is derived independently through progressive activation.

Multidimensional `hygieneProfile.dimensions` configuration, migration, evidence, output, and CI semantics are defined in [Hygiene Profiles](12_hygiene_profile.md).

## Rule Activation

`rules` controls stable rule families independently from document-contract
maturity. The default `auto` mode derives applicability from centralized project
facts. `required` forces an error state and `disabled` forces an inactive state.

```yaml
rules:
  governance.traceability:
    mode: auto
  localization.parity:
    mode: required
  adapters.external:
    mode: disabled
```

Explicit modes override heuristics. Scale-only signals produce at most advisory
information; structural signals and explicit feature policy can produce warning
or error states. See [Progressive Rule Activation](10_progressive_rule_activation.md) for details.

## Governance Graph

`governance.manifests` enables the graph; `governance.topology` optionally enforces Fan and cycle thresholds; `governance.contentAnchors.verifyGitCommits` opts into local Git verification for explicit commit anchors and defaults to `false`. File and block SHA-256 anchors do not require it. See [Governance Graph](07_governance_graph.md) for the complete contract.

## Forbid ASCII Art

`docs.forbidAsciiArt` is disabled by default. Enable it to report ASCII flowcharts and box diagrams in prose:

```yaml
docs:
  forbidAsciiArt: true
```

## Suppressions

`suppressions` disables selected diagnostics for selected paths. This is useful
for fixtures, translated examples, generated docs, or mixed-language test cases.

```yaml
suppressions:
  - code: DH_LANG_002
    paths:
      - docs/fixtures/**
    reason: Fixtures intentionally contain Chinese examples in every locale.
```

Use `code: "*"` only for narrow paths where every Docs Hygiene diagnostic is
expected to be noisy.

## Ignore Paths

`ignore.paths` accepts glob patterns relative to the project root.
Docs Hygiene only checks Markdown files under each docs base root; other file
extensions are ignored by the built-in policy engine. Use `ignore.paths` for
generated directories, archives, fixtures, or any subtree that should not be
considered part of the active docs contract.

Use `docs.bases[].ignore` for paths that should be ignored only by one docs
base. This is useful when a parent docs directory contains independently
checked subtrees such as ADRs or user stories.

```yaml
ignore:
  paths:
    - target/**
    - docs/generated/**
```
