---
id: PRD-004-FR-001
status: baselined
---

# 统一治理边

检查器应把语义引用与锁定引用规范化为类型化依赖边模型。Issue 适配器可以增加
`addresses`、`dependsOn` 与 `evidencedBy` 关系，而不规定仓库拓扑。目标解析、
selector、锁定、生命周期、影响和拓扑策略应复用该模型。用于导航的 Markdown Link
仍作为路径完整性输入，不能仅因自身是链接就成为语义依赖。
