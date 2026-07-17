# Critical Dependency Pins

`governance.criticalDependencies` upgrades selected normalized governance edges
from optional anchoring to an explicit Pin policy. Edges outside every matcher
remain valid without a Pin.

## Policy

```yaml
governance:
  pinAuditLog: .docs-hygiene/pin-updates.jsonl
  criticalDependencies:
    - id: reviewed-contracts
      match:
        sourceKinds: [body]
        targetKinds: [library]
        relations: [references, projects]
        sourcePaths: [docs/definition/**/*.md]
        targetPaths: [docs/intent/ul/**/*.md]
        sourceIds: [SPEC-1]
        targetIds: [RETRY-POLICY]
      require:
        algorithms: [sha256]
        minimumScope: block
        forbidWholeFile: true
        maxAgeDays: 90
```

Every non-empty matcher dimension is conjunctive; values inside one list are
alternatives. Empty lists are wildcards. Kinds are `body` and `library`.
Relations are `references`, `formalizes`, `realizes`, and `projects`, independent
of whether the normalized edge currently has a Pin. Paths are project-relative
globs; identity lists use stable governed IDs.

`algorithms` accepts `sha256` and `git`. By content range, scopes are listed
smallest to largest as `block | file | commit`. Policy strength is separate from
that display order: `minimumScope: file` accepts all three scopes;
`minimumScope: commit` accepts block or commit; and `minimumScope: block`
accepts only block. This treats commit provenance as stronger than a working-tree
file snapshot and block isolation as the strictest requirement.
`forbidWholeFile` rejects both file and commit scope and therefore requires a
SHA-256 block anchor.
`maxAgeDays`, when present, additionally requires valid `updatedAt`, non-empty
`updatedBy`, and non-empty `reason` audit metadata.

## Diagnostics

- `DH_PIN_001`: no companion pinned edge exists;
- `DH_PIN_002`: scope is below policy or whole-file scope is forbidden;
- `DH_PIN_003`: no Pin uses an allowed algorithm;
- `DH_PIN_004`: the declaration is valid but target content changed;
- `DH_PIN_005`: audit metadata is incomplete or older than `maxAgeDays`;
- `DH_PIN_006`: policy or anchor declaration is invalid or cannot be verified.

Each diagnostic identifies the direct source, relates the canonical target
location, and includes the target's deterministic reverse transitive impact.
This separates content drift from declaration damage. Existing
`DH_REFERENCE_001` anchor validation remains active underneath the policy.

## Audit Metadata

Explicit frontmatter anchors may record acceptance metadata:

```yaml
anchors:
  - target: RETRY-POLICY
    algorithm: sha256
    digest: <64-hex>
    scope: block
    locator: retry-contract
    updatedAt: 2026-07-17
    updatedBy: alice
    reason: reviewed upstream retry semantics
```

The new fields are optional for non-critical anchors, preserving existing
projects. A critical policy with `maxAgeDays` makes all three mandatory.

## Read-only Plan And Explicit Apply

```bash
docs-hygiene update-pins . \
  --actor alice \
  --reason "reviewed upstream contract" \
  --format json

docs-hygiene update-pins . \
  --policy reviewed-contracts \
  --target RETRY-POLICY \
  --actor alice \
  --reason "accepted retry revision" \
  --apply
```

The default is read-only. The versioned `docs-hygiene.pin-update.v1` plan lists
old and new digests, policy, source, target, relation, selector, actor, reason,
and date. Repeated `--policy` and `--target` options select a subset. Unknown
selections, unresolved selectors, malformed anchors, unsafe paths, unsupported
algorithm/scope pairs, or uncommitted commit targets block the selected plan
before any write.

`--apply` updates frontmatter anchors and appends the same records to
`governance.pinAuditLog` as one rollback-protected atomic batch. Missing
reference and vertical Pins can be synthesized in governed Markdown content
when their required scope is resolvable; existing Pins whose scope or algorithm
is no longer sufficient are migrated. Commit updates use `HEAD` only when its
target blob equals the working tree. Running `docs-hygiene check` never changes
or accepts a fingerprint.

The `dependency.scoped-anchor` profile invariant continues to prove mechanism
support. `dependency.critical-pins` is separately applicable only when at least
one critical dependency policy is configured and proves policy satisfaction.
