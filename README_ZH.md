# Docs Hygiene

Docs Hygiene 是面向仓库文档的 policy-as-code 卫生检查器。它让文档保持整洁、完整、结构一致，并适合进入 CI 门禁。

它不是 Markdown 语法 linter。Markdown 格式应交给 markdownlint，链接检查应交给 lychee，文案风格应交给 Vale 或 cspell。Docs Hygiene 专注仓库级文档治理：

- README、CHANGELOG、LICENSE 等入口文件完整性
- `docs/` 下的编号文档结构
- 文档长度预算
- 根文档与本地化文档的 i18n 同位关系
- 基于路径与文件名推导、随项目成熟度增强的文档契约
- 从高亮术语到 `concept/*.md` 的概念外键
- 对 markdownlint 等外部工具的 adapter 编排

## 快速开始

```bash
cargo run -- check --fail-on-warning
```

创建初始策略文件：

```bash
cargo run -- init
```

创建初始文档树：

```bash
cargo run -- scaffold
```

管理语言策略：

```bash
cargo run -- lang list
cargo run -- lang add ja --min-cjk-ratio 0.10
cargo run -- lang set-threshold ja --max-cjk-ratio 0.90
cargo run -- lang remove ja
```

## 策略

本仓库使用 `docs-hygiene.yml` dogfood Docs Hygiene。

## Adapter

Docs Hygiene 可以调用外部工具，而不是重写它们的规则。第一版 adapter 是
markdownlint：

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
