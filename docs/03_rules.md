# Rules

Docs Hygiene emits stable diagnostic codes. The first release keeps the rule
surface small and focused on project documentation hygiene.

## Entry Files

`DH_REQUIRED_001` reports a missing required file from `requiredFiles`.

## Numbered Docs

`DH_NAME_001` reports a Markdown file under the docs root whose file name does
not match `docs.filenamePattern`.

`DH_SEQ_001` reports a missing number in a numbered docs group.

`DH_SEQ_002` reports a duplicate number in a numbered docs group.

## Slug Identity

`DH_SLUG_001` reports an invalid or reserved slug, a normalized or case-folding
collision, an alias conflict, canonical/localized slug drift for one stable
identity, a missing authoritative source, or an incomplete rename policy.

## Size

`DH_SIZE_001` reports a docs file that exceeds `docs.maxLines`.

## ASCII Art

When `docs.forbidAsciiArt` is enabled, `DH_ASCII_001` reports consecutive ASCII art blocks in document prose. Fenced code blocks, ordinary Markdown tables, and horizontal rules are excluded.

## Language Representations

`DH_REPRESENTATION_001` reports a canonical document without a localized representation.

`DH_REPRESENTATION_002` reports a localized representation without a canonical document.

## Language

`DH_LANG_001` reports a document below its configured minimum CJK ratio.

`DH_LANG_002` reports a document above its configured maximum CJK ratio.

## Document Contracts

`DH_CONTRACT_001` reports a missing required semantic section.

`DH_CONTRACT_002` reports a missing required field.

`DH_CONTRACT_003` reports a declared placeholder in a required section.

`DH_CONTRACT_004` reports required sections in the wrong order.

`DH_TEMPLATE_001` reports an invalid template/profile identity, duplicate
declaration, unknown binding, invalid expression, or duplicate resolved member.

`DH_TEMPLATE_002` reports a configured template with no profile binding.

`DH_TEMPLATE_003` reports template revision metadata or a compatible profile
pin that needs migration. It blocks only at governed document-contract maturity.

`DH_TEMPLATE_004` reports a profile pin outside its template's declared
compatibility window and always blocks.

`DH_MATURITY_001` recommends a higher declared profile maturity from configured project-scale signals.

`DH_KIND_001` reports an inconsistent Document Kind registry binding.

`DH_KIND_002` reports a missing, compatible-stale, or incompatible typed
frontmatter Schema revision.

`DH_FRONTMATTER_001` reports a typed field, enum, format, unknown-field,
conditional, or cross-field invariant violation.

## Concepts

`DH_CONCEPT_001` reports a highlighted concept reference without a concept
definition file.

`DH_CONCEPT_002` reports a concept definition file that is not referenced by
docs.

## Project-Root Links

`DH_LINK_001` reports a project-root-local inline Markdown Link, reference-style
link definition, or image target that does not resolve to an existing file or
directory. Fenced and inline code, same-document fragments, and external URI
schemes are excluded. External URL reachability remains an adapter concern.

## Governance Graph

`DH_ACTIVATION_001` reports that project facts activated an unconfigured rule
family. The diagnostic includes the activation state, ordered evidence, and the
per-rule override path. Scale-only activation is informational.

`DH_GOVERNANCE_001` reports an unreadable or malformed manifest, duplicate
semantic identity, invalid lifecycle status, or the removed document-level
`version` or Manifest-level `references` fields.

`DH_REFERENCE_001` reports a missing, unresolved, refinement-invalid, or
content-hash-stale semantic Wiki Link to a Library identity.

`DH_SELECTOR_001` reports a semantic Wiki Link heading selector that does not
resolve in the canonical governed target.

`DH_LIBRARY_001` reports a missing, malformed, duplicate, or undeclared member
of a recursive Library tree, including removed leaf-level `version` or `source` metadata.

`DH_BODY_001` reports the same structural or localization failure in a directory
PRD or Spec Body Package, or a malformed, duplicated, unsafe, or missing declared
Implementation Body member. Removed leaf-level `version` or `source` metadata is
also invalid.

`DH_CLAIM_001` reports an invalid core Library authority, confirmed forbidden
duplicate, expired migration, or missing/stale controlled-excerpt block pin.

`DH_PIN_001` through `DH_PIN_006` independently report missing Pins,
insufficient scope, disallowed algorithms, changed content, expired audit age,
and invalid policy or declaration state for critical dependencies.

`DH_SNAPSHOT_001` through `DH_SNAPSHOT_007` distinguish portable snapshot
registration, repository, commit, path, digest, signature, and lifecycle
failures without invoking a remote repository.

`DH_DERIVATION_001` reports a missing, unresolved, wrongly typed, or incomplete
adjacent-refinement-level Body derivation through `formalizes` or `realizes`.

`DH_DERIVATION_002` reports a missing, unresolved, wrongly typed, or incomplete
adjacent-refinement-level Library projection through `projects`.

`DH_TOPOLOGY_001` reports a governed identity whose distinct-neighbor Fan-In or
Fan-Out exceeds an explicit threshold. `DH_TOPOLOGY_002` reports a directed
cycle group when `forbidCycles` is enabled.

## Adapters

`DH_ADAPTER_001` reports an external adapter failure.
