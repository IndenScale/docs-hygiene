# Docs Hygiene CLI Playbook

## Command Selection

| Goal | Command | Mutation |
| --- | --- | --- |
| Audit one project | `check <root> --format json` | No, except enabled adapters may execute configured tools |
| Explain active rules | `explain-rules <root> --format json` | No |
| Explain a diagnostic | `explain <DH_CODE>` | No |
| Create a starter policy | `init --path <file>` | Yes |
| Create a starter tree | `scaffold <root>` | Yes |
| Inspect languages | `lang list --config <file>` | No |
| Change language policy | `lang add`, `lang remove`, `lang set-threshold` | Yes |

The project root is the governance boundary. In a monorepo, run one check per configured project rather than assuming the repository root is one Docs Hygiene scope.

## Diagnostic Families

- `DH_REQUIRED`, `DH_NAME`, `DH_SEQ`, `DH_SIZE`, `DH_ASCII`: repository and document surface structure.
- `DH_REPRESENTATION`, `DH_LANG`: canonical and localized representations.
- `DH_CONTRACT`, `DH_MATURITY`, `DH_ACTIVATION`: document profiles and progressive rule activation.
- `DH_CONCEPT`, `DH_LINK`: definitions and project-root-local links.
- `DH_GOVERNANCE`, `DH_REFERENCE`, `DH_LIBRARY`, `DH_BODY`, `DH_DERIVATION`: manifest identity, package membership, semantic references, and refinement traceability.
- `DH_DOMAIN`: canonical Library Domain direct-member fan-out budgets. Review semantic topology before introducing Sub Domains.
- `DH_ADAPTER`: external adapter failure.
- `DH_SUPPRESSION`: a diagnostic was suppressed by policy.

Always use `explain <code>` when the emitted message and related information do not make the invariant clear.

## Repair Order

1. Fix malformed configuration or manifests that prevent trustworthy discovery.
2. Fix missing identities, unsafe paths, and duplicate membership.
3. Fix broken references and derivation edges.
4. Fix canonical/localized structural parity.
5. Fix document contracts and surface hygiene.
6. Consider policy changes or suppressions only after verifying the rule is inappropriate for this project.

Do not create plausible-looking manifest identities or derivation links without evidence in the repository. Ask for the intended authority when multiple fixes would encode different governance decisions.

## Exit Status

Errors fail the command. Warnings remain advisory unless `--fail-on-warning` is supplied. When collecting JSON, preserve stdout as the report and treat the non-zero exit as a gate result, not as proof that JSON generation failed.
