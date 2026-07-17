---
id: FEATURE-022
epic: EPIC-006
status: baselined
delivery_status: partial
depends_on: [FEATURE-021]
---

# FEATURE-022 统一引用兼容性校验

## 能力边界

单一兼容矩阵比较期待类型与解析类型，并统一处理 selector、anchor、lifecycle 和 relation
约束；具体 checker 只声明 expectation，不重复实现端点判断。

## 验收

- 缺失、歧义、错层、错关系、错 Kind 和终止目标使用稳定分类；
- 一次解析结果可被诊断、画像、影响和图分析复用；
- 新 relation 只扩展策略表而非复制 checker 分支。

## 当前差距

现有垂直派生和 Wiki/claim 检查具有局部兼容判断，但尚未收敛为共享矩阵。
