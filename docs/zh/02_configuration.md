# 配置

Docs Hygiene 默认从被检查项目根目录读取 `docs-hygiene.yml`，也可以用 `--config` 指定其他配置文件。破坏性字段改名见[术语迁移](08_terminology_migration.md)。

## 必需文件

`entryDocs` 声明项目根目录入口文档。项目根目录采用默认允许策略，因为项目级 AI 工具可能引入 `AGENTS.md`、`CLAUDE.md` 或 `GEMINI.md` 等文件。

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
          documentKind: numbered
          numbered: true
        - id: index
          regex: "^INDEX\\.md$"
          documentKind: index
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
          documentKind: freeform
          numbered: false
```

## 语言表示

`languageRepresentations` 定义 canonical 语言和本地化语言表示。

```yaml
languageRepresentations:
  canonical: en
  localized: [zh]
  requireDocumentParity: true
  requireNumberParity: true
```

在这个布局下，`docs/01_overview.md` 应对应 `docs/zh/01_overview.md`。

默认情况下，以 `docs` 为根的 base 会把 `docs/zh` 识别为 `zh` 子树。当语义目录与
语言目录是两个正交维度时，使用 `localizedRoots` 显式配对：

```yaml
docs:
  bases:
    - id: intent
      root: docs/intent
      localizedRoots:
        zh: docs/zh/intent
      patterns:
        - id: numbered
          regex: "^\\d{2}_[a-z0-9_-]+\\.md$"
          numbered: true
```

本地化根目录使用与 canonical 语言目录相同的文件名规则、编号、行数预算和文档契约。

## 语言表示 CRUD

语言策略可以通过 CLI 命令编辑，不必手动修改 YAML。

```bash
docs-hygiene lang list
docs-hygiene lang add ja --min-cjk-ratio 0.10
docs-hygiene lang add en --canonical --max-cjk-ratio 0.05
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

## 文档契约

`documentContracts.profiles` 根据路径和文件名推导文档类型，第一个匹配的 profile 生效。必要章节可以声明多语言标题别名，其他章节始终开放。完整决策和模型见[文档契约](06_document_contracts.md)。

`documentContracts.maturity.declared` 仍是已配置文档 Profile 的严重程度下限。项目规模
建议只产生信息；通用规则适用性由渐进式激活独立推导。

## 规则激活

`rules` 独立于文档契约成熟度控制稳定规则族。默认 `auto` 模式从集中式项目事实推导
适用性；`required` 强制 error 状态，`disabled` 强制 inactive 状态。

```yaml
rules:
  governance.traceability:
    mode: auto
  localization.parity:
    mode: required
  adapters.external:
    mode: disabled
```

显式模式覆盖启发式。纯规模信号最高只产生 advisory 信息；结构信号和显式功能策略
可以产生 warning 或 error。稳定 ID、证据和 checker 行为见
[渐进式规则激活](10_progressive_rule_activation.md)。

## 治理关系图

`governance.manifests` 启用基于 ID 的资产解析、强制语义 Wiki Link、可选内容哈希锚、垂直派生和递归 Package
成员校验。UL 与 Glossary 是 Library Tree；PRD 与 Spec 是目录 Body Package。
Manifest Schema、关系规则和完整性策略见[治理关系图](07_governance_graph.md)。

## 禁止 ASCII 字符画

`docs.forbidAsciiArt` 默认关闭。开启后，文档中的 ASCII 流程图、框图等字符画会产生 `DH_ASCII_001`。普通正文和 `text`、`ascii`、`diagram` fenced block 会检查；`python`、`bash`、`yaml` 等代码示例不检查。

```yaml
docs:
  forbidAsciiArt: true
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

`ignore.paths` 接受相对项目根目录的 glob 模式。
Docs Hygiene 的内置 policy engine 只检查每个 docs base 根目录下的 Markdown 文件；其他扩展名会被忽略。生成目录、归档、fixtures，或任何不应纳入当前文档契约的子树，都可以通过 `ignore.paths` 排除。

`docs.bases[].ignore` 只对单个 docs base 生效。父级文档目录里包含 ADR、用户故事等独立检查子树时，优先用它排除这些子树。

```yaml
ignore:
  paths:
    - target/**
    - docs/generated/**
```
