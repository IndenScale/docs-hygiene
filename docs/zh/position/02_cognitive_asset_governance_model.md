# 认知资产治理模型

状态：adopted

范围：Docs Hygiene 叙事与产品边界

## 立场主张

Docs Hygiene 对认知资产与叙事资产实施工程化治理。它把文档，以及未来 Review Profile
中的 Issue，视为持续演化的受治理资产；其含义、依赖和人类判断必须始终可检查。

三个治理责任构成顶层产品模型：

| 治理责任 | 权威或机制 | 核心问题 |
| --- | --- | --- |
| Semantic Authority / 语义权威 | Library | 它是什么意思，哪一个定义具有权威？ |
| Change Propagation / 变化传播 | Dependency | 上游变化后，谁可能受到影响？ |
| Human Judgment / 人类判断 | Decision | 人们选择了什么、为什么，以及何时需要重新审议？ |

三者是治理责任，不是三个对称的图实体。`Reference` 是一种 Dependency；Library Entry
和 Decision Record 则是受治理的资产类型。

## 公共底座

形式模型分离节点、边和 Review 证据：

```text
Asset
├── Library Entry
├── Decision Record
├── Document
└── Issue

Dependency
├── Reference
├── Pinned Reference
├── Derivation
└── Lifecycle Relation

Attestation
├── Agent Analysis
└── Human Confirmation
```

稳定身份、生命周期、责任、证据和表示元数据是公共治理属性。当前 CLI 治理项目文档；
Issue Review 与通用 Agent Attestation 是产品方向，不是对已交付行为的主张。

## 语义权威

Library 为可复用概念、实体、指标、规则和设计组件提供 canonical 含义。消费方应引用
权威，而不是静默地重新定义。AI 可以建议现有身份、指出歧义匹配或漂移，并提出新候选；
仅凭相似度不能创建语义权威或阻断 CI。

当前已交付基础包括稳定 Library 身份、语义引用、claim 权威、建议性重复扫描、受控摘录
和显式权威迁移。

## 变化传播

Dependency 表示一个受治理资产消费另一个资产。`Reference` 是语义 Dependency；Pin
进一步锁定已经审议的上游状态。一次变化产生三个不同集合：

```text
potentialImpact  = 图结构上可达的消费方
invalidated      = 所锁定上游状态发生变化的已 Pin 消费方
reviewSet        = 按策略必须处理的失效消费方
```

未 Pin 的边只进入 `potentialImpact`：上游内容变化不会自动使其消费方失效。锁定内容哈希
或等价的已审议状态，才会让 Dependency 进入确定性的新鲜度检查。项目应只为确实需要
复核精确上游状态的依赖配置 Pin，以此降低虚警；AI 还可以把建议性影响分类为 `none`、
`possible`、`material` 或 `unknown`，但关键依赖的接受仍须可归因、可审计。

架构隔离通过真实的中间契约资产表达。若 `A → A1 → B` 取代直接的 `A → B`，B 变化时
首先使 A1 失效；A1 完成适配且没有改变 A 所消费的契约时，传播在 A1 停止。Docs Hygiene
可以分析路径、社区、集中度和可能的边界，但不替架构师决定必须引入 A1。

当前 `transitiveImpact` 是确定性的结构分析，因此最接近 `potentialImpact`。只有 Pin 或
其他显式新鲜度契约，才会把上游漂移变成确定性失效。关键 Pin、scoped anchor、拓扑和
显式例外，为选择性的 reviewSet 提供了已交付基础。

## 人类判断

Decision 是少而精的重大人类选择记录；Policy 是由 Decision 建立、修改或退役的当前规则。
已接受 Decision 应保留历史理由；选择发生变化时创建后继，而不是重写历史。

Decision 治理应覆盖替代方案、权衡、证据、Owner、范围、复核日期，以及 `proposed`、
`accepted`、`superseded`、`retired` 等生命周期。普通实现选择、Pin 刷新和 Issue 状态变化
不会自动升级为 Decision。

当前责任复核、审计记录、文档契约和生命周期迁移是基础；一等 Decision 资产及其 Policy
关系尚未交付。

## 工具与自动化边界

Docs Hygiene 负责确定性的资产发现、依赖图分析、策略评估、诊断和可审计更新协议；它
不负责 Agent 调度、无人值守服务配置或架构设计。

外部 Agent 可以消费带版本的 finding、分析语义影响并返回可归因证据。项目策略决定哪些
Info 可以自动接受、哪些必须由人处理。核心工具必须保留 actor、evidence、policy 和
result，不能把概率判断隐藏在确定性 checker 内部。

## 精化层级

在内建的软件文档 Profile 中，精化层级定位 Intent、Definition 与 Implementation 资产。
这是一个具体 Profile 对公共底座的使用，不是第四项顶层产品责任。

## 引用关系

在该 Profile 中，Body 与 Library 描述资产的角色；`Reference` 则是 Body 消费 Library
权威时形成的 Dependency 边。这个区分使 Library 始终表示权威，而 Reference 明确是一种
特殊依赖。

## 语言表示

canonical 与 localized 是同一语义资产的不同表示，共享身份和治理关系。本地化不会创建
第二个权威，也不会形成独立 Decision。

## 治理关系图

顶层产品关系图将资产、边与 Review 证据分开：

```text
Library Entry ── Reference/Dependency ──▶ Consumer Asset
Decision Record ── establishes/amends ──▶ Policy
Attestation ── evaluates/confirms ──▶ Asset or Dependency state
```

现有三维坐标定义软件文档如何实例化这张图的一部分。详见
[三维软件文档 Profile](01_three_dimensional_governance_model.md)。

## 边界

当前产品已交付 Library 身份、类型化文档依赖、Pin 与 anchor、确定性的结构影响分析、
拓扑分析、责任复核、审计和生命周期控制。一等 Decision 资产、通用 Agent Attestation
与 Issue Review 仍是产品方向。Docs Hygiene 提供依赖图证据，不配置无人值守 Agent，
也不替团队选择架构。
