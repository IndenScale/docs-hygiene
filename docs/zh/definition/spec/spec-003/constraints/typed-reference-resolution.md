---
id: SPEC-003-C-020
status: baselined
---

# C-020 类型化引用解析

每条规范化治理边都保留可序列化的 `expectation` 和 `resolution`。expectation 包含边关系、
允许的目标 refinement level、reference relation 和可选 Document Kind；resolution 包含
显式 `resolved`、`unresolved`、`ambiguous` 或 `incompatible` 结果，以及类型化候选端点、
位置、生命周期和有序不兼容分类。

单一兼容矩阵分类目标缺失/歧义、精化层级、引用关系和 Document Kind 不匹配、终止状态、
selector 失败与 anchor 失败。垂直目标可以声明期望的 `documentKind`，frontmatter anchor
可以声明 `expectedDocumentKind`。字段缺失表示关系策略不限制 Kind，不表示解析端点丢失
实际 Kind。

诊断、画像、影响分析、Fan、循环和社区分析复用同一记录。未解析、歧义、类型不兼容和
终止端点不进入拓扑或影响。selector/anchor 证据过期时，已解析依赖仍参与影响传播，但
有序兼容问题使该证据不能证明新鲜度。

新增 relation 只扩展 expectation 策略，不在 collector 或具体 checker 中复制端点比较。
