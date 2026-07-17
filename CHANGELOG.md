# Changelog

## Unreleased

- Add versioned portable commit snapshot manifests, typed anchor provenance,
  offline file/block payload verification, Ed25519 trust policy, lifecycle and
  retention checks, and an explicit atomic `import-snapshot` workflow.

- Add critical dependency policies across normalized edge relations, stable Pin
  diagnostics, maximum-age audit metadata, reverse-impact evidence, and an
  explicit atomic `update-pins` plan/apply workflow with JSONL audit records.

- Add explicit `governance.coreClaims` Library authorities, confirmed duplicate
  policies, deadline migrations, block-pinned controlled excerpts, lifecycle
  remediation, and a non-blocking `scan-library-claims` candidate report.

- Add a `documentKinds` registry shared by typed frontmatter validation and
  locale-aware document scaffolding; support field types, enums, formats,
  conditions, cross-field invariants, explicit unknown-field policy, dry runs,
  conflict-safe writes, and atomic Schema/Template revision migration.

- Add opt-in, Document-Kind-scoped slug identity schemas with filename,
  frontmatter, or stable-ID authority; deterministic normalization, reserved
  and collision checks; localized identity parity; alias migration policy; and
  structured `DH_SLUG_001` JSON remediation data.

- Define the planned multidimensional documentation governance model in PRD-004:
  three maturity levels per capability dimension, independent rule execution
  states, and a unified typed edge abstraction for semantic dependencies.
- Add SPEC-003 with deterministic profile, N/A, exception, compatibility, edge,
  and delivery-slice semantics; centralize the nine compatibility rule families'
  metadata in the first implementation registry.
- Add atomic invariant registration and `docs-hygiene.profile.v1` text and JSON
  reports with target/observed maturity, execution decisions, N/A rationale,
  legacy mapping, suppression non-evidence, and optional below-target gating.
- Normalize semantic references, pinned references, derivations, and projections
  into ordered governance edge records; make derivation and staleness checks
  consume them; and expose deterministic basic graph metrics in the profile.
- Compute distinct-neighbor Fan-In/Fan-Out and deterministic directed cycle
  groups; add explicit `maxFanIn`, `maxFanOut`, and `forbidCycles` enforcement.
- Add reusable document-contract templates with stable bindings, deterministic
  template/profile merge, registry diagnostics, profile coverage evidence, and
  compatibility for existing inline profiles; migrate the repository policy to
  one shared template across all five profiles.
- Add governed template lifecycle with revision compatibility windows, exact
  profile pins, `DH_TEMPLATE_003/004`, versioned migration reports, read-only
  checks, and atomic compatible pin migration; raise the repository structure
  target to governed.
- Add semantic Wiki Link heading selectors with deterministic ATX slug
  resolution, `DH_SELECTOR_001`, localized signature parity, normalized edge and
  profile evidence, coexistence with whole-file pins, and repository dogfood.
- Add the versioned `docs-hygiene.reference-occurrence.v1` IR, shared Wiki Link,
  Markdown Link, and frontmatter collectors, explicit syntax/context policy, and
  syntax-independent edge normalization without changing edge JSON or diagnostics.
- Add frontmatter multi-anchor declarations, compatible file-scope JSON,
  deterministic ATX-block SHA-256 spans, scoped-anchor profile evidence,
  per-item diagnostics, and default-off local Git commit verification.
- Add ordered reverse transitive-impact sets across all resolved semantic edge
  kinds, with deduplication, cycle-safe traversal, profile evidence, repository
  dogfood, and a maximum-impact text summary.
- Add lifecycle status obligations and `supersededBy` authority migration for
  governed assets and package identities, localized parity, terminal-target
  rejection, ordered report evidence, and governed identity profile maturity.
- Add deterministic progressive rule activation with nine stable rule-family
  IDs, including opt-in `governance.topology`; retain auto/required/disabled
  modes, four activation states, checker enforcement, and versioned
  `explain-rules` text and JSON output.

- Replace the overloaded governance fields `layer` and `role` with
  `refinementLevel` and `referenceRelation`; replace filename-pattern `role`
  with `documentKind`.
- Promote language to an explicit governance dimension through
  `languageRepresentations.canonical` and `languageRepresentations.localized`;
  rename `lang add --root` to `--canonical` and i18n parity diagnostics to
  `DH_REPRESENTATION_001/002`.
- Document the three-dimensional model: refinement level, reference relation,
  and language representation.
- Add native governance manifests for stable-ID Intent, Definition, and
  Implementation assets.
- Model UL and Glossary as mandatory directory Libraries with one stable-ID
  Markdown file per term, and validate manifest membership.
- Add arbitrary-depth domain trees for UL and Glossary, plus directory PRD and
  Spec Body Packages with atomic, localized members.
- Derive horizontal same-refinement-level `Body -> Library` references from
  semantic Wiki Links in Body content.
- Remove document-level version fields; use Git for history and optional
  SHA-256 Wiki Link anchors for change-impact detection.
- Validate vertical adjacent-refinement-level Body derivation and Library projection,
  including reverse completeness for baselined assets.
- Add stable governance, reference, Body derivation, and Library projection
  diagnostics.
- Dogfood the repository policy and governance graph in the Rust test suite and
  GitHub Actions.
- Split the core checker into responsibility-focused private implementation
  units and add a repository file-size gate: more than 500 lines warns and more
  than 1,000 lines blocks CI.

## 0.1.0

- Initialize Docs Hygiene as a Rust CLI.
- Add required-file, docs naming, numbering, max-lines, language-representation parity, and concept foreign-key checks.
- Add text and JSON report formats.
- Infer document profiles from path and file-name conventions.
- Add maturity-aware required sections, required fields, ordered sections, and governed placeholders.
- Allow one contract to accept localized heading aliases while leaving additional sections open.
