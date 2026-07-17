---
id: SPEC-003-C-007
status: proposed
---

# C-007 Atomic Invariant Ledger

Registry invariants are finer than compatibility families and diagnostic codes.
A diagnostic may provide failure evidence for multiple invariants, but every
invariant has one dimension and minimum maturity.

| Invariant | Minimum | Current evidence | Delivery |
| --- | --- | --- | --- |
| `structure.entry-docs` | basic | `DH_REQUIRED_001` | delivered |
| `structure.naming-sequence` | basic | `DH_NAME_001`, `DH_SEQ_*` | delivered |
| `structure.local-links` | basic | `DH_LINK_001` | delivered |
| `structure.contracts` | controlled | `DH_CONTRACT_*` | delivered |
| `structure.content-policy` | controlled | `DH_SIZE_001`, `DH_ASCII_001` | delivered |
| `structure.reusable-templates` | controlled | registry bindings, `DH_TEMPLATE_*` | delivered |
| `structure.template-migration` | governed | revision windows, `DH_TEMPLATE_003/004`, migration CLI | delivered |
| `structure.kind-schema` | governed | typed frontmatter, Kind scaffold, atomic migration | delivered |
| `identity.stable-ids` | basic | `DH_GOVERNANCE_001`, package checks | delivered |
| `identity.duplicates` | basic | governance and Library indexes | delivered |
| `identity.library-claims` | governed | explicit Library authority and confirmed occurrence policy | delivered |
| `identity.canonical-source` | controlled | representation parity | delivered |
| `identity.slug-schema` | controlled | kind-scoped index, `DH_SLUG_001` | delivered |
| `identity.semantic-reference` | controlled | `DH_CONCEPT_*`, `DH_REFERENCE_001` | delivered |
| `identity.lifecycle` | governed | status obligations and terminal-target rejection | delivered |
| `identity.authority-migration` | governed | `supersededBy`, successor validation, ordered migration evidence | delivered |
| `dependency.resolve` | basic | Wiki Link and vertical target resolution | delivered |
| `dependency.typed-edges` | controlled | ordered normalized edge records | delivered |
| `dependency.content-anchor` | controlled | whole-target SHA-256 | delivered |
| `dependency.target-staleness` | controlled | hash mismatch diagnostics | delivered |
| `dependency.selector` | governed | edge selector, `DH_SELECTOR_001`, localized signature parity | delivered |
| `dependency.scoped-anchor` | governed | file/block/commit scopes, per-item diagnostics, opt-in Git verification | delivered |
| `dependency.critical-pins` | governed | edge matchers, `DH_PIN_*`, audited update workflow | delivered |
| `dependency.portable-snapshot` | governed | offline payloads, typed provenance, signatures, `DH_SNAPSHOT_*` | delivered |
| `dependency.transitive-impact` | governed | deterministic reverse reachability across all resolved semantic edges | delivered |
| `topology.metrics` | basic | normalized node, edge, resolution and isolation counts | delivered |
| `topology.fan-and-cycles` | controlled | distinct-neighbor degree and deterministic SCCs | delivered |
| `topology.thresholds` | controlled | `DH_TOPOLOGY_001`, `DH_TOPOLOGY_002` | delivered |
| `topology.budgets` | governed | global/exception budgets, remaining capacity, `DH_TOPOLOGY_001/003` | delivered |
| `topology.public-exceptions` | governed | exact node/direction audit declarations, `excepted` evidence | delivered |
| `topology.trends` | governed | ordered degree history and trend delta, `DH_TOPOLOGY_005` | delivered |

`DH_MATURITY_001`, `DH_ACTIVATION_001`, adapter failures, and suppression metadata
are execution or recommendation evidence and do not directly prove maturity.
The ledger is updated when an end-to-end fixture changes a delivery value.
