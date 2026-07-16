# Docs Hygiene

Docs Hygiene is a repository documentation checker. It runs locally and in CI
to verify that documents are complete, clearly organized, and traceable from
intent through definition to implementation.

In AI-assisted development, implementation capacity scales faster than shared
understanding and business verification. A vague requirement can now drive
thousands of internally consistent lines of code before a team notices that
its concepts, rules, or expected benefits were ambiguous. Code has compilers,
types, tests, and static analysis; intent-bearing documents rarely have an
equivalent quality system. Docs Hygiene makes documentation continuously checkable.

## Documents

Docs Hygiene treats README files, PRDs, specifications, ADRs, and other written
material as maintained repository assets. Today it provides deterministic
repository-level governance checks:

It is not a general-purpose Markdown syntax or prose linter. Use markdownlint for formatting, lychee for external URL crawling, and Vale or cspell for prose quality. Docs Hygiene focuses on repository-level documentation governance:

- required files such as README, CHANGELOG, and LICENSE
- numbered docs under `docs/`
- document length budgets
- parity between canonical and localized language representations
- path-inferred document contracts with maturity-aware required sections
- concept foreign keys from highlighted terms to `concept/*.md`
- dead semantic Wiki Links and repository-local Markdown Link targets
- governed YAML frontmatter and type-specific directory contracts
- identity manifests for Intent, Definition, and Implementation documents
- semantic Wiki Links from Bodies to Libraries at the same refinement level
- adjacent-refinement-level traceability from intent through definition to implementation
- adapter orchestration for tools such as markdownlint

The product direction is to extend these foundations from structural hygiene
to semantic and traceability contracts:

- explicit local concepts and reviewable semantic change proposals
- PRD entity, action, invariant, benefit, and acceptance relationships
- item-level coverage and traceability from intent to definition and implementation

These contracts are intended to expose cognitive debt before an implementation
amplifies it. They do not ask an LLM to decide business meaning on behalf of a
team; deterministic checks block broken references and incomplete contracts,
while ambiguous semantic differences become explicit review items.

## Three Governance Dimensions

Every governed asset is located by three independent dimensions:

- **Refinement level**: Intent, Definition, or Implementation;
- **Reference relation**: a project-specific Body or a shared Library;
- **Language representation**: `en`, `zh`, or another configured language code.

One representation is canonical. Localized representations preserve the same semantic identity, lifecycle, structure, and governance edges; they are not separate assets.

## Refinement Levels and Reference Relations

Refinement progressively reduces ambiguity and implementation freedom:

| Refinement level | Body | Library |
| --- | --- | --- |
| Intent | PRDs: why, for whom, and what outcome is wanted | Ubiquitous Language (UL): shared product terms |
| Definition | Specs and test definitions: what precisely counts as correct | Glossary: precise definitions of product terms |
| Implementation | Code and configuration: how the definition is implemented | SDK: reusable types, interfaces, and rules |

Bodies progress through `PRD → Spec/Test Definition → Code/Configuration`;
Libraries are refined through `UL → Glossary → SDK`. This repository keeps Intent assets under `docs/intent`, Definition
assets under `docs/definition`, and their Chinese counterparts under `docs/zh`.
UL lives under `docs/intent/ul/`, and Glossary under `docs/definition/glossary/`; each
domain has a Manifest and each stable term is one Markdown leaf. PRD and Spec live
under `docs/intent/prd/` and `docs/definition/spec/` as directory Bodies. Implementation stays
in the repository root: `src/lib.rs` is the SDK, while code and configuration
relationships are declared in `implementation-manifest.yml`. The core
checker resolves these assets by stable ID, validates content-level Wiki Link references and optional SHA-256 anchors,
and validates adjacent-refinement-level `formalizes`, `realizes`, and `projects` edges.

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
