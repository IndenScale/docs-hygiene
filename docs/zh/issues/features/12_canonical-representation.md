---
id: FEATURE-012
epic: EPIC-004
status: baselined
delivery_status: delivered
depends_on: [FEATURE-001, FEATURE-003, FEATURE-010]
---

# FEATURE-012 Canonical 权威与语言表示模型

## 能力边界

配置明确 canonical language 和 localized representations；本地化表示投影 canonical 资产，
不声明竞争性的语义身份。

## 验收

- canonical 缺失和孤立 localized 分别诊断；
- 图节点按稳定资产身份去重；
- 语言代码只表示呈现方式，不改变 refinement level 或 reference relation。

## 交付证据

`languageRepresentations` 配置、representation checks、`lang` CLI 与本地化 fixture。
