# Docs Hygiene

Docs Hygiene is a policy-as-code hygiene checker for repository documentation. It keeps docs tidy, complete, structurally consistent, and ready for CI gates.

It is not a Markdown syntax linter. Use markdownlint for Markdown formatting, lychee for links, and Vale or cspell for prose quality. Docs Hygiene focuses on repository-level documentation governance:

- required files such as README, CHANGELOG, and LICENSE
- numbered docs under `docs/`
- document length budgets
- i18n parity between root docs and localized docs
- path-inferred document contracts with maturity-aware required sections
- concept foreign keys from highlighted terms to `concept/*.md`
- adapter orchestration for tools such as markdownlint

## Quick Start

```bash
cargo run -- check --fail-on-warning
```

Create a starter policy:

```bash
cargo run -- init
```

Create a starter docs tree:

```bash
cargo run -- scaffold
```

Manage language policy:

```bash
cargo run -- lang list
cargo run -- lang add ja --min-cjk-ratio 0.10
cargo run -- lang set-threshold ja --max-cjk-ratio 0.90
cargo run -- lang remove ja
```

## Policy

This repository dogfoods Docs Hygiene with `docs-hygiene.yml`.

## Adapters

Docs Hygiene can call external tools instead of reimplementing their rules. The
first adapter is markdownlint:

```yaml
adapters:
  markdownlint:
    enabled: true
    command: markdownlint-cli2
    args:
      - README.md
      - README_ZH.md
      - CHANGELOG.md
      - "docs/**/*.md"
```
