---
title: CI Integration
description: Run Docs Hygiene in continuous integration and produce stable JSON output for other tools.
---

# CI Integration

DH is designed to run both on developer machines and in CI. Both environments use the same project root and policy file, so the gate is never only explainable on the remote side.

## Basic Gate

```bash
docs-hygiene check . --fail-on-warning
```

If the project is in a gradual adoption phase, you can omit `--fail-on-warning` initially and let only errors block commits.

## GitHub Actions

```yaml
name: Docs Hygiene

on:
  pull_request:
  push:

jobs:
  docs-hygiene:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Build Docs Hygiene
        run: cargo build --release --locked
      - name: Check project documentation
        run: ./target/release/docs-hygiene check . --fail-on-warning
```

In a real project, it is recommended to pin the DH source version so that a tool upgrade and project document changes do not enter the same gate simultaneously.

## Machine-Readable Output

```bash
docs-hygiene check . --format json > dh-report.json
docs-hygiene explain-rules . --format json > dh-rules.json
docs-hygiene profile . --format json > dh-profile.json
```

JSON output is suitable for archiving, visualization, or consumption by other automation. External AI can analyze Info-level findings, but probabilistic judgment should not be disguised as a deterministic conclusion from DH.

## Working with Other Documentation Tools

DH does not replace existing tools:

| Tool type | Responsibility |
| --- | --- |
| markdownlint | Markdown syntax and formatting |
| lychee | External URL reachability |
| Vale, cspell | Style and spelling |
| Docs Hygiene | Identity, dependency, and traceability relationships that require project context |

Adapters can let DH invoke external tools during the same check, but each tool still maintains its own rule system.
