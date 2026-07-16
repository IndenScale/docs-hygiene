# Docs Hygiene

Docs Hygiene is a policy engine for the intent surfaces of a repository. It
puts documentation governance into local checks and CI so shared intent does
not depend only on templates, review rituals, or individual diligence.

In AI-assisted development, implementation capacity scales faster than shared
understanding and business verification. A vague requirement can now drive
thousands of internally consistent lines of code before a team notices that
its concepts, rules, or expected benefits were ambiguous. Code has compilers,
types, tests, and static analysis; intent-bearing documents rarely have an
equivalent quality system. Docs Hygiene exists to move that governance left.

## Intent Control Plane

Code remains the execution truth of a system. Documents such as references,
PRDs, ADRs, acceptance criteria, and evidence indexes form its intent control
plane: they name the concepts, constraints, decisions, and observable outcomes
that implementation should preserve.

Docs Hygiene checks that control plane as policy-as-code. Today it provides
repository-level structural governance:

It is not a Markdown syntax linter. Use markdownlint for Markdown formatting, lychee for links, and Vale or cspell for prose quality. Docs Hygiene focuses on repository-level documentation governance:

- required files such as README, CHANGELOG, and LICENSE
- numbered docs under `docs/`
- document length budgets
- i18n parity between root docs and localized docs
- path-inferred document contracts with maturity-aware required sections
- concept foreign keys from highlighted terms to `concept/*.md`
- adapter orchestration for tools such as markdownlint

The product direction is to extend these foundations from structural hygiene
to semantic and traceability contracts:

- governed ubiquitous language and versioned concept references
- explicit local concepts and reviewable semantic change proposals
- PRD entity, action, invariant, benefit, and acceptance relationships
- traceability from shared intent to executable validation evidence

These contracts are intended to expose cognitive debt before an implementation
amplifies it. They do not ask an LLM to decide business meaning on behalf of a
team; deterministic checks block broken references and incomplete contracts,
while ambiguous semantic differences become explicit review items.

## Product Boundary

Docs Hygiene is not a Spec-Driven Development workflow or an execution planner.
It does not generate PRDs, technical designs, or task breakdowns, and it does
not prescribe how a coding agent should implement a change. SDD and coding
agents may consume the governed intent; Docs Hygiene verifies that the upstream
language, documents, and evidence relationships remain coherent.

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

This repository dogfoods Docs Hygiene with `docs-hygiene.yml`. The implemented
rule surface is documented under `docs/`; capabilities described as product
direction above are not implied to be available until they are documented
there.

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
