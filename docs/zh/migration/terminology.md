# 术语迁移

开放工程资产模型移除了固定精化轴。

| 已移除模型 | 当前模型 |
| --- | --- |
| Intent 精化层 | UL 与 PRD 长期文档结构 |
| Definition / Spec / Glossary 层 | PRD 约束加 Issue 范围内验收 |
| Implementation / SDK 层 | 位置无关的 Artifact 证据 |
| `refinementLevel` | 已移除 |
| `formalizes`、`realizes`、`projects` | 已移除 |
| `requireCompleteVerticalDerivation` | 已移除 |

已移除字段会被拒绝，而不是静默忽略。把可复用含义迁入 UL，把长期产品需求迁入 PRD，
把变更范围内的约束、验收、协作与交付证据迁入 Issue。代码、测试、配置、SDK 内容、
生成物与提交可以留在项目拥有它们的位置。

其他早期配置更名保持不变：文件名 pattern 的 `role` 已改为 `documentKind`，语言配置使用
`languageRepresentations`，仓库规模阈值使用项目规模名称。
