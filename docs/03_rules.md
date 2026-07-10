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

## I18n

`DH_I18N_001` reports a root docs file without a localized counterpart.

`DH_I18N_002` reports a localized docs file without a root counterpart.

## Language

`DH_LANG_001` reports a document below its configured minimum CJK ratio.

`DH_LANG_002` reports a document above its configured maximum CJK ratio.

## Concepts

`DH_CONCEPT_001` reports a highlighted concept reference without a concept
definition file.

`DH_CONCEPT_002` reports a concept definition file that is not referenced by
docs.

## Adapters

`DH_ADAPTER_001` reports an external adapter failure.
