---
id: FEATURE-024
epic: EPIC-007
status: baselined
delivery_status: delivered
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

## 交付证据

`src/governance/topology.rs` 对去重后的已解析类型化边运行确定性的无向桥分解：删除桥后
的连通分量形成社区，最小成员提供稳定 ID，无随机种子。画像报告社区成员、跨社区边和
基线变化；`communityBaseline` 默认仅报告，只有 `enforceCommunityBaseline: true` 才以
`DH_TOPOLOGY_006` 阻断。governance/topology 测试覆盖重复边、稳定边界及显式执行策略。
