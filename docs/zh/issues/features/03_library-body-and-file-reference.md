---
id: FEATURE-003
epic: EPIC-001
status: baselined
delivery_status: delivered
depends_on: [FEATURE-001, FEATURE-002]
---

# FEATURE-003 Library/Body 内容模型与文件级引用

## 能力边界

Intent、Definition、Implementation 使用 Body 与 Library 组织内容；稳定 ID 可以作为文件级
引用目标解析。此处不要求 selector、hash、纵向依赖或图分析。

## 验收

- Package 成员、路径安全、身份唯一性和孤立成员可诊断；
- `[[ID]]` 可以确定性解析到 canonical 文件；
- 缺失或歧义目标不会产生伪边。

## 交付证据

`src/checks/package_structure.rs`、`src/checks/wiki_references.rs`、
`src/checks/governance_models.rs` 及 governance package 测试。
