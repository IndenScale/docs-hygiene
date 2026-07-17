---
id: FEATURE-017
epic: EPIC-005
status: baselined
delivery_status: delivered
depends_on: [FEATURE-005, FEATURE-007, FEATURE-015]
---

# FEATURE-017 Library Claim 与重复定义治理

## 能力边界

维护者显式把核心 claim 绑定到唯一 Library 身份和可选 block selector；已确认重复定义使用
forbidden、migrate 或 controlled excerpt 策略，相似扫描只产生候选。

## 验收

- 一个 claim 只能有一个有效权威；
- 过期迁移及未 Pin/漂移摘录分别诊断；
- 自然语言相似度不能直接阻断 CI。

## 交付证据

`src/checks/library_claims.rs`、`src/checks/library_claim_scan.rs` 及 claim tests。
