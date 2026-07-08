# 配置

Docs Hygiene 默认从被检查仓库根目录读取 `docs-hygiene.yml`。也可以用 `--config` 指定其他配置文件。

## 必需文件

`entryDocs` 声明仓库根目录入口文档。仓库根目录采用默认允许策略，因为项目级 AI 工具可能引入 `AGENTS.md`、`CLAUDE.md` 或 `GEMINI.md` 等文件。

```yaml
entryDocs:
  required:
    - README.md
    - README_ZH.md
    - CHANGELOG.md
    - LICENSE
  optional:
    - AGENTS.md
    - CLAUDE.md
    - GEMINI.md
```

## 文档结构

`docs.bases` 控制文档契约区域。docs base 对 Markdown 文件采用默认拒绝策略：每个被检查的 `.md` 文件都必须匹配某个配置的 pattern，或被全局 `ignore.paths`、当前 base 自己的 `ignore` 排除。

```yaml
docs:
  bases:
    - id: main
      root: docs
      requireContinuousNumbering: true
      maxLines: 500
      ignore:
        - docs/adr/**
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          role: numbered
          numbered: true
        - id: index
          regex: "^INDEX\\.md$"
          role: index
          numbered: false
```

在这个配置中，`INDEX.md` 是合法文件，但不参与连续编号。

旧的 `docs.root` 和 `docs.filenamePattern` 仍然可作为单 base 简写。

## 多 Base

不同文档区域可以使用不同命名规则。当父级 docs base 包含子级 docs base 时，可以用父级 base 的 `ignore` 排除子树，避免同一批文件被检查两次。

```yaml
docs:
  bases:
    - id: guide
      root: docs
      requireContinuousNumbering: true
      ignore:
        - docs/adr/**
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          numbered: true
    - id: adr
      root: docs/adr
      patterns:
        - id: adr
          regex: "^ADR-\\d{4}_[a-z0-9_-]+\\.md$"
          role: freeform
          numbered: false
```

## I18n

`i18n` 定义根语言和本地化文档目录。

```yaml
i18n:
  rootLang: en
  languages: [zh]
  requireDocsParity: true
  requireNumberParity: true
```

在这个布局下，`docs/01_overview.md` 应对应 `docs/zh/01_overview.md`。

## 语言 CRUD

语言策略可以通过 CLI 命令编辑，不必手动修改 YAML。

```bash
docs-hygiene lang list
docs-hygiene lang add ja --min-cjk-ratio 0.10
docs-hygiene lang add en --root --max-cjk-ratio 0.05
docs-hygiene lang set-threshold ja --max-cjk-ratio 0.90
docs-hygiene lang remove ja
```

当策略文件不是 `docs-hygiene.yml` 时，每个命令都可以使用 `--config`。

## 语言阈值

`language` 定义轻量 CJK 比例阈值。代码块会被忽略。

```yaml
language:
  en:
    maxCjkRatio: 0.05
  zh:
    minCjkRatio: 0.15
```

## 豁免

`suppressions` 可以按路径关闭指定诊断。它适合 fixtures、翻译示例、生成文档，或包含混合语言的测试用例。

```yaml
suppressions:
  - code: DH_LANG_002
    paths:
      - docs/fixtures/**
    reason: Fixtures intentionally contain Chinese examples in every locale.
```

只有在路径范围很窄、且该路径下所有 Docs Hygiene 诊断都预期会产生噪声时，才使用 `code: "*"`。

## 忽略路径

`ignore.paths` 接受相对仓库根目录的 glob 模式。
Docs Hygiene 的内置 policy engine 只检查每个 docs base 根目录下的 Markdown 文件；其他扩展名会被忽略。生成目录、归档、fixtures，或任何不应纳入当前文档契约的子树，都可以通过 `ignore.paths` 排除。

`docs.bases[].ignore` 只对单个 docs base 生效。父级文档目录里包含 ADR、用户故事等独立检查子树时，优先用它排除这些子树。

```yaml
ignore:
  paths:
    - target/**
    - docs/generated/**
```
