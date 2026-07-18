# ADR-0001：开放工程资产模型

状态：adopted

## 背景

Docs Hygiene 当前把一种软件文档 Profile 固化为 Intent、Definition、Implementation 三层
本体，同时把 Library 固化为 UL → Glossary → SDK 投影，并要求相邻的 `formalizes`、
`realizes`、`projects` 关系。实践中，Glossary 重复 UL 权威，Spec 保存的是变化范围内的
工程契约，Implementation Manifest 则枚举容易随重构变化的仓库路径。SDK、代码、配置、
测试和其他实现证据没有可通用的固定目录形态。

仓库已经用 Feature Issue 保存能力边界、验收条件、依赖、交付状态和证据。由 Issue
承载变化范围内的工程契约，比永久增加一个 Definition 精化层更合适。

## 决策

Docs Hygiene 不再规定 Definition 或 Implementation 精化层、Glossary 投影、SDK 投影及
固定实现目录。内建开放工程 Profile 包含四种角色：

- UL 是稳定语义权威，可保存精确定义、别名、边界和规范性主张；
- PRD 是稳定产品意图权威，保存问题、结果、需求、边界、非目标和生命周期；
- Issue 是一次变化的交付权威，保存范围、设计约束、决策、依赖、原子验收、状态和证据；
- Artifact 是位置无关的证据目标，例如代码、测试、配置、Schema、命令、Commit、URL
  或 SDK 表面。

内建 Profile 可以为 UL 和 PRD 约定受治理目录。Issue 存储和 Artifact 位置由 Adapter
决定，绝不从一个强制目录推导。本地 Markdown Issue 归档只是一种 Adapter，不是本体。

## 关系

内建 Profile 保留语义引用、Pin、生命周期迁移、责任、影响和拓扑；删除
`refinementLevel`、`formalizes`、`realizes`、`projects` 和完整相邻层派生。交付采用
位置无关关系：

```text
PRD   --references--> UL
Issue --addresses---> PRD 或 PRD 成员
Issue --references--> UL
Issue --dependsOn----> Issue
Issue --evidencedBy--> Artifact
```

Issue 与 Artifact 关系可以来自仓库文件或外部 Adapter。Artifact 路径不因此成为语义权威。

## 权威与保留

关闭 Issue 不能隐藏当前产品行为。用户可观察行为进入 Guide、Governance、Capability 和
Migration 文档；稳定含义进入 UL；稳定产品意图进入 PRD；可执行事实仍以 Schema、测试、
配置和代码为准。

已交付 Issue 必须保留稳定身份、冻结或可审计的验收、可解析证据和持久历史。可变的外部
Issue 系统必须提供 Pin、导出或可移植快照，才能证明交付。

## 迁移

现有 Definition 资产按主张迁移，不能整批直接删除：

- Glossary 含义和规范性主张合并到 UL；
- 稳定产品结果和边界合并到 PRD；
- 变化范围内的算法、决策、验收和交付证据进入交付该变化的 Issue；
- 用户可观察契约保留在公开产品文档；
- 可执行不变量保留在 Schema 和测试中。

三个 Implementation Manifest 和 SDK Manifest 在有效证据已进入 Issue 或已被测试证明后
删除。旧 Definition/Implementation 配置与边字段应被明确拒绝，不能静默成为无操作配置。

## 后果

核心模型更小，也能用于非软件项目，不再强迫瀑布式目录。PRD 保持稳定，不吸收实现细节；
Issue 成为受治理交付契约。需要 Spec、Glossary、SDK 或 Implementation Manifest 的项目
仍可定义自有 Document Kind 与 Adapter，但这些结构不再获得内建精化语义。

这是破坏性的产品模型变更。治理图 Schema、诊断、激活证据、示例、dogfood 策略、测试和
Position 文档必须一起迁移。只有被删除 Definition 资产中不再独占任何有效当前主张，迁移
才算完成。
