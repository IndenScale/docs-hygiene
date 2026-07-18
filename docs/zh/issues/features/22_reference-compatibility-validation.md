---
id: FEATURE-022
epic: EPIC-006
status: baselined
delivery_status: delivered
depends_on: [FEATURE-021]
---

# FEATURE-022 统一引用兼容性校验

## 能力边界

单一兼容矩阵比较期待类型与解析类型，并统一处理 selector、anchor、lifecycle 和 relation
约束；具体 checker 只声明 expectation，不重复实现端点判断。

## 验收

- 缺失、歧义、错关系、错 Kind 和终止目标使用稳定分类；
- 一次解析结果可被诊断、画像、影响和图分析复用；
- 新 relation 只扩展策略表而非复制 checker 分支。

## 交付证据

`resolve_reference` 集中比较 reference relation、Document Kind 和
lifecycle；selector/anchor 结果进入同一 `ReferenceResolution.incompatibilities`。
诊断、画像、传递影响、Fan、循环和社区分析复用该结果，相关治理图与 Reference IR
测试覆盖稳定分类和不兼容边隔离。
