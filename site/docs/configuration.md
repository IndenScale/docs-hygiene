---
title: 项目配置
description: docs-hygiene.yml 的配置入口、文档范围、语言表示与规则执行策略。
---

# 项目配置

DH 默认从被检查项目根目录读取 `docs-hygiene.yml`。也可以用 `--config` 指向其他策略文件。

## 最小配置

```yaml
entryDocs:
  required:
    - README.md
    - CHANGELOG.md

docs:
  bases:
    - id: guides
      root: docs
      requireContinuousNumbering: false
      maxLines: 300
      patterns:
        - id: guide-page
          regex: "^[a-z][a-z0-9-]*\\.md$"
          documentKind: guide
          numbered: false
```

这个配置声明公共入口文件和一个受管文档区域。DH 只会对显式进入治理范围的内容应用对应契约。

## 文档区域

一个 `docs.bases` 项通常包含：

- `id`：稳定的配置标识；
- `root`：相对项目根目录的文档目录；
- `localizedRoots`：同一内容的本地化表示；
- `patterns`：允许的文件名和 Document Kind；
- `maxLines`：文件体量预算；
- `ignore`：不进入当前契约的子树。

## 语言表示

```yaml
languageRepresentations:
  canonical: en
  localized: [zh]
  requireDocumentParity: true
  requireNumberParity: true
```

`canonical` 和 `localized` 表示权威关系，不是语言代码的别名。同一个语义资产拥有一个 canonical 表示和零到多个 localized 表示，它们共享稳定身份和治理关系。

## 显式控制规则

项目可以对稳定规则族使用三种模式：

- `auto`：从项目事实推导；
- `required`：项目明确要求执行；
- `disabled`：项目明确关闭。

规则的实际执行状态仍然是 `inactive`、`advisory`、`warning` 或 `error`。使用 `explain-rules` 查看事实、自动判断和显式覆盖后的最终结果。

## 多维画像

```yaml
hygieneProfile:
  dimensions:
    structure:
      target: governed
      required: true
    identity:
      target: controlled
      required: true
    dependency:
      target: controlled
      required: true
    topology:
      applicability: notApplicable
      rationale: 当前项目不治理语义依赖图。
```

每个能力维度独立声明目标。一个维度也可以明确为不适用，但必须留下理由。

## 完整参考

当前所有字段和兼容性要求以仓库中的 [配置指南](https://github.com/IndenScale/docs-hygiene/blob/main/docs/zh/guide/configuration.md) 为准。
