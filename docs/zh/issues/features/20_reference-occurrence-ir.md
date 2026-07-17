---
id: FEATURE-020
epic: EPIC-006
status: baselined
delivery_status: delivered
depends_on: [FEATURE-015, FEATURE-016]
---

# FEATURE-020 语法中立 Reference Occurrence IR

## 能力边界

Wiki Link、Markdown Link 和 frontmatter collector 只产出带版本的 ReferenceOccurrence；
显式 `(syntax, context)` 策略决定 semantic dependency、navigation 或 identity declaration。

## 验收

- 新语法只需新增 collector 和策略条目；
- 未知语法/上下文不产生伪语义边；
- 公开 edge JSON 保持兼容。

## 交付证据

`src/reference.rs`、`src/checks/reference_collectors.rs`、
`src/checks/reference_normalization.rs` 及 reference IR tests。
