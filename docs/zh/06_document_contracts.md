# 文档契约

Docs Hygiene 根据项目根目录约定推导文档意图，不要求每个 Markdown 文件自行声明类型。一个 profile 同时匹配项目根目录相对路径 glob 与文件名正则；配置顺序决定优先级，第一个匹配项获得该文档的治理权。

## 决策

文档治理遵循四项规则：

1. 用路径与文件名约定识别项目入口文档、目录索引、CHANGELOG、ROADMAP 和 ADR 等文档类型。
2. Profile 只定义必要字段和语义章节，允许作者开放补充其他章节。
3. 项目声明的成熟度决定 Profile 契约缺口是建议还是阻断；项目规模只能建议提高 Profile 成熟度，不能自动改变门禁。
4. 同一个语义章节可以接受多语言标题别名；译文新鲜度与结构同位分别治理。

这项决策保持 GitHub 和包工具对标准入口文档的正常阅读体验，避免仅为声明类型而添加 frontmatter，并允许早期项目渐进采用治理能力。

## 类型推导

每个 profile 包含 `match.paths` 和 `match.filenames`。两者同时配置时必须同时匹配。更具体的 profile 应排列在通用 profile 之前。

```yaml
documentContracts:
  profiles:
    - id: adr
      match:
        paths: ["docs/**/decisions/*.md"]
        filenames: ["^\\d{4}-[a-z0-9-]+\\.md$"]
```

## 开放契约

`requiredSections` 使用稳定语义 ID，并为每个语义提供一个或多个可接受标题。检查器要求这些章节存在，并可选检查顺序；其他章节保持开放。`requiredFields` 使用正则表达式约束可见元数据或无需 frontmatter 的重复约定。

```yaml
requiredSections:
  - id: context
    headings: [Context, 上下文]
  - id: decision
    headings: [Decision, 决策]
requiredFields:
  - id: status
    pattern: "(?m)^状态："
orderedSections: true
```

## 可复用模板注册表

`documentContracts.templates` 把共享契约策略从单个 Profile 中提取出来。Profile 指定一个
模板，并可追加自己的章节、字段或占位符表达式。解析时模板列表成员在 Profile 成员之前；
Profile 标量覆盖模板标量。解析后重复的章节或字段 ID 属于配置错误，而非隐式覆盖。

```yaml
documentContracts:
  templates:
    - id: maintained-open-contract
      revision: 1
      compatibleFrom: 1
      enforceFrom: maintained
      placeholdersAllowedUntil: growing
      orderedSections: true
  profiles:
    - id: project-readme
      template: maintained-open-contract
      templateRevision: 1
      match:
        paths: [README.md, README_ZH.md]
```

画像报告公开模板绑定和不完整覆盖。受控复用要求注册表有效、每个 Profile 都已绑定且每个
模板都被使用。旧内联 Profile 保持兼容，但不能证明复用。治理级模板进一步声明正整数
revision 窗口和精确 Profile pin。使用 `docs-hygiene migrate-templates --check` 检测缺失或
兼容但过期的 pin，使用 `docs-hygiene migrate-templates` 原子推进；不兼容 pin 会阻止全部
写入。详见 [SPEC-003 C-010](definition/spec/spec-003/constraints/template-lifecycle.md)。

## 成熟度与占位符

成熟度依次为 `seed`、`growing`、`maintained` 和 `governed`。Profile 的 `enforceFrom` 指定缺失要求从哪个等级开始成为错误；在此之前只产生 warning。

配置的占位符表达式用于显式暴露尚未完成的章节。占位符在 `placeholdersAllowedUntil` 及以前产生信息提示，超过该成熟度后成为错误。成熟度建议可以使用项目根目录总行数、总字节数和受管文档数量；同一建议中配置的阈值必须全部满足。

```yaml
maturity:
  declared: growing
  recommendations:
    - level: maintained
      minProjectLines: 10000
      minManagedDocuments: 20
```

建议只产生诊断。项目必须显式提高 `declared`，更强的 Profile 门禁才会生效。通用规则族
的适用性由[渐进式规则激活](10_progressive_rule_activation.md)独立推导，因此项目不必
为所有治理规则选择一个全局成熟度。

这四个名称仍是已交付的文档契约兼容模型。[PRD-004](intent/prd/prd-004/index.md)提出在
每个能力维度上应用独立的三层成熟度，但不会追溯性地改变当前配置契约。

## 多语言边界

不同语言的标题可以映射到同一个语义章节 ID，因此结构契约不要求展示标题完全一致。已有语言表示同位检查继续验证对应文档是否存在。译文内容是否过期需要保存源 revision 或内容 hash，首版文档契约暂不引入这类隐式状态。
