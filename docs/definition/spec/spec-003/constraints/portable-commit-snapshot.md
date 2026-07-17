---
id: SPEC-003-C-017
status: baselined
---

# C-017 Portable Commit Snapshot

A `docs-hygiene.snapshot.v1` manifest identifies one exported artifact by a
stable credential-free repository identity and full commit OID. Ordered entries
bind a governed target, original repository path, local payload path, file or
block scope, optional block locator, and SHA-256 digest. Commit scope is not a
portable entry scope.

A frontmatter anchor may carry typed `snapshot` provenance containing manifest
identity, repository identity, commit OID, and original path. Offline validation
requires those claims to equal one registered entry, verifies the local payload
with ordinary file/block semantics, and still verifies the current canonical
target. `DH_SNAPSHOT_001` through `DH_SNAPSHOT_007` separate registration,
repository, commit, path, digest, signature, and lifecycle failures.

An optional Ed25519 signature covers compact UTF-8 JSON of every manifest field
except `signature`, in schema field order. Trusted public keys are project
policy; repository values reject URLs and credential-bearing syntax. Present
signatures are always verified and policy may require them.

Only `active` snapshots prove anchors. `replaced` snapshots name a registered
active successor; replaced and `revoked` snapshots declare `retainUntil`.
External history remains audit evidence rather than semantic authority.

`import-snapshot` is a read-only `docs-hygiene.snapshot-import.v1` plan unless
`--apply` is explicit. It reads the exact declared commit from a user-supplied
local Git checkout, verifies every entry, and atomically writes payloads. It
never performs network acquisition or changes the signed manifest. Normal
`check` performs no Git or network operation for portable snapshots.
