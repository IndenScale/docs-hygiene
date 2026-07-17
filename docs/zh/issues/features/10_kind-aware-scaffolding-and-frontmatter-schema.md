---
id: FEATURE-010
status: proposed
delivery_status: planned
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
