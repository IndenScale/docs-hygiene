---
id: SPEC-003-C-003
status: baselined
---

# C-003 边规范化

语义引用、锁定引用、派生和投影规范化为有序边记录，包含来源身份、目标身份、关系类型、
来源位置、可选 selector、可选内容锚和生命周期出处。解析、过期检测、影响分析和拓扑
共同消费该记录。

Markdown 导航链接仍是路径完整性输入，除非显式声明语义关系，否则不会成为语义边。

引用声明表面先产出 [C-012](reference-occurrence-ir.md) 定义的带版本、语法中立
occurrence IR。一个 occurrence 是否进入该边模型，由显式策略而非 collector 语法决定。

期待类型、端点类型、显式解析结果和兼容分类遵循
[C-020](typed-reference-resolution.md)。标题 selector 存在时，按照 [C-011](selector-resolution.md)对 canonical 目标进行校验，
并保留在规范化边上。
