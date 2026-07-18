---
id: FEATURE-016
epic: EPIC-005
status: superseded
delivery_status: delivered
depends_on: [FEATURE-015]
superseded_by: FEATURE-027
---

# FEATURE-016 相邻层派生与 Library 投影

> 历史交付记录：该固定精化层契约已由 FEATURE-027 替代，不再代表当前产品模型。

## 能力边界

Definition Body formalizes Intent Body，Implementation Body realizes Definition Body；下游
Library projects 相邻上游 Library。跳层、反向和错关系边无效。

## 验收

- 六种 refinement/relation source 组合具有唯一策略；
- Body 派生和 Library 投影不能相互替代；
- 无依赖的最上游节点不会被要求声明伪边。

## 交付证据

`src/checks/derivation.rs` 的集中策略表及 derivation/dogfood 测试。
