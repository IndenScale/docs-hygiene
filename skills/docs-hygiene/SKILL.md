---
name: docs-hygiene
description: Audit, explain, configure, and repair project documentation with the Docs Hygiene CLI. Use when a repository has `docs-hygiene.yml`, when users mention documentation governance, Domain or Sub Domain organization, Library fan-out budgets, document contracts, localization parity, governance manifests, Wiki Link references, traceability, DH_* diagnostics, or when setting up Docs Hygiene in a new project or CI workflow.
---

# Docs Hygiene

Use Docs Hygiene as the deterministic authority. Inspect policy and machine-readable diagnostics before changing documents; never infer that prose is semantically equivalent merely because it looks similar.

## Establish the Library Topology Model

Use these concepts whenever UL or Glossary organization is in scope:

- A **Domain** is a stable semantic boundary inside a Library. Its `manifest.yml` declares `kind: domain`, a stable identity, lifecycle status, and direct members.
- A **Sub Domain** is a Domain nested below another Domain. It is the same governed node type, not a separate refinement level or reference relation.
- A **fan-out budget** limits the direct members of one canonical Library Domain. It measures reviewable topology, not semantic correctness, and never counts the whole descendant tree.

The default budget is warning at 15 direct members and error at 50. Count both Markdown leaves and child Domains as one direct member each. Inspect only the canonical tree so localized representations do not duplicate diagnostics.

Treat `DH_DOMAIN_001` as a request to review topology. Propose candidate Sub Domains from repository evidence such as terminology clusters, ownership, and downstream projection paths. Do not automatically move members or invent Domain identities from the count alone.

## Resolve the CLI

Use `scripts/run-docs-hygiene.sh` from this skill. It prefers an installed `docs-hygiene` binary and otherwise runs the Rust source bundled with the plugin. If neither is available, report that the CLI must be installed or the full plugin repository must be used.

## Audit Workflow

1. Resolve the project root from the user's explicit path. Otherwise use the nearest ancestor containing `docs-hygiene.yml`; do not silently check an entire monorepo.
2. Read `docs-hygiene.yml` before proposing changes. Treat rule modes, language representations, suppressions, document contracts, governance manifests, and adapters as project policy.
3. Inspect activation decisions:

   ```bash
   scripts/run-docs-hygiene.sh explain-rules <project-root> --format json
   ```

4. Run the audit in JSON for agent consumption:

   ```bash
   scripts/run-docs-hygiene.sh check <project-root> --format json
   ```

5. Group diagnostics by code and affected asset. For an unfamiliar code, run:

   ```bash
   scripts/run-docs-hygiene.sh explain DH_CODE
   ```

6. Apply the smallest policy-consistent repair. Preserve document identity, canonical/localized structure, manifest membership, and adjacent refinement links.
7. Re-run the focused check, then the complete project check. Use `--fail-on-warning` only when the user requests a warning gate or when reproducing CI.

Read [references/cli-playbook.md](references/cli-playbook.md) for command selection, safety boundaries, and diagnostic families.

## Setup and Policy Changes

- Use `init --path <file>` only when the user asks to create a starter policy.
- Use `scaffold <path>` only for an explicitly selected new or empty project. Do not add `--force` without explicit authorization because it can overwrite files.
- Use `lang list` for inspection. Treat `lang add`, `lang remove`, and `lang set-threshold` as policy mutations requiring an explicit user request.
- Explain a proposed suppression and its scope before adding it. Prefer fixing a broken invariant over suppressing the diagnostic.
- Warn before running enabled adapters from an untrusted repository because their commands come from project configuration.

## Interpretation Boundaries

Docs Hygiene checks deterministic structure, identity, references, representations, contracts, and traceability. Do not claim it verifies factual correctness, natural-language equivalence, translation freshness, external URL reachability, spelling, or business consistency.
