# Docs Hygiene

[English](README.md) | 中文

**Docs Hygiene（DH）对项目文档中的认知资产实施工程化治理。**

在 AI Coding 时代，文档是项目意图与决策的 SSOT。Agent 可以快速放大实现能力，
但模糊的需求、不稳定的概念和断裂的约束也会被同样快速地放大。代码质量已经拥有
编译器、类型系统、测试、静态分析和 CI；文档治理却仍大多停留在格式、拼写和死链
检查，无法验证意图是否完整、共享含义是否稳定、决策是否被落实。

Docs Hygiene 把项目文档中的治理要求声明为可持续验证的不变式。它不替团队解释
自然语言，而是在实现放大偏差之前，确定性地暴露结构、身份、引用和追溯关系的断裂。

在[回溯工程](https://indenscale.github.io/synthesis/retrospective-engineering/)中，DH 的定位是
“文档的 Dependabot”：回溯工程从历史证据中重建正向设计，治理技术债和一部分认知债；
DH 则持续检查这些知识资产的身份、依赖、新鲜度与演化关系，防止回溯结果再次静默过期。
DH 不判断业务叙事是否正确，也不替代领域专家。

## 三项治理责任

| 治理责任 | 权威或机制 | 核心问题 |
| --- | --- | --- |
| Semantic Authority / 语义权威 | Library | 它是什么意思，哪一个定义具有权威？ |
| Change Propagation / 变化传播 | Dependency | 上游变化后，谁可能受到影响？ |
| Human Judgment / 人类判断 | Decision | 人们选择了什么、为什么，以及何时需要重新审议？ |

Library Entry 与 Decision Record 是受治理资产；Reference 是一种 Dependency。DH 当前
已经交付 Library 身份、Dependency 图、Pin、影响和生命周期基础；一等 Decision 资产、
通用 Agent Attestation 与 Issue Review 仍是产品方向。完整定位见
[认知资产治理模型](docs/zh/position/02_cognitive_asset_governance_model.md)。

## 内建软件文档 Profile

| Profile 方向 | Docs Hygiene 守护什么 |
| --- | --- |
| 从意图到实现 | 项目决策从 Intent 经 Definition 到 Implementation 的逐层精化与追溯 |
| 从项目主张到共享定义 | Body 内容对同层级 Library 身份的引用，以及 Library 的逐层投影 |
| 从工作语言到分发语言 | canonical 与 localized 表示之间的身份、结构和治理关系同位 |

这三个方向对应精化层级、引用关系和语言表示三个相互独立的治理维度。它们让意图层
的决策能够被逐层验证地落实到实现，让偏离尽早暴露，同时减少理解项目时反复确认
术语、文档身份、权威表示和实现依据的代价。这是认知资产治理模型在软件文档上的
内建 Profile，不是顶层产品本体。详见
[三维软件文档 Profile](docs/zh/position/01_three_dimensional_governance_model.md)。

## 项目与运行边界

项目是治理对象，目录是运行边界，Git 仓库只是物理载体。一个 DH 治理范围可以对应
整个仓库，也可以对应 monorepo 中的一个项目目录。每次检查从显式指定的项目根目录
读取策略并解析受治理资产；DH 当前不会自动发现或编排 monorepo 中的所有项目。

```text
monorepo/
├── platform/
│   ├── docs-hygiene.yml
│   ├── docs/
│   └── src/
└── sdk/
    ├── docs-hygiene.yml
    ├── docs/
    └── src/
```

两个项目可以分别检查：

```bash
docs-hygiene check platform --fail-on-warning
docs-hygiene check sdk --fail-on-warning
```

## 当前能力

Docs Hygiene 当前提供确定性的项目级治理检查：

- README、CHANGELOG、LICENSE 等入口文件完整性；
- 编号文档、允许的文件类型和长度预算；
- 按 Kind 显式启用的 slug Schema、权威来源、规范化冲突索引、本地化身份同位、alias
  与重命名策略；
- 基于路径与文件名推导、可复用模板、确定性 Profile 绑定与 revision 迁移、随项目成熟度增强的文档契约；
- 由类型化 frontmatter 校验和 locale-aware、安全冲突检测的文档脚手架共同消费的 Kind
  注册表，以及原子 Schema/Template 迁移；
- 显式核心 Library claim 权威、已确认重复策略、block pin 受控摘录和建议性相似扫描；
- 跨规范化边关系的关键依赖匹配与 Pin 要求，以及只读计划和显式审计更新；
- 带离线 file/block payload 校验、类型化 provenance、Ed25519 信任、生命周期策略和显式
  本地导入的可移植 commit snapshot；
- 带独立预算、expiry、度数趋势、清理诊断和非 Passed `excepted` 证据的精确
  node/direction 超级节点例外；
- 离线 person/group Principal、已建立身份的 Owner/复核日落/双人确认门禁、覆盖率与
  bus factor 证据，以及带 JSONL 审计的原子 Reset dry-run/apply；
- canonical 与 localized 语言表示的路径、身份和结构同位；
- 从受管内容到 `concept/*.md` 和 Library 身份的语义引用；
- 项目根目录内 Markdown Link、图片目标和语义 Wiki Link 的有效性；
- 带版本的引用 occurrence IR、共享 collector，以及语义边规范化前的显式语法与上下文策略；
- YAML frontmatter、身份 Manifest 和递归 Package 结构；
- 跨资产与 Package 身份的生命周期状态义务、终止目标拒绝和显式 `supersededBy` 权威迁移；
- 带类型化 expectation/resolution、标题 selector、block/file/opt-in repo 锚的规范化
  语义、锁定、派生和投影治理边，以及确定性的传递影响、桥连通社区、跨社区边、
  Fan-In/Fan-Out、循环组和可选拓扑阈值/基线；
- 带版本的多维文档卫生画像，分别报告目标、检测结果、N/A 和不变量证据，同时保留独立
  规则执行状态；
- 对 markdownlint 等外部工具的 Adapter 编排。

Docs Hygiene 不替代 Markdown 格式、外部 URL、拼写或文风工具，也不从自然语言推断
语义等价、翻译新鲜度或业务矛盾。条目级需求覆盖和符号级语义映射仍属于后续方向。

## 渐进式治理

DH 从项目事实渐进激活治理要求，不要求每个项目预先选择一个全局成熟度。本地化文档、
受治理 Manifest、多个精化层级、frontmatter 和语义 Wiki Link 等结构存在性信号会
激活对应规则族；文档和代码规模可以引入非阻断建议，但不会意外让 CI 变红。

每个稳定规则族具有 `inactive`、`advisory`、`warning` 或 `error` 状态。项目策略通过
`auto`、`required` 和 `disabled` 模式保留最终权威。使用以下命令查看当前决策及其
证据：

```bash
docs-hygiene explain-rules
docs-hygiene explain-rules --format json
docs-hygiene profile
docs-hygiene profile --format json
docs-hygiene migrate-templates --check
```

事实模型、规则 ID、覆盖优先级和严重程度契约见
[渐进式规则激活](docs/zh/10_progressive_rule_activation.md)。
画像评估器已经分离成熟度、能力维度与执行状态，完整模型见
[文档卫生治理模型](docs/zh/11_hygiene_governance_model.md)。

## 快速开始

在本仓库构建二进制：

```bash
cargo build --release
```

为项目创建最小文档树和初始策略：

```bash
./target/release/docs-hygiene scaffold /path/to/project
```

运行检查：

```bash
./target/release/docs-hygiene check /path/to/project --fail-on-warning
./target/release/docs-hygiene profile /path/to/project --fail-below-target
```

如果已经安装或已将二进制加入 `PATH`：

```bash
docs-hygiene scaffold .
docs-hygiene scaffold . --kind article --identity ARTICLE-42 --slug cache-policy
docs-hygiene check --fail-on-warning
```

默认情况下，error 会让命令失败，warning 只提供建议；`--fail-on-warning` 会把 warning
也提升为门禁。供其他工具消费时可以输出 JSON：

```bash
docs-hygiene check --format json
```

其他命令包括 `init`、`lang`、`migrate-templates`、`migrate-kinds`、
`scan-library-claims`、`explain` 和 `explain-rules`。`update-pins` 规划或显式执行关键
Pin 刷新；`import-snapshot` 从本地 Git checkout 显式生成 portable payload；
`reset-review` 规划或执行单个审计期限 Reset。运行 `docs-hygiene --help` 查看完整界面。

## 策略

每个治理范围默认从项目根目录读取 `docs-hygiene.yml`。策略声明入口文档、文档区域、
语言表示、概念外键、文档契约、治理 Manifest、豁免和外部 Adapter。项目可以从结构
卫生开始，再随成熟度逐步启用更强的语义与追溯门禁。

本仓库使用自己的 [docs-hygiene.yml](docs-hygiene.yml) 进行 dogfood。它展示了完整的
软件文档 Profile，但不是所有项目都必须复制的固定目录模板。

配置说明见[配置](docs/zh/02_configuration.md)，已交付规则以[规则](docs/zh/03_rules.md)
和[治理关系图](docs/zh/07_governance_graph.md)为准。

## Adapter

Docs Hygiene 负责需要项目上下文的治理规则；已有工具继续负责各自擅长的表层检查。
Adapter 允许一次运行编排这些工具，而不在核心检查器中重复实现它们。

```yaml
adapters:
  markdownlint:
    enabled: true
    command: markdownlint-cli2
    args:
      - README.md
      - README_ZH.md
      - CHANGELOG.md
      - "docs/**/*.md"
```

当前 Adapter 契约见[外部工具 Adapter](docs/zh/04_adapters.md)。

## 文档导航

- [概览](docs/zh/01_overview.md)
- [认知资产治理模型](docs/zh/position/02_cognitive_asset_governance_model.md)
- [三维软件文档 Profile](docs/zh/position/01_three_dimensional_governance_model.md)
- [配置](docs/zh/02_configuration.md)
- [规则](docs/zh/03_rules.md)
- [CI 与 JSON 输出](docs/zh/05_ci.md)
- [文档契约](docs/zh/06_document_contracts.md)
- [治理关系图](docs/zh/07_governance_graph.md)
- [Library Claim 治理](docs/zh/15_library_claims.md)
- [关键依赖 Pin](docs/zh/16_critical_dependency_pins.md)
- [可移植 Commit 快照](docs/zh/17_portable_snapshots.md)
- [可审计超级节点例外](docs/zh/18_supernode_exceptions.md)
- [文档责任与复核日落](docs/zh/19_document_ownership.md)
- [渐进式规则激活](docs/zh/10_progressive_rule_activation.md)
- [文档卫生治理模型](docs/zh/11_hygiene_governance_model.md)
- [文档卫生画像](docs/zh/12_hygiene_profile.md)
- [Roadmap](ROADMAP.md)
- [Changelog](CHANGELOG.md)
