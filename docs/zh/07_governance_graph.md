# 治理关系图

Docs Hygiene 从显式配置的版本化资产 Manifest 构建确定性关系图。它校验关系结构
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

每项声明提供稳定 `id`、语义 `version`、`layer`、`role` 和生命周期 `status`。
可用层次为 `intent`、`definition` 和 `implementation`；可用角色为 `body` 和
`library`。

```yaml
id: SPEC-001
version: 1.0.0
layer: definition
role: body
status: baselined
references: { id: GLOSSARY-001, version: 1.0.0 }
formalizes: { id: PRD-001, version: 1.1.0 }
```

每个关系字段既可接受单个目标映射，也可接受目标列表。目标始终包含精确身份
和语义版本。

## 递归 Package Tree

目录型 Body 与 Library 必须声明非空 `members` 列表。成员是一个直属 Markdown
叶子或子领域。每个子领域具有自己的 `manifest.yml`，声明 `kind: domain`，并递归
枚举直属子项，树深度不受限制。绝对路径、路径穿越、深层直接声明、Manifest 缺失、
身份重复和未登记子项都是错误。

Library 叶子是稳定术语；Body 叶子是原子角色、故事、需求、约束或验证陈述。每个
叶子的 YAML frontmatter 包含 `id`、语义 `version` 和生命周期 `status`。本地化树
保持每个 canonical 路径与身份。Library 失败产生 `DH_LIBRARY_001`，Body Package
失败产生 `DH_BODY_001`。

UL 与 Glossary 是递归 Library Tree，不是单体术语表；PRD 与 Spec 是目录 Body
Package，不是单体文档。Body 使用普通 Markdown Link 引用具体术语，治理边固定
Package 版本。

## 水平引用

每个 Body 至少声明一条指向同层 Library 的 `references` 边：

- Intent Body 指向 UL；
- Definition Body 指向 Glossary；
- Implementation Body 指向 SDK。

目标缺失、目标为 Body 或目标位于其他层次都是错误。Library 不能使用 `references`
代替 `projects`。

## 垂直派生

垂直边从下游资产指向相邻上游权威：

- Definition Body 通过 `formalizes` 指向 Intent Body；
- Implementation Body 通过 `realizes` 指向 Definition Body；
- Definition Library 通过 `projects` 指向 Intent Library；
- Implementation Library 通过 `projects` 指向 Definition Library。

跳过中间层、反转边方向、使用错误角色或无法解析 `id@version` 都是错误。
水平边不能满足缺失的垂直边。

## 反向完整性

启用 `requireCompleteVerticalDerivation` 后，每个状态为 `baselined` 或 `current` 的
非 Implementation 资产都必须具有相邻下游派生。这会发现没有 Spec 的 PRD、没有实现的
Spec、没有 Glossary 的 UL 和没有 SDK 的 Glossary。`draft`、`review`、`proposed`
及其他未基线资产可以暂无下游派生。

## 边界

当前关系图校验资产级身份、版本、角色、层次、边类型、可达性、递归 Package 成员
和本地化身份一致性。
它尚不校验条目级需求覆盖、术语级投影完整性、符号级语义映射、Evidence 或自然
语言矛盾。
