# Configuration

Docs Hygiene reads `docs-hygiene.yml` from the checked repository root unless
`--config` points to another file.

## Required Files

`entryDocs` declares repository root entry documents. The repository root uses
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
          role: numbered
          numbered: true
        - id: index
          regex: "^INDEX\\.md$"
          role: index
          numbered: false
```

`INDEX.md` is legal in this configuration but does not participate in
continuous numbering.

The older `docs.root` and `docs.filenamePattern` fields are still accepted as a
single-base shorthand.

## Multiple Bases

Different documentation areas can use different naming rules. When a parent
docs base contains a child docs base, use the parent base's `ignore` list to
avoid checking the child files twice.

```yaml
docs:
  bases:
    - id: guide
      root: docs
      requireContinuousNumbering: true
      ignore:
        - docs/adr/**
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          numbered: true
    - id: adr
      root: docs/adr
      patterns:
        - id: adr
          regex: "^ADR-\\d{4}_[a-z0-9_-]+\\.md$"
          role: freeform
          numbered: false
```

## I18n

`i18n` defines the root language and localized docs directories.

```yaml
i18n:
  rootLang: en
  languages: [zh]
  requireDocsParity: true
  requireNumberParity: true
```

For this layout, `docs/01_overview.md` should have
`docs/zh/01_overview.md`.

## Language CRUD

Language policy can be edited through CLI commands instead of manual YAML
patches.

```bash
docs-hygiene lang list
docs-hygiene lang add ja --min-cjk-ratio 0.10
docs-hygiene lang add en --root --max-cjk-ratio 0.05
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

`ignore.paths` accepts glob patterns relative to the repository root.
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
