# 概览

Docs Hygiene 是面向项目文档中认知资产的 **Policy Engine**。它把显式身份、依赖、
生命周期和项目策略编译为确定性 finding，不从自然语言推断业务真相。

## 治理责任

| 治理责任 | 权威或机制 | 核心问题 |
| --- | --- | --- |
| Semantic Authority / 语义权威 | Library | 它是什么意思，哪一个定义具有权威？ |
| Change Propagation / 变化传播 | Dependency | 上游变化后，谁可能受到影响？ |
| Human Judgment / 人类判断 | Decision | 人们选择了什么、为什么，以及何时需要重新审议？ |

Library Entry 与 Decision Record 是受治理资产；Reference 是一种 Dependency。完整定位见
[认知资产治理模型](position/02_cognitive_asset_governance_model.md)。

当前 CLI 已交付稳定 Library 身份、语义与锁定 Dependency、scoped 新鲜度证据、生命周期、
结构影响、拓扑、责任复核和审计更新。一等 Decision 资产、通用 Agent Attestation 与
Issue Review 是产品方向，不属于当前行为。

## Review 与自动化边界

Dependency 分析区分三个概念：

```text
potentialImpact  = 图结构上可达的消费方
invalidated      = 所锁定上游状态发生变化的已 Pin 消费方
reviewSet        = 按策略必须处理的失效消费方
```

当前 `transitiveImpact` 是结构性的 `potentialImpact`；关键 Pin 与 scoped anchor 提供
确定性失效证据。未 Pin 的边在上游内容变化后仍只产生建议性影响；项目只为需要精确状态
复核的依赖配置 Pin，即可减少虚警。外部 AI 可以分析 Info 级 finding，但 Docs Hygiene
不配置无人值守 Agent，也不把概率判断隐藏在确定性检查中。

架构仍由人负责。DH 可以暴露路径、社区、集中度和可能的隔离边界，但不替团队决定引入
中间契约或改写依赖图。

## 软件文档 Profile

内建 Profile 按精化层级、引用关系和语言表示定位资产，守护逐层精化、Body → Library
语义和 canonical/localized 同位。这些坐标把顶层治理责任实例化到软件文档，不是顶层
产品本体。详见[三维软件文档 Profile](position/01_three_dimensional_governance_model.md)。

## 产品边界

Docs Hygiene 负责需要项目上下文的规则：

- 公开入口文件完整性和 docs base 默认拒绝；
- 编号结构、索引、文档契约和受治理 frontmatter；
- 稳定身份、Library 权威、语义引用和生命周期；
- canonical/localized 表示同位；
- 类型化 Dependency 边、Pin、scoped anchor、影响和拓扑；
- 可解释渐进激活、责任复核和审计更新；
- 外部 Adapter 编排。

它不负责通用 Markdown 格式、外部 URL 爬取、拼写或文风；这些能力继续交给
markdownlint、lychee、Vale、cspell 或 slop-lint。它不判断叙事是否正确，不调度 Agent，
也不替团队作架构或产品 Decision。

已交付的[文档卫生治理模型](11_hygiene_governance_model.md)在这一边界内分离能力维度、
成熟度和执行状态。
