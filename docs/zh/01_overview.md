# 概览

Docs Hygiene 是面向仓库文档的 **Policy Engine**。

它在仓库边界维持文档卫生：检查必须存在的入口文件、编号文档结构、语言同位、长度预算和概念完整性。

Docs Hygiene 不替代通用 Markdown 或文风 linter。它直接检查需要仓库上下文的链接和文档契约，外部工具仍可通过 **Adapter** 配置接入。

## 产品边界

Docs Hygiene 负责需要仓库上下文的文档卫生规则：

- 公开入口文件完整性
- 根目录入口文档默认允许
- docs base 默认拒绝未知 Markdown 文件
- 编号文档结构和索引文件
- canonical 与本地化语言表示的同位
- 语言阈值检查
- 概念外键
- 死的语义 Wiki Link 和仓库内 Markdown Link 目标
- 受治理的 YAML frontmatter 契约
- 特定文档类型的 Profile 与递归 Package 目录结构
- 外部 adapter 编排

它不负责通用 Markdown 格式、外部 URL 爬取、拼写或文风；这些能力继续交给 markdownlint、lychee、Vale、cspell 或 slop-lint。这个边界不排除对仓库内 Markdown 目标、Wiki Link 身份、frontmatter 和文档结构的治理校验。
