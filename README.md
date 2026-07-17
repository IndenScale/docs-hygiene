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
- opt-in, kind-scoped slug schemas with authoritative sources, normalized
  collision indexes, localized identity parity, aliases, and rename policy;
- path-inferred document contracts with reusable templates, deterministic
  profile binding, revision migration, and maturity-aware enforcement;
- a Kind registry shared by typed frontmatter validation and locale-aware,
  conflict-safe document scaffolding, with atomic Schema/Template migration;
- explicit core Library claim authorities, confirmed-duplicate policy,
  block-pinned controlled excerpts, and advisory similarity scanning;
- critical dependency matchers and Pin requirements across normalized edge
  relations, with read-only update plans and explicit audited application;
- portable commit snapshot manifests with offline file/block payload checks,
  typed provenance, Ed25519 trust, lifecycle policy, and explicit local import;
- exact node/direction supernode exceptions with audited budgets, expiry,
  degree trends, cleanup diagnostics, and non-passing `excepted` evidence;
- offline person/group principals, established-identity Owner, review sunset and
  two-person confirmation gates, coverage/bus-factor evidence, and atomic
  dry-run/apply review resets with JSONL audit;
- path, identity, and structure parity across canonical and localized representations;
- semantic references from governed content to `concept/*.md` and Library identities;
- project-root-local Markdown Links, image targets, and semantic Wiki Links;
- a versioned reference-occurrence IR with shared collectors and explicit
  syntax/context policy before semantic edge normalization;
- YAML frontmatter, identity Manifests, and recursive Package structure;
- lifecycle status obligations, terminal-target rejection, and explicit
  `supersededBy` authority migration across assets and package identities;
- normalized semantic, pinned, derivation, and projection governance edges with
  heading selectors, block/file/opt-in commit anchors, deterministic
  transitive impact, Fan-In/Fan-Out, cycle groups, and opt-in topology thresholds;
- versioned multidimensional hygiene profiles with target, observed, N/A, and
  invariant evidence while retaining independent rule execution states;
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
docs-hygiene profile
docs-hygiene profile --format json
docs-hygiene migrate-templates --check
```

See [Progressive Rule Activation](docs/10_progressive_rule_activation.md) for
the fact model, rule IDs, override precedence, and severity contract.
The profile evaluator now separates maturity, capability dimensions, and
execution state. See the [Documentation Hygiene Governance Model](docs/11_hygiene_governance_model.md).

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
./target/release/docs-hygiene profile /path/to/project --fail-below-target
```

If the binary is already installed or available on `PATH`:

```bash
docs-hygiene scaffold .
docs-hygiene scaffold . --kind article --identity ARTICLE-42 --slug cache-policy
docs-hygiene check --fail-on-warning
```

Errors fail by default, while warnings remain advisory. `--fail-on-warning`
promotes warnings to a gate. Use JSON when another tool consumes the report:

```bash
docs-hygiene check --format json
```

Other commands include `init`, `lang`, `migrate-templates`, `migrate-kinds`,
`scan-library-claims`, `explain`, and `explain-rules`. `update-pins` plans or
explicitly applies critical Pin refreshes; `import-snapshot` explicitly
materializes portable payloads from a local Git checkout; `reset-review` plans
or applies one audited deadline reset. Run
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
- [Library Claim Governance](docs/15_library_claims.md)
- [Critical Dependency Pins](docs/16_critical_dependency_pins.md)
- [Portable Commit Snapshots](docs/17_portable_snapshots.md)
- [Audited Supernode Exceptions](docs/18_supernode_exceptions.md)
- [Document Ownership And Review Sunset](docs/19_document_ownership.md)
- [Progressive Rule Activation](docs/10_progressive_rule_activation.md)
- [Documentation Hygiene Governance Model](docs/11_hygiene_governance_model.md)
- [Hygiene Profiles](docs/12_hygiene_profile.md)
- [Roadmap](ROADMAP.md)
- [Changelog](CHANGELOG.md)
