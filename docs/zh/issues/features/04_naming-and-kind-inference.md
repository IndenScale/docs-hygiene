---
id: FEATURE-004
status: baselined
delivery_status: delivered
---

# FEATURE-004 命名约定与 Document Kind 推导

## 背景

Docs Hygiene 已经把文档文件名从自由文本收敛为可配置规则，并让路径与文件名共同推导
`documentKind`。本票据逆向记录已经交付的命名治理基线，不把尚未存在的显式 slug 身份
误报为完成。

## 已交付范围

- 每个 docs base 可以声明多个命名 pattern；每个 pattern 包含稳定 ID、正则、
  `documentKind` 和是否编号。
- 未命中允许 pattern 的 Markdown 文件产生确定性错误。
- 连续编号、重复编号、本地化文件及编号同位可独立校验。
- Document Contract 可以同时按路径 glob 与文件名正则绑定，首个匹配 Profile 获得治理权。
- 标题 selector 使用确定性的 lowercase slug 解析，但它目前是块定位符，不是文件身份。

## 交付证据

- `src/config.rs` 定义 `DocsBaseConfig`、`FilenamePatternConfig` 与 `document_kind`；
- `src/checks/repository_structure.rs` 执行 pattern、编号与本地化结构检查；
- `src/checks/document_contracts.rs` 按路径和文件名推导 Profile；
- `src/checks/tests/documents.rs` 与 `src/checks/tests/policies.rs` 覆盖非法文件名、多个 base、
  重复编号和 Kind/Profile 推导；
- 本仓库 `docs-hygiene.yml` 对 numbered、index、library-term、body-item 等 Kind 进行 dogfood。

## 未覆盖边界

当前文件名正则可以约束“形状”，但没有独立的 slug Schema、slug 与稳定 ID 的对应关系、
重命名迁移规则或冲突索引。这些缺口由
[FEATURE-009](09_slug-identity-governance.md) 承载。
