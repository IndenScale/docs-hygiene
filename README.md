# Docs Hygiene

Docs Hygiene is a repository documentation checker. It runs locally and in CI
to verify that documents are complete, clearly organized, and traceable across
layers.

In AI-assisted development, implementation capacity scales faster than shared
understanding and business verification. A vague requirement can now drive
thousands of internally consistent lines of code before a team notices that
its concepts, rules, or expected benefits were ambiguous. Code has compilers,
types, tests, and static analysis; intent-bearing documents rarely have an
equivalent quality system. Docs Hygiene makes documentation continuously checkable.

## Documents

Docs Hygiene treats README files, PRDs, specifications, ADRs, and other written
material as maintained repository assets. Today it provides repository-level
structural checks:

It is not a Markdown syntax linter. Use markdownlint for Markdown formatting, lychee for links, and Vale or cspell for prose quality. Docs Hygiene focuses on repository-level documentation governance:

- required files such as README, CHANGELOG, and LICENSE
- numbered docs under `docs/`
- document length budgets
- i18n parity between root docs and localized docs
- path-inferred document contracts with maturity-aware required sections
- concept foreign keys from highlighted terms to `concept/*.md`
- versioned manifests for Intent, Definition, and Implementation documents
- references from subject documents to same-layer reference documents
- adjacent-layer traceability from intent through definition to implementation
- adapter orchestration for tools such as markdownlint

The product direction is to extend these foundations from structural hygiene
to semantic and traceability contracts:

- governed ubiquitous language and versioned concept references
- explicit local concepts and reviewable semantic change proposals
- PRD entity, action, invariant, benefit, and acceptance relationships
- item-level coverage and traceability from intent to definition and implementation

These contracts are intended to expose cognitive debt before an implementation
amplifies it. They do not ask an LLM to decide business meaning on behalf of a
team; deterministic checks block broken references and incomplete contracts,
while ambiguous semantic differences become explicit review items.

## Subjects and References

Each layer contains two document roles:

- a **Body** describes what this project specifically intends, defines, or implements;
- a **Reference** stores reusable terms, types, and rules so multiple Bodies can share
  a coherent vocabulary.

## Three Document Layers

Documents are organized into Intent, Definition, and Implementation:

| Layer | Body | Reference |
| --- | --- | --- |
| Intent | PRDs: why, for whom, and what outcome is wanted | Ubiquitous Language (UL): shared product terms |
| Definition | Specs and test definitions: what precisely counts as correct | Glossary: precise definitions of product terms |
| Implementation | Code and configuration: how the definition is implemented | SDK: reusable types, interfaces, and rules |

Bodies progress through `PRD → Spec/Test Definition → Code/Configuration`;
references are refined through `UL → Glossary → SDK`. This repository keeps Intent assets under `docs/intent`, Definition
assets under `docs/definition`, and their Chinese counterparts under `docs/zh`.
UL lives under `docs/intent/ul/`, and Glossary under `docs/definition/glossary/`; each
domain has a Manifest and each stable term is one Markdown leaf. PRD and Spec live
under `docs/intent/prd/` and `docs/definition/spec/` as directory Bodies. Implementation stays
in the repository root: `src/lib.rs` is the SDK, while code and configuration
relationships are declared in `implementation-manifest.yml`. The core
checker resolves these assets by stable ID and version, validates same-layer references,
and validates adjacent-layer `formalizes`, `realizes`, and `projects` edges.

## Product Boundary

Docs Hygiene is not a Spec-Driven Development workflow or an execution planner.
It does not generate PRDs, technical designs, or task breakdowns, and it does
not prescribe how a coding agent should implement a change. SDD and coding
agents may consume the governed intent; Docs Hygiene verifies that the upstream
documents and their reference relationships remain coherent.

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
