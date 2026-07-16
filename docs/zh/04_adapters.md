# Adapter

Docs Hygiene 通过 **Adapter** 配置调用外部文档工具。这样项目可以专注 policy，而成熟工具继续维护自己的规则生态。

## Markdownlint

第一版 adapter 是 markdownlint。

```yaml
adapters:
  markdownlint:
    enabled: true
    command: markdownlint-cli2
    args:
      - README.md
      - README_ZH.md
      - CHANGELOG.md
      - "docs/**/*.md"
```

Docs Hygiene 目前不解析 markdownlint 输出。如果命令以非零状态退出，Docs Hygiene 会输出 `DH_ADAPTER_001`，并把 adapter 输出放进诊断消息。

## 推荐边界

用 markdownlint 检查 Markdown 语法和格式规则。

用 Docs Hygiene 检查项目级 policy：

- 必需文档入口文件
- 编号文档结构
- 语言表示同位
- 语言阈值
- 概念外键
