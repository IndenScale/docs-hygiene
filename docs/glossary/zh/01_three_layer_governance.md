---
id: GLOSSARY-001
version: 0.1.0
ul_registry: docs-hygiene
ul_version: 0.2.0
status: baselined
---

# 三层治理术语表

## 来源语义

本术语表从 UL 0.2.0 投影 `DH-THREE-LAYER-MODEL`、三个层次身份、
`DH-REFERENCE-LIBRARY`、`DH-GOVERNED-BODY` 和 `DH-EVIDENCE-PLANE`。

## 定义身份

| 身份 | 精确定义 |
| --- | --- |
| `layer.intent` | Body 声明预期结果、Library 提供产品含义的层次。 |
| `layer.definition` | Body 声明可证伪正确性、Library 提供精确规格术语的层次。 |
| `layer.implementation` | Body 兑现定义、Library 提供可复用实现原语的层次。 |
| `role.body` | 位于一个层次中的具体受管主张。 |
| `role.library` | 被同层多个 Body 消费的复用参考。 |
| `role.evidence` | 根据定义和意图收益评估特定实现版本的观察记录。 |
| `edge.references` | 同层 Body 指向 Library 的依赖。 |
| `edge.formalizes` | Definition Body 声明它使哪个 Intent Body 可证伪。 |
| `edge.realizes` | Implementation Body 声明它实现哪个 Definition Body。 |
| `edge.projects` | 下游 Library 身份声明其上游语义来源。 |
| `edge.verifies` | Evidence 声明它评估的 Definition 和实现版本。 |

## 投影规则

1. 每个身份保留 UL 来源和版本。
2. Definition 身份可以收窄表示，但不能静默改变含义。
3. 关系方向固定：Body 引用 Library；下游形式化、实现、投影或验证上游权威。
4. 即使后续资产被直接链接，缺失的中间关系仍然是缺失。
