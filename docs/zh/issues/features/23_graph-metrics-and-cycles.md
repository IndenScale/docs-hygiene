---
id: FEATURE-023
epic: EPIC-007
status: baselined
delivery_status: delivered
depends_on: [FEATURE-019, FEATURE-020]
---

# FEATURE-023 图指标、循环与影响闭包

## 能力边界

统一治理节点和边形成确定性有向图，报告节点、边、resolved/unresolved、孤立节点、relation
计数、强连通循环组和反向影响闭包。

## 验收

- 节点、边和循环输出顺序稳定；
- 自环和多节点 SCC 都被识别；
- 重复声明不会制造重复图边。

## 交付证据

`src/governance.rs`、`src/governance/topology.rs`、`src/governance/impact.rs` 及图测试。
