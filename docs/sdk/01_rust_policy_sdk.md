---
id: SDK-001
status: current
glossary: GLOSSARY-001
glossary_version: 0.1.0
crate_version: 0.1.0
---

# Rust Policy SDK

## Definition Source

SDK-001 projects GLOSSARY-001 identities into reusable Rust policy primitives.
Manifest graph validation is not part of the current public surface.

## Public Surface

The `docs_hygiene` Rust library exports `Config`, `run_checks`, `Report`,
`print_text_report`, and `print_json_report`. The CLI imports this surface rather
than owning a separate checker implementation. The repository implementation Body
declares this dependency in `implementation-manifest.yml`.

## Semantic Mappings

| Definition identity | SDK symbol |
| --- | --- |
| governed policy input | `Config` |
| policy evaluation | `run_checks` |
| recorded diagnostic outcome | `Report` |
| human-readable evidence adapter | `print_text_report` |
| machine-readable evidence adapter | `print_json_report` |

These mappings cover existing structural and document-contract checks. They do not
claim that cross-layer manifests or trace graphs are implemented.

## Evidence

`cargo test` verifies the shared library modules and CLI integration. Running
`cargo run -- check --fail-on-warning` against this repository verifies the configured
UL, PRD, Glossary, Spec, and SDK Reference document contracts.
