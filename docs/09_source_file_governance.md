# Source File Governance

Source files become difficult to review and unsafe to change when unrelated
responsibilities accumulate in one place. This repository therefore applies a
line-budget gate to tracked and unignored text source files.

## Thresholds

- More than 500 lines emits a warning and asks maintainers to plan a split.
- More than 1,000 lines emits an error and blocks CI.

Warnings are advisory. Errors have no legacy allowlist: an oversized file must
be split by responsibility instead of receiving a permanent exception.

## Local Check

Run the same gate used by CI:

```bash
./scripts/check-file-size.sh
```

The checker covers common code, script, configuration, and Markdown extensions.
It reads Git's tracked and unignored file set, so build output and ignored
dependencies do not create noise.

## Module Boundaries

Split by policy responsibility, not by arbitrary line ranges. The checker uses
separate units for package validation, Wiki Link validation, derivation rules,
document contracts, repository checks, and shared support. The parent module
keeps orchestration and includes those private units without expanding the
crate's public API.

When a unit crosses 500 lines, first extract cohesive data parsing, validation,
or reporting behavior with focused tests. A 1,000-line failure is a design
defect, not a request to raise the threshold.
