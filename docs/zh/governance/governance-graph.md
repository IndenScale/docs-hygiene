# 治理关系图

Docs Hygiene 从配置的 Manifest 与语义 Wiki Link 构建确定性关系图。它验证显式结构，
不从正文或仓库位置推断业务含义。

## 资产来源

```yaml
governance:
  manifests:
    - docs/engineering/ul/manifest.yml
    - docs/engineering/prd/prd-001/manifest.yml
```

本地化文档表示同一资产，不作为独立声明。Issue 与 Artifact 关系通过适配器或 Issue
证据进入。

## 资产契约

每项声明提供稳定 `id`、`referenceRelation` 和生命周期 `status`。UL 术语使用
`library`，PRD Package 使用 `body`。已移除的 `refinementLevel`、`formalizes`、
`realizes` 与 `projects` 字段会被拒绝。

```yaml
id: PRD-001
referenceRelation: body
status: baselined
```

文档级版本、Manifest 级语义引用与叶子级 source 元数据会被拒绝。Git 记录历史，受治理
正文记录语义依赖。

## 递归 Package Tree

UL Library 与 PRD Body 目录声明非空直属 `members`。子领域拥有 `manifest.yml` 并递归
枚举直属成员；Markdown 叶子携带稳定 `id` 与生命周期 `status`。本地化树保持 canonical
路径、身份、成员关系与引用签名。Issue 与 Artifact 不要求镜像 Package Tree。

## 语义引用

受治理 PRD Body 使用 Wiki Link 指向 UL Library 身份。Issue 适配器可以暴露
`addresses`、`dependsOn` 与 `evidencedBy` 关系，而不规定 Issue 或 Artifact 的位置。

- `[[DH-LIBRARY]]` 解析语义身份；
- `[[DH-LIBRARY|Library]]` 添加显示文本；
- `[[DH-LIBRARY#library]]` 选择 canonical 标题；
- `[[DH-LIBRARY#library@sha256:<hash>|Library]]` 同时锁定已审阅内容。

canonical 与 localized Package 保持目标、selector 与 anchor 同位。

## 规范化边记录

语义引用与锁定引用规范化为有序边记录，包含来源和目标身份、关系、来源位置、可选
selector 与 anchor、生命周期出处、端点期待、候选端点，以及显式的 `resolved`、
`unresolved`、`ambiguous` 或 `incompatible` 结果。除非策略赋予语义，Markdown Link
只用于导航。

所有语法先进入版本化 `docs-hygiene.reference-occurrence.v1` IR。语法/上下文策略分类
每个 occurrence，因此增加 collector 不会静默改变语义。详见
[FEATURE-020](../issues/features/20_reference-occurrence-ir.md)。

## Selector、Pin 与 Snapshot

标题 selector 按 canonical ATX 标题解析。内容锚可以锁定文件、块或完整 tracked 仓库
状态。关键依赖策略选择哪些边需要 Pin、算法、最小 scope 与审计年龄。Portable snapshot
保存签名后的离线文件/块证据，而不把 Git 变成语义权威。详见[关键依赖 Pin](../capabilities/critical-dependency-pins.md)
与[可移植 Commit 快照](../capabilities/portable-snapshots.md)。

## 生命周期与影响

终态身份不能继续作为边目标。声明的当前 successor 保持引用关系，并支持确定性权威迁移。
反向遍历报告已解析语义边的直接与传递消费者；未解析端点不传播。

## 拓扑策略

关系图报告不同邻居的 Fan-In/Fan-Out、循环组、确定性社区、跨社区边与反向影响。显式阈值、
循环策略、社区基线与按节点/方向审计的例外把选定结果转成诊断。详见
[FEATURE-023](../issues/features/23_graph-metrics-and-cycles.md)、
[FEATURE-024](../issues/features/24_graph-communities.md)与
[FEATURE-025](../issues/features/25_fan-budgets-and-exceptions.md)。

## 边界

当前关系图验证已配置文档身份、引用、生命周期、内容锚、Package 成员、本地化同位、影响
与拓扑。外部 Issue 覆盖和通用 Artifact 发现仍属于适配器边界。关系图不证明自然语言
等价性或产品验收。
