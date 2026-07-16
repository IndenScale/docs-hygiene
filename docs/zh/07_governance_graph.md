# 治理关系图

Docs Hygiene 从显式配置的资产 Manifest 和 Body 内容中的语义 Wiki Link 构建确定性关系图。它校验关系结构
和可达性，不从自然语言正文推断业务含义。

## 资产来源

`governance.manifests` 列出 YAML 文件，或带 YAML frontmatter 的 Markdown 文件。
本地化文档是同一资产的表示，不应作为独立声明重复列入。

```yaml
governance:
  manifests:
    - docs/intent/ul/manifest.yml
    - docs/intent/prd/prd-001/manifest.yml
    - docs/definition/glossary/manifest.yml
    - docs/definition/spec/spec-001/manifest.yml
    - sdk-manifest.yml
    - implementation-manifest.yml
  requireCompleteVerticalDerivation: true
```

## 资产契约

每项声明提供稳定 `id`、`refinementLevel`、`referenceRelation` 和生命周期 `status`。
精化层级可取 `intent`、`definition` 和 `implementation`；
引用关系可取 `body` 和 `library`。语言表示由 canonical 根目录或配置的
`localizedRoots` 推导，因此本地化文件不重复声明资产。
文档级 `version` 字段会被拒绝。历史状态由 Git 记录；需要审议的依赖可改用内容哈希锚。
Manifest 级 `references` 和叶子级 `source` 元数据同样会被拒绝；语义依赖以 Wiki Link 写入受治理内容。

```yaml
id: SPEC-001
refinementLevel: definition
referenceRelation: body
status: baselined
formalizes: PRD-001
```

垂直关系接受单个稳定目标 ID 或目标列表。横向 Library 引用位于 Body 内容中，
不再集中维护在 Manifest 元数据中。

## 递归 Package Tree

目录型 Body 与 Library 必须声明非空 `members` 列表。成员是一个直属 Markdown
叶子或子领域。每个子领域具有自己的 `manifest.yml`，声明 `kind: domain`，并递归
枚举直属子项，树深度不受限制。绝对路径、路径穿越、深层直接声明、Manifest 缺失、
身份重复和未登记子项都是错误。

Library 叶子是稳定术语；Body 叶子是原子角色、故事、需求、约束或验证陈述。每个
叶子的 YAML frontmatter 包含 `id` 和生命周期 `status`。本地化树
保持每个 canonical 路径与身份。Library 失败产生 `DH_LIBRARY_001`，Body Package
失败产生 `DH_BODY_001`。

Implementation Body 可以按 `code`、`configuration` 等类型分组声明项目根目录相对文件成员。
一旦声明，这些路径必须安全、唯一并指向已有文件。

UL 与 Glossary 是递归 Library Tree，不是单体术语表；PRD 与 Spec 是目录 Body
Package，不是单体文档。

Domain 是 Library 内部的稳定语义边界。Sub Domain 是嵌套在另一个 Domain 下的同一种
受治理节点，不形成新的精化层级或引用关系。Domain 边界保持为显式的人类决策，不从
自然语言正文推断分类。

每个 canonical Library Domain 具有可配置的直属成员 fan-out 预算。Markdown 叶子与
子 Domain 各计为一个直属成员，子 Domain 的后代不计入父级。默认从 15 开始 warning、
从 50 开始 error。本地化树保持 canonical 拓扑，但不重复产生 fan-out 诊断。

## 水平引用

每个 Body 内容至少包含一个指向同一精化层级 Library 身份的语义 Wiki Link：

- Intent Body 指向 UL；
- Definition Body 指向 Glossary；
- Implementation Body 指向 SDK。

Definition 或 Implementation Library 内容也至少包含一个指向相邻上游 Library 身份的
Wiki Link。这些内容引用取代叶子级 `source` frontmatter，Package 级 `projects` 边仍保持显式。

`[[DH-LIBRARY]]` 按语义 ID 解析；`[[DH-LIBRARY|Library]]` 添加显示文本；
`[[DH-LIBRARY@sha256:<hash>|Library]]` 还会锚定 canonical 目标字节。目标缺失、精化方向
无效或哈希过期都是错误。canonical 与 localized Package 必须保持相同的 Wiki Link 目标和哈希锚。

## 垂直派生

垂直边从下游资产指向相邻上游权威：

- Definition Body 通过 `formalizes` 指向 Intent Body；
- Implementation Body 通过 `realizes` 指向 Definition Body；
- Definition Library 通过 `projects` 指向 Intent Library；
- Implementation Library 通过 `projects` 指向 Definition Library。

跳过中间精化层级、反转边方向、使用错误引用关系或无法解析 ID 都是错误。
水平边不能满足缺失的垂直边。

## 反向完整性

启用 `requireCompleteVerticalDerivation` 后，每个状态为 `baselined` 或 `current` 的
非 Implementation 资产都必须具有相邻下游派生。这会发现没有 Spec 的 PRD、没有实现的
Spec、没有 Glossary 的 UL 和没有 SDK 的 Glossary。`draft`、`review`、`proposed`
及其他未基线资产可以暂无下游派生。

## 边界

当前关系图校验资产级身份、精化层级、引用关系、边类型、可达性、语义 Wiki Link、
可选内容哈希锚、递归 Package 成员和语言表示身份一致性。历史文档状态以 Git 历史为权威。
它尚不校验条目级需求覆盖、术语级投影完整性、符号级语义映射或自然语言矛盾。
