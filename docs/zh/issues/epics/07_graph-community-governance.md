---
id: EPIC-007
status: baselined
delivery_status: partial
depends_on: [EPIC-006]
---

# EPIC-007 图、社区与 Fan-Out 治理

## 能力链

`Validated Edges → Graph → Communities/Cycles → Fan-In/Fan-Out → Budgets → Human Governance`

图只消费经过类型校验的边，不能通过拓扑结果倒推引用语义。图社区/模块边界与人员社区
责任是不同子能力，分别建模。

## Features

1. [FEATURE-023 图指标、循环与影响闭包](../features/23_graph-metrics-and-cycles.md)
2. [FEATURE-024 图社区与模块边界](../features/24_graph-communities.md)
3. [FEATURE-025 Fan-In/Fan-Out 预算与例外](../features/25_fan-budgets-and-exceptions.md)
4. [FEATURE-026 Owner、复核与知识冗余](../features/26_ownership-and-community.md)

## Epic 验收

- 重复链接和平行关系不放大度数；
- 社区或模块边界具有稳定、可解释的输出；
- 阈值、超级节点例外、Owner 和复核证据均可审计。

图、Fan 预算和人员责任已交付；图社区/模块边界尚未交付，因此 Epic 为部分交付。
