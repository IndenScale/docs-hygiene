---
id: FEATURE-010
status: baselined
delivery_status: delivered
---

# FEATURE-010 Kind-aware Scaffold 与类型化 Frontmatter Schema

## 问题

当前 `scaffold` 只生成固定 starter tree；Document Contract 的 `requiredFields` 是正则，
无法表达字段类型、枚举、条件必填或字段间不变量。创建文档与校验文档使用了两套知识。

## 目标

- 建立 Document Kind 注册表，聚合文件命名、slug、frontmatter Schema、章节结构、模板
  revision 和适用路径。
- 提供按 Kind 创建文档的 CLI，例如指定 Kind、身份、locale 和目标目录后生成合法骨架。
- Frontmatter 支持类型、枚举、格式、必填、禁止未知字段及条件不变量。
- 必要标题和顺序继续使用稳定语义 section ID；生成器与检查器消费同一 Schema。
- 支持 dry-run、冲突检测和兼容迁移，不得默认覆盖现有文件。

## 验收标准

- 每个受治理 Kind 至少有一个生成成功和一个非法输入测试。
- 新生成文件在无需人工补结构的情况下通过对应契约；业务内容占位符仍受成熟度策略治理。
- Schema revision 与模板 revision 的兼容、迁移和失败原子性有明确契约。
- 现有开放契约继续允许额外正文章节；frontmatter 是否允许扩展由 Kind 显式声明。

## 依赖

基于 [FEATURE-005](05_document-contracts-and-template-lifecycle.md)，并与
[FEATURE-009](09_slug-identity-governance.md) 共同定义 Kind 注册表边界。

## 交付证据

- `src/config/document_kinds.rs` 定义 `documentKinds` 注册表，把 base、文件名 pattern、
  Profile/Template 契约、脚手架和带 revision 的 frontmatter Schema 聚合到稳定 Kind ID；
- `src/document_kinds/` 让生成器与检查器共享字段类型、枚举、格式、默认值、来源、未知
  字段策略、条件必填/禁止、字段间不变量以及稳定 section ID；
- `docs-hygiene scaffold --kind ...` 支持 identity、slug、locale、target、重复 `--field`、
  `--dry-run` 和显式 `--force`；生成前完成路径、命名、Profile 和 Schema 校验，默认不覆盖；
- `DH_KIND_001`、`DH_KIND_002` 与 `DH_FRONTMATTER_001` 提供注册表、revision 和字段级
  确定性诊断，`structure.kind-schema` 已进入多维画像不变量注册表；
- `docs-hygiene migrate-kinds` 用 `docs-hygiene.kind-migration.v1` 联合规划兼容的文档
  Schema revision 与 Profile Template pin；任一不兼容或非法文档阻止全部写入；
- `tests/kind_scaffold.rs` 覆盖 canonical/localized 成功生成、开放正文、五类字段、枚举、
  格式、未知字段、条件、字段间不变量、dry-run、冲突与非法输入；
- `tests/kind_migration.rs` 覆盖只读计划、联合迁移、迁移后通过契约和失败原子性；完整
  配置与 CLI 契约见[Document Kind 注册表](../../14_document_kinds.md)。
