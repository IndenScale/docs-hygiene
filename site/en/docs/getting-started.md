---
title: Getting Started
description: Build Docs Hygiene, generate the initial policy for your project, and run the first check.
---

# Getting Started

This page covers three things: building DH, generating the project policy, and running the first check.

## Prerequisites

- Rust 1.85 or later;
- A project directory managed with Git;
- The ability to run commands locally or in CI.

## Build the CLI

The current version is built from source:

```bash
git clone https://github.com/IndenScale/docs-hygiene.git
cd docs-hygiene
cargo build --release
```

The resulting binary is at `target/release/docs-hygiene`. You can use this path directly or add it to your own tools directory.

## Generate the Initial Policy for a Project

Navigate to the project you want to govern, then run:

```bash
/path/to/docs-hygiene scaffold .
```

The scaffold creates a minimal document structure and a `docs-hygiene.yml`. If the target files already exist, DH will not silently overwrite them.

## Run the First Check

```bash
/path/to/docs-hygiene check .
```

The default behavior is:

- `error` causes the command to return a failure;
- `warning` shows suggestions without blocking;
- `info` provides observations.

When you are ready to treat warnings as gate criteria as well, run:

```bash
/path/to/docs-hygiene check . --fail-on-warning
```

## Understand Why a Rule Is Activated

DH does not require a project to choose a uniform maturity level upfront. It determines rule applicability based on current project facts, then uses explicit policy to set enforcement strength.

```bash
/path/to/docs-hygiene explain-rules .
/path/to/docs-hygiene explain-rules . --format json
```

The output explains which facts were found, whether a rule is currently `inactive`, `advisory`, `warning`, or `error`, and whether the project configuration overrides the automatic determination.

## View the Governance Profile

```bash
/path/to/docs-hygiene profile .
```

The profile reports structure, identity, dependency, and topology capabilities separately, without compressing different concerns into a single vague score.

## Next Steps

- Read [How DH Works](how-it-works.md) to understand where findings come from;
- Read [Project Configuration](configuration.md) to align the check scope with your real project;
- Read [CI Integration](ci.md) to establish continuous feedback.
