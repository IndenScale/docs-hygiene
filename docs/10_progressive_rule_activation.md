# Progressive Rule Activation

Docs Hygiene derives rule-family decisions from one deterministic project-fact
snapshot. This keeps governance proportional to demonstrated need while leaving
explicit project policy in control.

## Decision Pipeline

```text
project files and policy
  → project facts
  → activation decisions
  → checker diagnostics
```

Facts include Markdown document and line counts, code lines, localized
documents, concept documents, Manifests, frontmatter, semantic Wiki Links,
configured document profiles, governed refinement levels, explicit topology
policies, and enabled Adapters.
Ignore policy is applied before facts are counted.

## Stable Rule Families

| Rule ID | Governance surface |
| --- | --- |
| `project.entry-docs` | required project entry documents |
| `docs.structure` | names, numbering, links, size, and structural policy |
| `documents.contracts` | path-inferred semantic document contracts |
| `localization.parity` | language thresholds and representation parity |
| `concepts.references` | concept foreign keys and orphan concepts |
| `governance.identity` | identities, packages, semantic references, and heading selectors |
| `governance.traceability` | adjacent-level derivation and projection |
| `governance.topology` | explicit Fan-In, Fan-Out, and cycle policy |
| `adapters.external` | external Adapter execution |

Every decision has a `mode`, a state, ordered evidence, a rationale, and
remediation. States progress through `inactive`, `advisory`, `warning`, and
`error`.

## Maturity Boundary

Activation state describes how one rule executes now; it is not a documentation
hygiene maturity level. The target model in [Documentation Hygiene Governance
Model](11_hygiene_governance_model.md) applies basic, controlled, or governed
maturity independently to each capability dimension while retaining these
activation states as explicit execution policy. The multidimensional profile is
delivered and reports these decisions without replacing them.

## Automatic Activation

`auto` is the default. Explicit feature configuration or structural presence
can activate the relevant family. Scale-only signals currently begin at 20
Markdown documents or 20,000 code lines and yield at most `advisory`; they do
not unexpectedly make CI blocking. Structural signals such as localized
documents or multi-level Manifests may yield `warning` even before full policy
is configured.

Evidence is monotonic within one fact model: adding a satisfied signal can only
preserve or strengthen a decision. Docs Hygiene does not persist hidden maturity
state or mutate policy. A team that needs a rule to remain permanently blocking
pins it as `required`.

## Explicit Authority

Per-rule policy overrides automatic inference:

```yaml
rules:
  governance.traceability:
    mode: auto
  governance.topology:
    mode: auto
  localization.parity:
    mode: required
  adapters.external:
    mode: disabled
```

- `auto` derives state from project facts;
- `required` selects `error` regardless of inferred facts;
- `disabled` selects `inactive` regardless of inferred facts and prevents the checker or external
  process from emitting diagnostics.

Unknown rule IDs and unknown mode values are configuration errors.
`governance.topology` remains inactive in `auto` mode until at least one explicit
`governance.topology` threshold is configured; graph presence or scale alone
never makes it blocking.

## Explainability

Inspect the current decisions without running the checks:

```bash
docs-hygiene explain-rules
docs-hygiene explain-rules --format json
```

Text output is intended for people. JSON uses schema
`docs-hygiene.rule-activation.v1` and contains the complete fact snapshot and
ordered decisions for CI, editor, or governance tooling.

When an unconfigured structural or scale signal activates a family, `check`
emits `DH_ACTIVATION_001` with the evidence and override path. Advisory decisions
cap derived diagnostics at Info, warning decisions cap them at Warning, and
error decisions retain the checker's configured severity semantics.

## Boundaries

The first contract does not auto-discover monorepo projects, infer business
risk from prose, persist hysteresis state, or invent missing policy parameters.
Each explicitly selected project root receives its own fact snapshot and rule
decisions.
