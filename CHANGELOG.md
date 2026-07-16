# Changelog

## Unreleased

- Add deterministic progressive rule activation with nine stable rule-family
  IDs, auto/required/disabled modes, four activation states, checker enforcement,
  and versioned `explain-rules` text and JSON output.
- Define Domain, Sub Domain, and direct-member fan-out budgets across UL,
  Glossary, the coding-agent Skill, and the policy engine; warn at 15 canonical
  direct members and fail at 50 by default with configurable thresholds.

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
