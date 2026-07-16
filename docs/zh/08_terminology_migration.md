# 术语迁移

三维治理模型引入以下破坏性配置字段变更：

| 旧名称 | 当前名称 |
| --- | --- |
| 治理资产 `layer` | `refinementLevel` |
| 治理资产 `role` | `referenceRelation` |
| 文件名 pattern `role` | `documentKind` |
| `i18n.rootLang` | `languageRepresentations.canonical` |
| `i18n.languages` | `languageRepresentations.localized` |
| `i18n.requireDocsParity` | `languageRepresentations.requireDocumentParity` |
| `lang add --root` | `lang add --canonical` |
| `minRepositoryLines` | `minProjectLines` |
| `minRepositoryBytes` | `minProjectBytes` |

旧名称不再被接受。请同时更新配置、Manifest、自动化和诊断消费者；语言表示同位诊断现为 `DH_REPRESENTATION_001/002`。
