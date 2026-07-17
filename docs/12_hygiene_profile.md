# Hygiene Profiles

The profile evaluator reports governance depth separately from rule execution
severity. A profile is a vector, not a replacement for dimensional results.

## Configuration

```yaml
hygieneProfile:
  dimensions:
    structure: { target: controlled, required: true }
    identity: { target: controlled, required: true }
    dependency: { target: controlled, required: true }
    topology:
      applicability: notApplicable
      rationale: No semantic dependency graph is governed.
```

Applicable required dimensions participate in the overall minimum. Optional
dimensions are still reported but do not lower the overall result. A
`notApplicable` dimension requires a rationale and cannot declare a target.
An applicable topology target of `controlled` additionally requires explicit
`governance.topology` policy; merely discovering a graph proves only `basic`.

## Compatibility

During migration, `documentContracts.maturity.declared` maps only to the
structure target:

| Legacy maturity | Structure target |
| --- | --- |
| `seed` | `basic` |
| `growing` | `controlled` |
| `maintained` | `controlled` |
| `governed` | `governed` |

An explicit structure target must equal the mapped legacy target. Conflicts are
configuration errors rather than silent precedence. Legacy maturity never sets
identity, dependency, or topology targets.

## Evaluation

Each atomic invariant owns one dimension and minimum maturity. A level is
observed only when every applicable invariant at that level and all lower levels
passes. These outcomes do not count as passes:

- a visible checker failure;
- an explicitly disabled checker;
- a failure hidden by legacy suppression;
- a partial or missing invariant implementation.

An inactive rule without applicability evidence is excluded for that invariant.
When a legacy suppression matches, the profile retains its configured reason in
`suppressionReasons`, so migration debt remains auditable.
Execution state remains independent: advisory or warning may be non-blocking,
but its failing invariant still prevents the observed maturity result.

## Output and CI

```bash
docs-hygiene profile
docs-hygiene profile --format json
docs-hygiene profile --fail-below-target
```

JSON uses `docs-hygiene.profile.v1`. It contains the project fact snapshot,
ordered execution decisions, document-template coverage and bindings, the
normalized governance graph and topology metrics, dimensional targets and
observations, N/A rationale, invariant evidence, the optional overall result,
and whether all configured required targets are met.
The failure flag is opt-in and prints the report before returning a non-zero
status.

## Current Delivery Boundary

Atomic invariant evaluation, legacy mapping, N/A exclusion, suppression
non-evidence, target gating, reusable template binding and lifecycle migration,
normalized governance edges, heading selector resolution, Fan-In/Fan-Out and
cycle analysis, explicit topology thresholds, and versioned syntax-neutral
reference collection, scoped multi-anchor verification, transitive impact,
identity lifecycle, authority migration, critical Pin policy, and portable
offline commit snapshots, audited topology budgets, supernode exceptions, and
degree trends are delivered. An applied exception intentionally remains
`excepted`, never Passed evidence.
The authoritative coverage ledger is
[SPEC-003 C-007](definition/spec/spec-003/constraints/atomic-invariants.md).
