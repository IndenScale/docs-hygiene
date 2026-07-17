---
id: FEATURE-005
status: baselined
delivery_status: delivered
---

# FEATURE-005 文档契约、模板注册表与结构迁移

## 背景

Docs Hygiene 已能根据文档 Kind 的路径/文件名信号绑定开放契约，校验必要标题、字段、
顺序与占位符，并管理共享模板 revision。本票据逆向记录结构治理机制；通用 `scaffold`
仍不按 Kind 生成完整文档。

## 已交付范围

- Profile 声明 `requiredSections`、`requiredFields`、`orderedSections` 与成熟度门禁。
- 语义章节使用稳定 ID，并允许 canonical/localized 标题别名。
- Template 注册表提供共享契约、revision、兼容窗口和精确 Profile pin。
- `migrate-templates --check` 只读报告迁移；兼容迁移原子推进，不兼容时不写入。
- 画像报告区分模板复用、绑定覆盖和迁移证据。

## 交付证据

- `src/config/document_contracts.rs` 定义模板、Profile、章节与字段配置；
- `src/checks/document_contracts.rs` 执行章节、字段、顺序、占位符和成熟度检查；
- `src/checks/document_templates.rs` 解析模板注册表与绑定；
- `src/template_migration.rs` 实现 revision 迁移计划和原子更新；
- `src/checks/tests/template_lifecycle.rs`、`tests/template_migration.rs` 与 dogfood 测试覆盖闭环；
- Git 提交 `737d746` 对应最初的路径推导文档契约，后续模板生命周期由
  `10b84a7` 补齐。

## 后续闭环

本票据中的 `requiredFields` 仍是正文/原始文本正则。字段级类型 Schema，以及接受 Kind、
Profile 和模板 revision 的 scaffold 已由
[FEATURE-010](10_kind-aware-scaffolding-and-frontmatter-schema.md) 交付。
