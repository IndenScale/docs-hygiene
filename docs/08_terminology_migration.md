# Terminology Migration

The three-dimensional software-documentation profile introduces breaking configuration names:

| Previous name | Current name |
| --- | --- |
| governance asset `layer` | `refinementLevel` |
| governance asset `role` | `referenceRelation` |
| filename-pattern `role` | `documentKind` |
| `i18n.rootLang` | `languageRepresentations.canonical` |
| `i18n.languages` | `languageRepresentations.localized` |
| `i18n.requireDocsParity` | `languageRepresentations.requireDocumentParity` |
| `lang add --root` | `lang add --canonical` |
| `minRepositoryLines` | `minProjectLines` |
| `minRepositoryBytes` | `minProjectBytes` |

The previous names are no longer accepted. Update configuration, manifests, automation, and diagnostic consumers together; language-representation parity diagnostics are now `DH_REPRESENTATION_001/002`.
