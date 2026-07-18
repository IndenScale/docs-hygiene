# 概览

Docs Hygiene 是面向项目文档的 **Policy Engine**。

它在显式指定的项目根目录内守护文档不变式：检查必须存在的入口文件、编号文档结构、
语言同位、长度预算和概念完整性。

Docs Hygiene 不替代通用 Markdown 或文风 linter。它直接检查需要项目根目录上下文的
链接和文档契约，外部工具仍可通过 **Adapter** 配置接入。

## 产品边界

Docs Hygiene 负责需要项目上下文的文档卫生规则：

- 公开入口文件完整性
- 根目录入口文档默认允许
- docs base 默认拒绝未知 Markdown 文件
- 编号文档结构和索引文件
- canonical 与本地化语言表示的同位
- 语言阈值检查
- 概念外键
- 死的语义 Wiki Link 和项目根目录内 Markdown Link 目标
- 受治理的 YAML frontmatter 契约
- 特定文档类型的 Profile 与递归 Package 目录结构
- 从确定性项目事实派生的可解释渐进式激活
- 外部 adapter 编排

它不负责通用 Markdown 格式、外部 URL 爬取、拼写或文风；这些能力继续交给
markdownlint、lychee、Vale、cspell 或 slop-lint。这个边界不排除对项目根目录内
Markdown 目标、Wiki Link 身份、frontmatter 和文档结构的治理校验。

已交付的[文档卫生治理模型](11_hygiene_governance_model.md)分离能力维度、成熟度和执行
状态，同时保持上述产品边界。
