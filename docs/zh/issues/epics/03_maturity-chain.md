---
id: EPIC-003
status: baselined
delivery_status: delivered
depends_on: [EPIC-001]
---

# EPIC-003 治理成熟度依赖链

## 能力链

`Atomic Invariant → Capability Dimension → Cumulative Maturity → Execution → Migration`

成熟度评估能力证据，但不拥有具体业务规则。后续 Epic 把原子不变量登记到该框架，
执行严重程度不能伪造成熟度通过。

## Features

1. [FEATURE-009 原子不变量与能力维度](../features/09_atomic-invariants.md)
2. [FEATURE-010 累积成熟度画像](../features/10_cumulative-maturity-profile.md)
3. [FEATURE-011 执行状态、例外与兼容迁移](../features/11_execution-and-migration.md)

## Epic 验收

- 每个不变量只属于一个维度和最低成熟度；
- 高等级必须建立在所有适用低等级证据之上；
- disabled、suppressed 和未执行不能被报告为 Passed。
