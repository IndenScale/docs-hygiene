# Rules

Docs Hygiene emits stable diagnostic codes. The first release keeps the rule
surface small and focused on repository hygiene.

## Entry Files

`DH_REQUIRED_001` reports a missing required file from `requiredFiles`.

## Numbered Docs

`DH_NAME_001` reports a Markdown file under the docs root whose file name does
not match `docs.filenamePattern`.

`DH_SEQ_001` reports a missing number in a numbered docs group.

`DH_SEQ_002` reports a duplicate number in a numbered docs group.

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

`DH_MATURITY_001` recommends a higher declared governance maturity from configured repository-size signals.

## Concepts

`DH_CONCEPT_001` reports a highlighted concept reference without a concept
definition file.

`DH_CONCEPT_002` reports a concept definition file that is not referenced by
docs.

## Repository Links

`DH_LINK_001` reports a repository-local inline Markdown Link, reference-style
link definition, or image target that does not resolve to an existing file or
directory. Fenced and inline code, same-document fragments, and external URI
schemes are excluded. External URL reachability remains an adapter concern.

## Governance Graph

`DH_GOVERNANCE_001` reports an unreadable or malformed manifest, duplicate
semantic identity, invalid lifecycle status, or the removed document-level
`version` or Manifest-level `references` fields.

`DH_REFERENCE_001` reports a missing, unresolved, refinement-invalid, or
content-hash-stale semantic Wiki Link to a Library identity.

`DH_LIBRARY_001` reports a missing, malformed, duplicate, or undeclared member
of a recursive Library tree, including removed leaf-level `version` or `source` metadata.

`DH_BODY_001` reports the same structural or localization failure in a directory
PRD or Spec Body Package, or a malformed, duplicated, unsafe, or missing declared
Implementation Body member. Removed leaf-level `version` or `source` metadata is
also invalid.

`DH_DERIVATION_001` reports a missing, unresolved, wrongly typed, or incomplete
adjacent-refinement-level Body derivation through `formalizes` or `realizes`.

`DH_DERIVATION_002` reports a missing, unresolved, wrongly typed, or incomplete
adjacent-refinement-level Library projection through `projects`.

## Adapters

`DH_ADAPTER_001` reports an external adapter failure.
