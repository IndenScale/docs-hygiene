---
id: SPEC-003-C-013
status: baselined
---

# C-013 Scoped Content Anchors

A normalized content anchor contains `algorithm`, `digest`, `scope`, and an
optional `locator`. Supported combinations are:

| Scope | Algorithm | Locator | Verification |
| --- | --- | --- | --- |
| `file` | `sha256` | forbidden | exact bytes of the complete canonical target |
| `block` | `sha256` | required heading slug | exact UTF-8 bytes of one canonical ATX heading section |
| `repo` | `git` | forbidden | complete tracked repository state equals the full Git commit OID |

`file` is the compatibility default. Its `scope` and absent `locator` are
omitted from JSON, so existing file-anchor edge records remain schema
compatible. Existing inline `[[ID#selector@sha256:<digest>]]` syntax still
selects a heading while anchoring the whole file. It never silently changes to
block scope.

Multiple explicit anchors use Markdown frontmatter:

```yaml
anchors:
  - target: TERM-1
    algorithm: sha256
    digest: <64-hex>
    scope: block
    locator: normative-behavior
  - target: TERM-2
    algorithm: sha256
    digest: <64-hex>
    scope: file
  - target: TERM-3
    algorithm: git
    digest: <full-40-or-64-hex-commit-oid>
    scope: repo
```

Each list item emits one `frontmatter` / `governedAnchor` reference occurrence
under [C-012](reference-occurrence-ir.md), and therefore one independently
ordered pinned edge. Invalid or stale items report `DH_REFERENCE_001` at their
own list-item line. Canonical and localized representations preserve target,
selector, algorithm, digest, scope, and locator signatures.

A block begins at the uniquely resolved ATX heading line and ends immediately
before the next ATX heading of the same or a higher level, or at end of file.
The hash covers that exact raw byte span. Changes outside the span do not stale
the anchor. Heading lookup and ambiguity follow
[C-011](selector-resolution.md).

Repo verification is disabled by default. It requires:

```yaml
governance:
  contentAnchors:
    verifyGitCommits: true
```

When enabled, the checker proves that the supplied full object ID resolves to a
commit and compares that commit with the complete current tracked repository
state. Added, deleted, mode-changed, staged, or modified tracked paths stale the
anchor; untracked paths do not. A repo anchor without opt-in is an error and
never invokes Git. Git remains physical audit evidence; stable governance IDs
and canonical content remain semantic authority. Cross-repository objects,
default repo anchoring, and automatic digest migration are outside this
constraint. `scope: commit` is invalid and is not a compatibility alias.
