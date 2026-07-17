---
id: SPEC-003-C-004
status: baselined
---

# C-004 兼容性

首个画像 Schema 与诊断和激活 Schema 分别独立版本化。该版本保持现有规则 ID、诊断码、
`required`、`disabled` 行为及 checker 严重程度不变。

旧文档契约成熟度只映射结构目标：`seed` 映射基础，`growing` 和 `maintained` 映射受控，
`governed` 映射治理。它不能设置身份、依赖或拓扑目标。显式新结构目标与旧值冲突时产生
可操作迁移错误，不允许静默覆盖。
