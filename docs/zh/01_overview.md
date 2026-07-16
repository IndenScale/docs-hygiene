# 概览

Docs Hygiene 是面向仓库文档的 **Policy Engine**。

它在仓库边界维持文档卫生：检查必须存在的入口文件、编号文档结构、语言同位、长度预算和概念完整性。

Docs Hygiene 不替代 Markdown 或文风 linter。语法、风格和链接检查应通过 **Adapter** 配置交给外部工具。

## 产品边界

Docs Hygiene 负责需要仓库上下文的文档卫生规则：

- 公开入口文件完整性
- 根目录入口文档默认允许
- docs base 默认拒绝未知 Markdown 文件
- 编号文档结构和索引文件
- canonical 与本地化语言表示的同位
- 语言阈值检查
- 概念外键
- 外部 adapter 编排

它不负责 Markdown 格式、坏链爬取、拼写或文风。这些能力应继续交给 markdownlint、lychee、Vale、cspell 或 slop-lint。
