# 三维治理模型

状态：adopted

范围：Docs Hygiene 产品模型

## 立场主张

Docs Hygiene 使用三个相互独立的维度定位每项受治理资产：

1. 精化层级描述认知从意图走向可执行形式的过程；
2. 引用关系区分表达项目具体主张的 Body 与维护共享含义的 Library；
3. 语言表示标识同一资产使用哪一种自然语言呈现。

三个维度确定资产表示的坐标，类型化治理边则维护这些坐标之间的引用和派生关系。

## 精化层级

精化是歧义和实现自由度逐层减少的过程，并不意味着 Intent 本身低质或不完整。

| 精化层级 | Body | Library | 核心问题 |
| --- | --- | --- | --- |
| Intent | PRD | UL | 为什么做、为谁做、期望什么结果？ |
| Definition | Spec 与测试定义 | Glossary | 怎样才精确地算正确？ |
| Implementation | 代码与配置 | SDK | 哪一种可执行形式落实定义？ |

Body 派生链是 `PRD → Spec/Test → Code/Configuration`；Library 投影链是 `UL → Glossary → SDK`。每向下一级，歧义减少、约束增加、剩余决策空间收窄。

## 引用关系

Body 表达当前项目的具体主张；Library 维护可复用的术语、类型或规则，为多个 Body 提供共享含义。`references` 边从 Body 指向同一精化层级的 Library：

- `PRD → UL`；
- `Spec/Test → Glossary`；
- `Code/Configuration → SDK`。

`Reference` 专门表示引用边或引用动作；受治理的共享资产统一称为 Library。

## 语言表示

`en`、`zh`、`ja` 等语言代码是语言表示维度的取值；`canonical` 与 `localized` 是表示的权威属性，不是语言值。

一项语义资产拥有一个 canonical 表示和零到多个 localized 表示。本地化表示保持 canonical 路径、身份、语义版本、生命周期、Package 结构和治理边，不形成独立图节点或竞争性的含义来源。

```text
PRD-001@1.2.0
├── en  canonical
└── zh  localized
```

## 治理关系图

资产坐标与图关系是两个不同概念：

```text
coordinate = (refinementLevel, referenceRelation, languageRepresentation)
edges      = references | formalizes | realizes | projects
```

- `references` 连接同一精化层级的 Body 与 Library；
- `formalizes` 连接 Definition Body 与 Intent Body；
- `realizes` 连接 Implementation Body 与 Definition Body；
- `projects` 连接下游 Library 与相邻上游 Library。

治理依据是语义权威，而不是文件扩展名。YAML 可以表达意图策略、定义 Schema 或运行配置；它的精化层级取决于它承载的主张。

## 边界

当前检查校验资产身份、版本、生命周期、三维分类、Package 结构、语言同位和治理图可达性；尚不推断自然语言等价性、译文新鲜度、条目级覆盖或语义矛盾。
