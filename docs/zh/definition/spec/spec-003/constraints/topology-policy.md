---
id: SPEC-003-C-008
status: proposed
---

# C-008 拓扑策略

Fan-In 和 Fan-Out 统计不同且已解析的受管邻居身份；同一身份之间的重复引用和并行关系
类型不会放大度数。有向循环组是有序的强连通分量；自环是只有一个身份的循环组。

拓扑执行需要显式启用。只有明确配置 `maxFanIn`、`maxFanOut` 或 `forbidCycles` 才会激活
独立的 `governance.topology` 规则族；仅存在关系图或规模信号绝不会产生阻断性拓扑策略。

只有确定性的 Fan 与循环分析通过，且活跃的显式拓扑策略没有可见或被 suppression 隐藏
的违规时，拓扑才能达到受控成熟度。suppression 仍然不构成证据。

审计 supernode exception 遵循 [C-018](supernode-exceptions.md)。它可以让一个精确 Fan
违规不阻断，但阈值证据为 `excepted` 而非 Passed；governed 级预算、例外和趋势机制不会
覆盖 controlled 阈值证明义务。
