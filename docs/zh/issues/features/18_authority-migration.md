---
id: FEATURE-018
epic: EPIC-005
status: baselined
delivery_status: delivered
depends_on: [FEATURE-014, FEATURE-015, FEATURE-016]
---

# FEATURE-018 生命周期与权威迁移

## 能力边界

所有治理身份使用统一 lifecycle；superseded 声明同层、同关系且已建立的 successor，终止
身份不能继续作为当前治理边目标。

## 验收

- 非法状态、缺失/错误 successor 和陈旧消费方分别诊断；
- authority migration 有序进入报告；
- localized successor 保持同位。

## 交付证据

`src/checks/lifecycle.rs`、`LifecycleStatus`、governance graph 和 lifecycle tests。
