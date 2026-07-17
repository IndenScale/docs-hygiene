---
id: FEATURE-009
epic: EPIC-003
status: baselined
delivery_status: delivered
depends_on: [FEATURE-001, FEATURE-002, FEATURE-003, FEATURE-004, FEATURE-005]
---

# FEATURE-009 原子不变量与能力维度

## 能力边界

单一注册表声明稳定不变量 ID、能力维度、最低成熟度、适用性证据和 checker 归属。

## 验收

- 一个不变量只属于一个维度和最低成熟度；
- 注册表顺序和输出确定；
- 诊断码或兼容规则族不冒充原子不变量。

## 交付证据

`src/profile/registry.rs`、`src/activation.rs`、SPEC-003 C-002/C-007 及 profile 测试。
