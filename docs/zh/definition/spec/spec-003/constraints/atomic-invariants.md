---
id: SPEC-003-C-007
status: proposed
---

# C-007 原子不变量账本

注册表不变量比兼容规则族和诊断码粒度更细。一个诊断可以为多个不变量提供失败证据，
但每个不变量只属于一个维度和一个最低成熟度。

| 不变量 | 最低等级 | 当前证据 | 交付情况 |
| --- | --- | --- | --- |
| `structure.entry-docs` | 基础 | `DH_REQUIRED_001` | 已交付 |
| `structure.naming-sequence` | 基础 | `DH_NAME_001`、`DH_SEQ_*` | 已交付 |
| `structure.local-links` | 基础 | `DH_LINK_001` | 已交付 |
| `structure.contracts` | 受控 | `DH_CONTRACT_*` | 已交付 |
| `structure.content-policy` | 受控 | `DH_SIZE_001`、`DH_ASCII_001` | 已交付 |
| `structure.reusable-templates` | 受控 | 注册表绑定、`DH_TEMPLATE_*` | 已交付 |
| `structure.template-migration` | 治理 | revision 窗口、`DH_TEMPLATE_003/004`、迁移 CLI | 已交付 |
| `structure.kind-schema` | 治理 | 类型化 frontmatter、Kind 脚手架、原子迁移 | 已交付 |
| `identity.stable-ids` | 基础 | `DH_GOVERNANCE_001`、Package 检查 | 已交付 |
| `identity.duplicates` | 基础 | Governance 与 Library 索引 | 已交付 |
| `identity.library-claims` | 治理 | 显式 Library 权威与已确认 occurrence 策略 | 已交付 |
| `identity.canonical-source` | 受控 | 表示同位 | 已交付 |
| `identity.slug-schema` | 受控 | Kind 级索引、`DH_SLUG_001` | 已交付 |
| `identity.semantic-reference` | 受控 | `DH_CONCEPT_*`、`DH_REFERENCE_001` | 已交付 |
| `identity.lifecycle` | 治理 | 状态义务与终止目标拒绝 | 已交付 |
| `identity.authority-migration` | 治理 | `supersededBy`、后继校验、有序迁移证据 | 已交付 |
| `identity.responsibility` | 治理 | 离线 Principal 目录、精确 Owner 解析、覆盖证据 | 已交付 |
| `identity.review-sunset` | 治理 | 期限 warning/error、显式 Reset 计划/执行与审计 | 已交付 |
| `identity.knowledge-redundancy` | 治理 | 带日期 active-person 确认与逐身份 bus factor | 已交付 |
| `dependency.resolve` | 基础 | Wiki Link 与垂直目标解析 | 已交付 |
| `dependency.typed-edges` | 受控 | 有序的规范化边记录 | 已交付 |
| `dependency.content-anchor` | 受控 | 全目标 SHA-256 | 已交付 |
| `dependency.target-staleness` | 受控 | 哈希不一致诊断 | 已交付 |
| `dependency.selector` | 治理 | 边 selector、`DH_SELECTOR_001`、本地化签名同位 | 已交付 |
| `dependency.scoped-anchor` | 治理 | file/block/commit scope、逐项诊断、opt-in Git 校验 | 已交付 |
| `dependency.critical-pins` | 治理 | 边匹配器、`DH_PIN_*`、审计更新工作流 | 已交付 |
| `dependency.portable-snapshot` | 治理 | 离线 payload、类型化 provenance、签名、`DH_SNAPSHOT_*` | 已交付 |
| `dependency.transitive-impact` | 治理 | 所有已解析语义边上的确定性反向可达集合 | 已交付 |
| `topology.metrics` | 基础 | 规范化节点、边、解析及孤立节点计数 | 已交付 |
| `topology.fan-and-cycles` | 受控 | 不同邻居度数与确定性强连通分量 | 已交付 |
| `topology.thresholds` | 受控 | `DH_TOPOLOGY_001`、`DH_TOPOLOGY_002` | 已交付 |
| `topology.budgets` | 治理 | 全局/例外预算、剩余额度、`DH_TOPOLOGY_001/003` | 已交付 |
| `topology.public-exceptions` | 治理 | 精确 node/direction 审计声明、`excepted` 证据 | 已交付 |
| `topology.trends` | 治理 | 有序度数历史、趋势差、`DH_TOPOLOGY_005` | 已交付 |

`DH_MATURITY_001`、`DH_ACTIVATION_001`、Adapter 失败和 suppression 元数据属于执行或
建议证据，不直接证明成熟度。只有端到端 fixture 改变交付事实时才更新本账本。
