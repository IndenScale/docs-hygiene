# Library Claim Governance

`governance.coreClaims` makes selected reusable meanings explicit Library SSOT.
Maintainers name the governed claim and its one canonical Library identity;
Docs Hygiene never infers authority or blocking duplicates from prose.

## Claim Registry

```yaml
governance:
  manifests:
    - docs/intent/ul/manifest.yml
    - docs/intent/prd/prd-001/manifest.yml
  coreClaims:
    - id: retry-policy
      authority:
        id: RETRY-POLICY
        selector: canonical-contract
      candidatePaths:
        - docs/intent/prd/**/*.md
      similarityThreshold: 0.72
      occurrences:
        - path: docs/intent/prd/prd-001/requirements/retries.md
          selector: copied-contract
          policy: forbidden
        - path: docs/intent/prd/prd-002/migration.md
          selector: legacy-contract
          policy: migrate
          migrateBy: 2026-09-30
        - path: docs/intent/prd/prd-003/summary.md
          selector: approved-excerpt
          policy: controlledExcerpt
```

Claim IDs are unique. `authority.id` must resolve to one governed Library
identity; `authority.selector`, when present, must resolve exactly once. A
superseded authority produces remediation from its `supersededBy` declaration.
Confirmed occurrence paths must be declared members of a governed Body, and
their heading selectors must resolve exactly once.

## Confirmed Duplicate Policies

- `forbidden` always blocks and directs the Body to the canonical identity.
- `migrate` requires `migrateBy: YYYY-MM-DD`. It is a warning through that UTC
  date and an error afterward.
- `controlledExcerpt` permits an explicitly reviewed excerpt only when the Body
  pins the configured authority block. It cannot declare `migrateBy`.

These rules consume only configured occurrences. Similar prose elsewhere does
not become a diagnostic until a maintainer confirms it in this registry.

## Controlled Excerpts

Controlled excerpts reuse the scoped-anchor contract:

```yaml
---
id: PRD-RETRY-SUMMARY
status: proposed
anchors:
  - target: RETRY-POLICY
    algorithm: sha256
    digest: <64-hex SHA-256 of the canonical heading block>
    scope: block
    locator: canonical-contract
---
```

The anchor target and locator must equal the claim authority. Missing and stale
pins produce `DH_CLAIM_001` at the Body occurrence or anchor declaration. The
same declaration becomes a `pinnedReference` governance edge, so ordinary
anchor validation, lifecycle enforcement, Fan analysis, and reverse transitive
impact continue to apply.

## Candidate Scan

```bash
docs-hygiene scan-library-claims . --format json
```

The read-only command compares each authority block with heading blocks matched
by that claim's explicit `candidatePaths`. It uses a deterministic normalized
word-set Jaccard score and the configured `similarityThreshold` in `0..=1`.
The versioned `docs-hygiene.library-claim-scan.v1` report contains authority and
unconfirmed candidate locations, selectors, lines, scores, and short evidence
fragments. Already registered occurrences are omitted.

Candidates are advisory data: finding one does not fail the command and does
not alter `docs-hygiene check`. Maintainers review the evidence and add a
confirmed occurrence only when the text is actually a duplicate definition or
controlled excerpt.

## Diagnostics And Migration

`DH_CLAIM_001` covers registry ambiguity, unresolved or terminal authorities,
invalid occurrence locations, forbidden duplicates, expired migrations, and
missing or stale controlled-excerpt pins. `DH_REFERENCE_001` continues to cover
the underlying semantic or pinned edge. Both belong to
`governance.identity`; the optional `identity.library-claims` profile invariant
becomes applicable when at least one core claim is configured.

Authority replacement uses the existing Library lifecycle: retain the former
identity as `status: superseded`, set `supersededBy` to a baselined or current
Library identity, then update claim authorities and Body dependencies to the
reported successor.
