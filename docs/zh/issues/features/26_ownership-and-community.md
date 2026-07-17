---
id: FEATURE-026
epic: EPIC-007
status: baselined
delivery_status: delivered
depends_on: [FEATURE-018, FEATURE-025]
---

# FEATURE-026 Owner、复核日落与知识社区冗余

## 能力边界

每个已建立身份声明可解析 Owner、review deadline 和至少两名不同 active person 的理解确认；
Reset 是显式人工动作并留下审计，群组不能冒充两个人。

## 验收

- 缺 Owner、临期/过期复核、理解不足和确认过期分别诊断；
- Reset 默认只读，非法 actor/reason/deadline 保持零写入；
- successor 独立满足责任和知识冗余，画像报告 coverage 与 bus factor。

## 交付证据

`src/checks/ownership.rs`、`src/ownership.rs`、`tests/review_reset.rs` 和 profile tests。
