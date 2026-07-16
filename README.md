# Docs Hygiene

English | [中文](README_ZH.md)

**Docs Hygiene (DH) is a governance tool for project documentation.**

In the age of AI coding, documentation is the SSOT for project intent and
decisions. Agents can amplify implementation capacity quickly, but they amplify
ambiguous requirements, unstable concepts, and broken constraints just as
quickly. Code quality already has compilers, type systems, tests, static analysis,
and CI. Documentation governance still largely stops at formatting, spelling,
and dead-link checks, without verifying whether intent is complete, shared
meaning is stable, or decisions reach implementation.

Docs Hygiene expresses governance requirements in project documentation as
continuously verifiable invariants. It does not interpret natural language on a
team's behalf. Instead, it deterministically exposes broken structure, identity,
references, and traceability before implementation amplifies the deviation.

## Three Kinds of Invariants

| Governance direction | What Docs Hygiene preserves |
| --- | --- |
| From intent to implementation | Progressive refinement and traceability from Intent through Definition to Implementation |
| From project assertions to shared definitions | Body references to same-level Library identities and progressive Library projection |
| From working language to distribution languages | Identity, structure, and governance parity between canonical and localized representations |

These directions correspond to three independent governance dimensions:
refinement level, reference relation, and language representation. Together,
they make intent-level decisions verifiably realizable in implementation, expose
deviation early, and reduce the cost of repeatedly rediscovering terminology,
document identity, authoritative representations, and implementation grounds.

See the [Three-Dimensional Governance Model](docs/position/01_three_dimensional_governance_model.md)
for the formal product model.

## Project and Execution Boundaries

The project is the governed subject, the directory is the execution boundary,
and a Git repository is only a physical container. One Docs Hygiene scope may
cover an entire repository or one project directory inside a monorepo. Each run
loads policy and resolves governed assets from an explicitly selected project
root. Docs Hygiene does not currently auto-discover or orchestrate every project
in a monorepo.

```text
monorepo/
├── platform/
│   ├── docs-hygiene.yml
│   ├── docs/
│   └── src/
└── sdk/
    ├── docs-hygiene.yml
    ├── docs/
    └── src/
```

The projects can be checked independently:

```bash
docs-hygiene check platform --fail-on-warning
docs-hygiene check sdk --fail-on-warning
```

## Current Capabilities

Docs Hygiene currently provides deterministic project-level governance checks:

- required entry files such as README, CHANGELOG, and LICENSE;
- numbered documents, allowed file types, and length budgets;
- path-inferred document contracts with maturity-aware enforcement;
- path, identity, and structure parity across canonical and localized representations;
- semantic references from governed content to `concept/*.md` and Library identities;
- project-root-local Markdown Links, image targets, and semantic Wiki Links;
- YAML frontmatter, identity Manifests, and recursive Package structure;
- adjacent-level governance edges across Intent, Definition, and Implementation;
- Adapter orchestration for external tools such as markdownlint.

Docs Hygiene does not replace tools for Markdown formatting, external URLs,
spelling, or prose quality. It does not infer natural-language equivalence,
translation freshness, or business contradictions. Item-level requirement
coverage and symbol-level semantic mapping remain future work.

## Progressive Governance

DH activates governance progressively from project facts instead of requiring
every project to select a global maturity up front. Structure-presence signals
such as localized documents, governed Manifests, multiple refinement levels,
frontmatter, and semantic Wiki Links activate their corresponding rule families.
Document and code scale can introduce non-blocking advice without unexpectedly
turning CI red.

Each stable rule family has an `inactive`, `advisory`, `warning`, or `error`
state. Project policy retains final authority through `auto`, `required`, and
`disabled` modes. Inspect the current decisions and their evidence with:

```bash
docs-hygiene explain-rules
docs-hygiene explain-rules --format json
```

See [Progressive Rule Activation](docs/10_progressive_rule_activation.md) for
the fact model, rule IDs, override precedence, and severity contract.

## Quick Start

Build the binary from this repository:

```bash
cargo build --release
```

Create a minimal documentation tree and starter policy for a project:

```bash
./target/release/docs-hygiene scaffold /path/to/project
```

Run the checks:

```bash
./target/release/docs-hygiene check /path/to/project --fail-on-warning
```

If the binary is already installed or available on `PATH`:

```bash
docs-hygiene scaffold .
docs-hygiene check --fail-on-warning
```

Errors fail by default, while warnings remain advisory. `--fail-on-warning`
promotes warnings to a gate. Use JSON when another tool consumes the report:

```bash
docs-hygiene check --format json
```

Other commands include `init`, `lang`, `explain`, and `explain-rules`. Run
`docs-hygiene --help` for the complete interface.

## Policy

Each governance scope reads `docs-hygiene.yml` from the project root by default.
Policy declares entry documents, documentation areas, language representations,
concept foreign keys, document contracts, governance Manifests, suppressions,
and external Adapters. A project can begin with structural hygiene and enable
stronger semantic and traceability gates as it matures.

This repository dogfoods Docs Hygiene with its own
[docs-hygiene.yml](docs-hygiene.yml). It demonstrates the complete
three-dimensional model, but it is not a fixed directory template that every
project must copy.

See [Configuration](docs/02_configuration.md) for policy syntax. Shipped behavior
is defined by [Rules](docs/03_rules.md) and the
[Governance Graph](docs/07_governance_graph.md).

## Adapters

Docs Hygiene owns governance rules that require project context. Existing tools
continue to own the surface checks they already perform well. Adapters orchestrate
those tools in the same run without reimplementing them in the core checker.

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

See [External Tool Adapters](docs/04_adapters.md) for the current contract.

## Documentation

- [Overview](docs/01_overview.md)
- [Configuration](docs/02_configuration.md)
- [Rules](docs/03_rules.md)
- [CI and JSON output](docs/05_ci.md)
- [Document Contracts](docs/06_document_contracts.md)
- [Governance Graph](docs/07_governance_graph.md)
- [Progressive Rule Activation](docs/10_progressive_rule_activation.md)
- [Roadmap](ROADMAP.md)
- [Changelog](CHANGELOG.md)
