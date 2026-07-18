---
id: EPIC-005
status: baselined
delivery_status: delivered
depends_on: [EPIC-001, EPIC-004]
---

# EPIC-005 引用依赖链与 SSOT

## 能力链

`Semantic Reference → Authority Migration → Impact`

本 Epic 定义哪些依赖必须存在和如何保持完整，不在各 checker 内重复定义端点类型系统。

## Features

1. [FEATURE-015 Body → UL Library 引用](../features/15_same-level-reference.md)
2. [FEATURE-016 相邻层派生与投影（已由 FEATURE-027 替代）](../features/16_vertical-derivation-and-projection.md)
3. [FEATURE-017 Library Claim 与重复定义治理](../features/17_library-claim-governance.md)
4. [FEATURE-018 生命周期与权威迁移](../features/18_authority-migration.md)
5. [FEATURE-019 反向完整性与影响传播](../features/19_reverse-completeness-and-impact.md)

## Epic 验收

- 语义依赖的端点关系可独立校验；
- Library claim 只有一个有效权威；
- 终止目标和受影响消费方可确定性定位。
