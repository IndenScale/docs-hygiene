# Changelog

## Unreleased

- Add native governance manifests for versioned Intent, Definition, and
  Implementation assets.
- Model UL and Glossary as mandatory directory Libraries with one versioned
  Markdown file per term, and validate manifest membership.
- Add arbitrary-depth domain trees for UL and Glossary, plus directory PRD and
  Spec Body Packages with atomic, localized members.
- Validate horizontal same-layer `Body -> Library` references.
- Validate vertical adjacent-layer Body derivation and Library projection,
  including reverse completeness for baselined assets.
- Add stable governance, reference, Body derivation, and Library projection
  diagnostics.
- Dogfood the repository policy and governance graph in the Rust test suite and
  GitHub Actions.

## 0.1.0

- Initialize Docs Hygiene as a Rust CLI.
- Add required-file, docs naming, numbering, max-lines, i18n parity, and concept foreign-key checks.
- Add text and JSON report formats.
- Infer document profiles from path and file-name conventions.
- Add maturity-aware required sections, required fields, ordered sections, and governed placeholders.
- Allow one contract to accept localized heading aliases while leaving additional sections open.
