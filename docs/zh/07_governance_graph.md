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

## 水平引用

每个 Body 内容至少包含一个指向同一精化层级 Library 身份的语义 Wiki Link：

- Intent Body 指向 UL；
- Definition Body 指向 Glossary；
- Implementation Body 指向 SDK。

Definition 或 Implementation Library 内容也至少包含一个指向相邻上游 Library 身份的
Wiki Link。这些内容引用取代叶子级 `source` frontmatter，Package 级 `projects` 边仍保持显式。

`[[DH-LIBRARY]]` 按语义 ID 解析；`[[DH-LIBRARY|Library]]` 添加显示文本；
`[[DH-LIBRARY#library]]` 选择 canonical 的 `# Library` 标题；
`[[DH-LIBRARY#library@sha256:<hash>|Library]]` 还会锚定完整 canonical 目标字节。目标或
selector 缺失、精化方向无效或哈希过期都是错误。canonical 与 localized Package 必须
保持相同的 Wiki Link 目标、selector 和 hash 锚。

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

## 规范化边记录

[PRD-004](intent/prd/prd-004/index.md)现已把语义引用、锁定引用、派生和投影规范化为统一
的有序依赖边记录。每条记录包含来源和目标身份、关系类型、来源位置、可选 selector 与
内容锚，以及生命周期出处。画像报告有序节点与边，并给出基础解析、关系类型和孤立节点
指标；现有解析、过期检测和反向完整性检查消费这些记录。标准 Markdown Link 仍保持
导航语义，没有显式语义关系时绝不会进入语义图。
对于非 Markdown 实现成员，只有独立注释行中的语义 Wiki Link 会被识别；字符串字面量
和 fixture 不会成为边。

声明语法通过带版本的 `docs-hygiene.reference-occurrence.v1` IR 与边契约解耦。
Wiki Link、Markdown Link 和 frontmatter collector 都产出这一结构；显式
`(syntax, context)` 策略先将其判定为语义依赖、仅导航或身份声明，再由语法无关
规范化器处理。Markdown 默认不进入语义图因此成为可检查策略，同时公开边 JSON 保持
不变。详见
[SPEC-003 C-012](definition/spec/spec-003/constraints/reference-occurrence-ir.md)。

## 标题 Selector

selector 使用小写 ASCII 标题 slug，例如
`[[DH-HYGIENE-PROFILE#documentation-hygiene-profile]]`。ATX 标题中的连续标点、格式字符
和空白会折叠为一个连字符。治理边保留 selector；canonical 目标不存在对应标题时，
`DH_SELECTOR_001` 定位到来源行。selector 不会把可选 SHA-256 锚从全文件范围改为块级
范围。详见 [SPEC-003 C-011](definition/spec/spec-003/constraints/selector-resolution.md)。

## 多粒度与多锚

Markdown frontmatter 可以声明 `anchors` 序列。每项指定一个受管目标和显式的 `file`、
`block` 或 `commit` scope。file 对完整目标做 hash；block 要求标题 slug `locator`，只对
该 ATX 标题段做 hash。多个项目形成独立排序的锁定边，并在各自声明行报告失败。

commit scope 使用 `algorithm: git` 和完整 commit OID，默认关闭；只有配置
`governance.contentAnchors.verifyGitCommits: true` 后，检查器才比较当前目标与该 commit
中同路径 blob。既有内联 selector 加 SHA-256 的链接继续锁定整文件。详见
[SPEC-003 C-013](definition/spec/spec-003/constraints/scoped-content-anchors.md)。

外发或跨仓库证据可在 SHA-256 file/block anchor 上携带类型化 `snapshot` provenance。
已登记的版本化 manifest 把稳定 repository 身份和 commit OID 绑定到本地 payload，显式
导入后常规检查保持离线。详见[可移植 Commit 快照](17_portable_snapshots.md)和
[SPEC-003 C-017](definition/spec/spec-003/constraints/portable-commit-snapshot.md)。

## 传递影响

关系图对每条已解析语义边取反，并以 `transitiveImpact` 报告：对于每个发生变化的权威
身份，列出排序后的全部直接和间接依赖方。并行边会去重，循环会终止且不包含变化身份
自身，未解析端点不传播。这是确定性分析，不是预算或通知策略。详见
[SPEC-003 C-014](definition/spec/spec-003/constraints/transitive-impact.md)。

## 生命周期与权威迁移

资产、Package 领域和叶子可以保留旧稳定身份，以 `status: superseded` 标记，并通过
`supersededBy` 声明已建立的替代权威。后继必须保持精化层级与引用关系，并处于
`baselined` 或 `current`；其他状态不得声明后继。superseded、archived 和 abandoned
身份不能继续作为边目标；诊断定位陈旧消费方并提示已声明替代。关系图通过
`authorityMigrations` 有序公开这些声明。详见
[SPEC-003 C-015](definition/spec/spec-003/constraints/identity-lifecycle.md)。

## 拓扑策略

关系图以不同的受管邻居身份计算 Fan-In 和 Fan-Out，因此重复链接或并行边类型不会放大
度数。有向循环组采用确定性的强连通分量；自环是只有一个节点的循环组。

拓扑执行需要显式启用：

```yaml
governance:
  topology:
    maxFanIn: 8
    maxFanOut: 12
    forbidCycles: true
```

配置限制后会激活独立的 `governance.topology` 规则族。仅存在关系图或仓库规模增长不会
激活阻断性拓扑策略。`DH_TOPOLOGY_001` 报告 Fan 阈值违规，`DH_TOPOLOGY_002` 报告被
禁止的循环组；精确节点、限定方向且可审计的例外遵循
[SPEC-003 C-018](definition/spec/spec-003/constraints/supernode-exceptions.md)。
