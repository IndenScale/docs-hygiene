# Audited Supernode Exceptions

Global Fan-In and Fan-Out limits remain the default topology contract. A
supernode exception acknowledges one intentional excess for one stable identity
and one direction; it is not a wildcard, diagnostic suppression, or proof that
the global threshold passed.

## Policy

```yaml
governance:
  topology:
    maxFanIn: 8
    maxFanOut: 12
    exceptions:
      - id: shared-retry-contract
        node: RETRY-POLICY
        direction: fanIn
        budget: 20
        reason: shared public retry semantics
        owner: platform-docs
        approvedBy: architecture-council
        expires: 2027-01-31
        history:
          - observedAt: 2026-04-01
            degree: 11
          - observedAt: 2026-07-01
            degree: 14
```

`id` and `(node, direction)` must each be unique. `node` is an exact registered
governance identity, and direction is `fanIn` or `fanOut`. The exception budget
must be greater than the corresponding global limit. `reason`, `owner`,
`approvedBy`, and a valid future-or-current `expires` date are mandatory.
There is no glob or wildcard field.

History dates are valid, strictly increasing, and not in the future. An active
violation needs at least one observation, which makes the most recent trend
`currentDegree - latestObservedDegree` inspectable. Renewal is an explicit
policy edit to the approval and expiry evidence; `check` never renews it.

## Runtime Semantics

An exception is `applied` only when all metadata is valid, the node exceeds the
global limit, recent history exists, and current degree is within the exception
budget. Only that node and direction avoid `DH_TOPOLOGY_001`. Other nodes, the
opposite direction, cycle policy, and excess beyond the exception budget remain
blocking.

The standard text/JSON report and profile expose each declaration with:

- status: `applied`, `idle`, `invalid`, `expired`, or `exceeded`;
- current degree, global and exception budgets, and signed remaining capacity;
- latest observation and trend delta;
- owner, approver, reason, expiry, and reverse transitive impact.

Removing the last excess edge makes the exception `idle` and emits a cleanup
warning. An expired exception becomes `expired`, no longer applies, and the
underlying threshold violation immediately returns.

## Diagnostics

- `DH_TOPOLOGY_001`: an ordinary or no-longer-excepted node exceeds budget;
- `DH_TOPOLOGY_002`: a forbidden directed cycle remains present;
- `DH_TOPOLOGY_003`: exception identity, target, direction, budget, audit fields,
  or expiry is invalid;
- `DH_TOPOLOGY_004`: an exception is idle and should be deleted;
- `DH_TOPOLOGY_005`: active exception history is missing, invalid, unordered, or
  future-dated.

An exception over its own budget has status `exceeded` and receives the normal
`DH_TOPOLOGY_001` failure. Invalid or expired declarations cannot relax policy.

## Profile Evidence And Legacy Suppression

Applied exceptions are retained as `topologyExceptions` evidence. The
`topology.thresholds` invariant reports outcome `excepted`, so a non-blocking
exception cannot prove Passed or raise observed topology maturity. Audited
budget, public-exception, and trend mechanisms are separately represented by
`topology.budgets`, `topology.public-exceptions`, and `topology.trends`.

Legacy `suppressions` still hide matching diagnostics for compatibility, but
the profile records them as `unverified`; they cannot substitute for an audited
supernode declaration or establish governed topology maturity.
