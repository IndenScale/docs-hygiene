---
id: FEATURE-007
epic: EPIC-002
status: baselined
delivery_status: delivered
depends_on: [FEATURE-004, FEATURE-005]
---

# FEATURE-007 Critical Pin 策略与更新工作流

## 能力边界

策略按 source/target Kind、relation、路径或身份选择关键依赖，要求允许算法、最小 scope、
whole-file 策略和审计年龄；普通依赖不承担 Pin 成本。

## 验收

- 缺 Pin、scope 不足、算法不允许、漂移、过期和损坏分别诊断；
- 更新默认只读，显式 apply 才原子写入并追加 JSONL 审计；
- 无效选择保持零写入。

## 交付证据

`src/checks/critical_dependencies.rs`、`src/pin_update.rs`、`src/repository_anchor.rs`、
`tests/pin_update.rs`。Repo scope 已由 FEATURE-006 接入同一策略与更新工作流，并以完整
commit OID 校验当前 tracked repository state。
