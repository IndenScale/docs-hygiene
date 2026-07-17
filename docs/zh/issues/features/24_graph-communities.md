---
id: FEATURE-024
epic: EPIC-007
status: proposed
delivery_status: planned
depends_on: [FEATURE-021, FEATURE-022, FEATURE-023]
---

# FEATURE-024 图社区与模块边界

## 能力边界

从已验证的类型化边识别稳定、可解释的社区或模块边界，并报告跨社区依赖，而不是用目录
结构或自然语言相似度替代图证据。

## 验收

- 算法、权重、随机种子或确定性 tie-break 明确；
- 重复链接不改变社区结果；
- 报告社区成员、跨界边和边界变化；
- 社区建议默认不直接阻断 CI，除非项目显式配置边界策略。

## 当前差距

当前实现只有 SCC、Fan 和影响分析，没有社区发现或模块边界模型。
