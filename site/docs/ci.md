---
title: 接入 CI
description: 在持续集成中运行 Docs Hygiene，并为其他工具输出稳定 JSON。
---

# 接入 CI

DH 适合同时运行在开发者本地和 CI。两处使用相同的项目根目录和策略文件，避免门禁只在远端才可解释。

## 基本门禁

```bash
docs-hygiene check . --fail-on-warning
```

如果项目处在渐进采用阶段，可以先不使用 `--fail-on-warning`，只让 error 阻断提交。

## GitHub Actions

```yaml
name: Docs Hygiene

on:
  pull_request:
  push:

jobs:
  docs-hygiene:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Build Docs Hygiene
        run: cargo build --release --locked
      - name: Check project documentation
        run: ./target/release/docs-hygiene check . --fail-on-warning
```

在实际项目中，建议固定 DH 的来源版本，避免工具升级和项目文档变化同时进入一次门禁。

## 机器可读输出

```bash
docs-hygiene check . --format json > dh-report.json
docs-hygiene explain-rules . --format json > dh-rules.json
docs-hygiene profile . --format json > dh-profile.json
```

JSON 输出适合归档、可视化或交给其他自动化消费。外部 AI 可以分析 Info 级 finding，但不应把概率判断伪装成 DH 的确定性结论。

## 与其他文档工具协作

DH 不替代已有工具：

| 工具类型 | 负责内容 |
| --- | --- |
| markdownlint | Markdown 语法与格式 |
| lychee | 外部 URL 可达性 |
| Vale、cspell | 文风与拼写 |
| Docs Hygiene | 需要项目上下文的身份、依赖与追溯关系 |

可以通过 Adapter 让 DH 在同一次检查中调用外部工具，但各工具仍然维护自己的规则系统。
