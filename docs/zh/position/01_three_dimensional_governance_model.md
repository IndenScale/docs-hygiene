# 三维软件文档 Profile

状态：adopted

范围：内建软件文档 Profile

本 Profile 派生自[认知资产治理模型](02_cognitive_asset_governance_model.md)。它是一类受治理
表面的坐标系，不再作为顶层产品本体。Library 提供 Semantic Authority，类型化边实现
Change Propagation，项目策略控制何处需要 Human Judgment。

## 立场主张

在 AI Coding 时代，项目文档是项目意图与决策的 SSOT。代码与配置是实现事实，
但只有受治理的文档能够稳定说明为什么做、为谁做、什么算正确，以及实现依据什么
决策。Docs Hygiene 的职责不是生成这些决策，而是把决策之间必须持续成立的关系变成
可验证的不变式。

项目是治理对象，目录是运行边界，Git 仓库只是物理载体。一个治理范围可以覆盖整个
仓库，也可以对应 monorepo 中的一个项目目录。当前 CLI 每次从显式指定的项目根目录
加载策略和解析资产，不自动发现或编排其他项目。

在本 Profile 中，Docs Hygiene 使用三个相互独立的维度定位每项受治理的项目资产：

1. 精化层级守护从意图到实现的逐层精化；
2. 引用关系守护项目主张与共享定义之间的语义依赖；
3. 语言表示守护工作语言与分发语言对同一资产的表示。

三个维度确定资产表示的坐标，类型化治理边则维护这些坐标之间的引用和派生关系。
这些不变式使偏离尽早暴露，并减少反复确认术语、文档身份、权威表示和实现依据的
理解代价。

## 精化层级

精化是歧义和实现自由度逐层减少的过程，并不意味着 Intent 本身低质或不完整。

| 精化层级 | Body | Library | 核心问题 |
| --- | --- | --- | --- |
| Intent | PRD | UL | 为什么做、为谁做、期望什么结果？ |
| Definition | Spec 与测试定义 | Glossary | 怎样才精确地算正确？ |
| Implementation | 代码与配置 | SDK | 哪一种可执行形式落实定义？ |

Body 派生链是 `PRD → Spec/Test → Code/Configuration`；Library 投影链是 `UL → Glossary → SDK`。每向下一级，歧义减少、约束增加、剩余决策空间收窄。

## 引用关系

Body 表达当前项目的具体主张；Library 维护可复用的术语、类型或规则，为多个 Body 提供共享含义。Body 内容中的语义 Wiki Link 建立 `references` 边，指向同一精化层级 Library 身份：

- `PRD → UL`；
- `Spec/Test → Glossary`；
- `Code/Configuration → SDK`。

`Reference` 专门表示引用边或引用动作；受治理的共享资产统一称为 Library。

## 语言表示

`en`、`zh`、`ja` 等语言代码是语言表示维度的取值；`canonical` 与 `localized` 是表示的权威属性，不是语言值。

一项语义资产拥有一个 canonical 表示和零到多个 localized 表示。本地化表示保持 canonical 路径、身份、生命周期、Package 结构和治理边，不形成独立图节点或竞争性的含义来源。

```text
PRD-001
├── en  canonical
└── zh  localized
```

## 治理关系图

资产坐标与图关系是两个不同概念：

```text
coordinate = (refinementLevel, referenceRelation, languageRepresentation)
edges      = references | formalizes | realizes | projects
```

- `references` 由语义 Wiki Link 派生，连接同一精化层级的 Body 与 Library 身份；
- `formalizes` 连接 Definition Body 与 Intent Body；
- `realizes` 连接 Implementation Body 与 Definition Body；
- `projects` 连接下游 Library 与相邻上游 Library。

治理依据是语义权威，而不是文件扩展名。YAML 可以表达意图策略、定义 Schema 或运行配置；它的精化层级取决于它承载的主张。

## 边界

当前检查校验项目根目录内 Markdown 目标、受治理 frontmatter、资产身份、生命周期、三维分类、特定类型 Package 结构、语言同位、Wiki Link 引用、可选内容哈希锚定和治理图可达性。外部 URL 可达性和通用文风质量仍是集成工具的职责；检查器尚不推断自然语言等价性、译文新鲜度、条目级覆盖或语义矛盾。
