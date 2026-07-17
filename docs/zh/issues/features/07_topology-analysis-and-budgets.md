---
id: FEATURE-007
status: baselined
delivery_status: delivered
---

# FEATURE-007 图拓扑分析与硬阈值预算

## 背景

统一治理边已形成确定性有向图。Docs Hygiene 可以计算 Fan-In、Fan-Out、孤立节点、循环组
和反向传递影响，并以显式阈值阻断不健康拓扑。本票据逆向记录已交付的硬预算能力。

## 已交付范围

- 度数按不同邻居身份计算，重复链接和并行边不会放大 Fan-In/Fan-Out。
- Tarjan 强连通分量产生确定性循环组，自环也视为循环。
- `maxFanIn`、`maxFanOut` 和 `forbidCycles` 是显式、独立激活的拓扑策略。
- 超阈值产生 `DH_TOPOLOGY_001`，循环产生 `DH_TOPOLOGY_002`。
- 画像报告节点、边、度数、循环、孤立节点和反向传递影响。

## 交付证据

- `src/governance/topology.rs` 实现度数和强连通分量分析；
- `src/checks/topology.rs` 执行阈值和循环策略；
- `src/governance/impact.rs` 计算去重且 cycle-safe 的传递影响；
- `src/checks/tests/topology.rs` 与 `src/governance.rs` 测试确定性和门禁行为；
- 本仓库 dogfood `maxFanIn: 8`、`maxFanOut: 12`、`forbidCycles: true`。

## 未覆盖边界

当前阈值是全局硬限制，没有按节点声明的超级节点例外，也没有 owner、理由、到期时间或
健康度趋势。可审计例外由 [FEATURE-014](14_supernode-governance-exceptions.md) 承载。
