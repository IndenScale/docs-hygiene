# Portable Commit Snapshots

Portable snapshots preserve commit evidence as registered local payloads. They
extend SHA-256 file and block anchors without requiring the producing repository,
a network connection, or Git during `check`. `scope: repo` anchors remain the
local full-repository mechanism.

## Registration

```yaml
governance:
  portableSnapshots:
    manifests: [snapshots/vendor-release.yml]
    requireSignatures: true
    trustedKeys:
      vendor-release: <64-hex-ed25519-public-key>
```

An empty configuration is the compatibility default. It performs no snapshot
I/O and adds no Git or network dependency. Every registered manifest and local
payload is checked even when no anchor currently consumes it.

## Versioned Manifest

```yaml
schemaVersion: docs-hygiene.snapshot.v1
id: vendor-release-1
repository: github:vendor/docs
commit: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
status: active
replacedBy: null
retainUntil: 2030-01-01
entries:
  - target: RETRY-POLICY
    path: docs/retry.md
    payload: payload/retry.md
    scope: block
    locator: retry-contract
    digest: <64-hex-sha256>
signature:
  algorithm: ed25519
  keyId: vendor-release
  value: <128-hex-signature>
```

`id` is the snapshot artifact identity. `repository` is a credential-free
stable identity such as `github:organization/repository`; URLs, user-info,
query strings, and fragments are rejected and never become authority. `commit`
is a full 40- or 64-hex OID. Entry `path` records the path in the producing
repository, while `payload` is relative to the manifest in this project.

File entries hash the complete payload bytes. Block entries retain the complete
source payload but hash only the uniquely resolved ATX section named by
`locator`, using the same normalization as ordinary block anchors. Repo scope
is intentionally not valid inside a portable manifest.

## Anchor Provenance

```yaml
anchors:
  - target: RETRY-POLICY
    algorithm: sha256
    digest: <64-hex-sha256>
    scope: block
    locator: retry-contract
    snapshot:
      manifest: vendor-release-1
      repository: github:vendor/docs
      commit: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
      path: docs/retry.md
```

The normalized pinned edge retains this typed provenance. The checker verifies
the manifest registration, repository, commit, source path, target, scope,
locator, digest, local payload, and current canonical target. A URL is never
used as the snapshot identity.

## Signatures And Lifecycle

An Ed25519 signature covers compact UTF-8 JSON of all manifest fields except
`signature`, in schema field order. This includes `schemaVersion`, identity,
repository, commit, lifecycle fields, and the ordered entry list. Trusted public
keys live in project policy; private keys and credentials never belong in a
manifest. `requireSignatures` makes absence an error, while any present
signature is always verified.

Statuses are `active`, `replaced`, and `revoked`. Only active snapshots may
prove anchors. A replaced snapshot requires `replacedBy` to resolve to another
registered active snapshot; replaced and revoked snapshots require a valid
`retainUntil` date. This preserves audit evidence without turning external
history into the project's semantic SSOT.

## Explicit Import, Offline Check

Remote acquisition is deliberately outside the checker. Clone or fetch the
producer explicitly, then import only from that local checkout:

```bash
docs-hygiene import-snapshot . \
  --manifest snapshots/vendor-release.yml \
  --source /tmp/vendor-checkout

docs-hygiene import-snapshot . \
  --manifest snapshots/vendor-release.yml \
  --source /tmp/vendor-checkout \
  --apply
```

The default `docs-hygiene.snapshot-import.v1` plan is read-only. It reads each
declared path from the exact commit with local Git, verifies the declared file
or block digest, and atomically materializes all payloads only with `--apply`.
It never clones, fetches, changes the signed manifest, or stores the checkout
location. After import, the source checkout can be removed and `check` remains
fully offline.

## Diagnostics And Profile Evidence

- `DH_SNAPSHOT_001`: manifest registration, schema, identity, or entry failure;
- `DH_SNAPSHOT_002`: repository identity or provenance mismatch;
- `DH_SNAPSHOT_003`: commit OID or provenance mismatch;
- `DH_SNAPSHOT_004`: source/payload path or payload availability failure;
- `DH_SNAPSHOT_005`: target, scope, locator, or digest mismatch;
- `DH_SNAPSHOT_006`: missing, untrusted, or invalid signature;
- `DH_SNAPSHOT_007`: invalid status, replacement, or retention policy.

`dependency.scoped-anchor` continues to describe anchor mechanism support.
`dependency.portable-snapshot` is independently applicable when manifests are
registered and proves the complete offline portability policy.
