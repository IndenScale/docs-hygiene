---
id: FEATURE-002
epic: EPIC-001
status: baselined
delivery_status: delivered
depends_on: [FEATURE-001]
---

# FEATURE-002 内容契约、Schema 与脚手架

## 能力边界

Document Kind 注册表统一内容 Profile、Template revision、类型化 frontmatter、稳定章节、
脚手架和原子迁移，创建与检查不维护两套知识。

## 验收

- 每个受管 Kind 都有合法生成和非法输入测试；
- 章节、字段、顺序、条件不变量和未知字段策略可执行；
- 迁移默认只读，失败时整批零写入。

## 交付证据

`src/config/document_contracts.rs`、`src/config/document_kinds.rs`、`src/document_kinds/`、
`src/template_migration.rs`、`src/kind_migration.rs` 及 CLI 测试。
