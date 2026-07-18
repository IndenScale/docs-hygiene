---
id: FEATURE-010
epic: EPIC-003
status: baselined
delivery_status: delivered
depends_on: [FEATURE-009]
---

# FEATURE-010 累积成熟度画像

## 能力边界

结构、身份、依赖和拓扑分别记录 applicability、target、detected maturity 和有序证据；
总体等级只取适用且必需维度的最低结果。

## 验收

- G2/G3 必须包含所有适用低等级不变量；
- N/A 保留理由且不参与总体等级；
- 目标成熟度与检测成熟度分别报告。

## 交付证据

`src/profile.rs`、`src/profile/`、`docs-hygiene.yml` 及 profile 测试。
