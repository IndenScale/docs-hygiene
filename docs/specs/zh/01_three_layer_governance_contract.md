---
id: SPEC-001
version: 0.1.0
status: baselined
prd: PRD-001
glossary: GLOSSARY-001
glossary_version: 0.1.0
---

# 三层治理契约

## 意图来源

本 Spec 形式化 PRD-001 的 FR-001 至 FR-007，并使用 GLOSSARY-001。

## 输入

受管资产提供 `id`、`layer`、`role`、`status` 和带版本的关系条目。关系使用
`edge.references`、`edge.formalizes`、`edge.realizes`、`edge.projects` 或
`edge.verifies`，并指向稳定目标及版本。

## 约束

1. `(layer.intent, role.body)` 接受 PRD；`(layer.intent, role.library)` 接受 UL。
2. `(layer.definition, role.body)` 接受 Spec 或 Test Definition；
   `(layer.definition, role.library)` 接受 Glossary。
3. `(layer.implementation, role.body)` 接受 Code 或 Configuration；
   `(layer.implementation, role.library)` 接受封装可复用实现原语的 SDK。
4. 每个 Body 都有指向同层 Library 的 `edge.references`。
5. 每个下游 Body 都有一个或多个类型化上游追溯边。
6. Glossary 和 SDK 身份使用 `edge.projects` 保留语义来源。
7. `role.evidence` 既不是 Body 也不是 Library，且必须指明 Definition 目标、
   实现版本、结果状态和 Intent 收益。
8. `baselined` 资产拒绝浮动版本和未声明、未解决的提案。

## 验证

Verifier 必须分别报告无效分类、缺少同层引用、缺少 Body 追溯、缺少 Library 投影和
缺少 Evidence。一个有效关系不能满足另一种缺失关系。当前文档契约测试验证仓库形态；
Manifest 和关系图诊断仍属于后续实现。
