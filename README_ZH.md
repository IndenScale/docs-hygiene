# Docs Hygiene

Docs Hygiene 是面向仓库意图层的 Policy Engine。它把文档治理放进本地检查和
CI，让共享意图不再只依赖模板、评审仪式和个人责任感。

在 AI 辅助研发中，实现能力的增长速度已经超过共享认知和业务验证能力。一条
模糊需求可以在团队发现概念、规则或预期收益存在歧义之前，驱动 Agent 生成数千
行内部一致但业务错误的代码。代码拥有编译器、类型系统、测试和静态分析，承载
意图的文档却很少拥有同等级别的质量体系。Docs Hygiene 要把这类治理真正左移。

## 意图控制平面

代码仍然是系统的执行事实。Reference、PRD、ADR、验收标准和证据索引共同构成
意图控制平面：它们定义实现需要保持的概念、约束、决策和可观察结果。

Docs Hygiene 以 policy-as-code 检查这个控制平面。当前版本提供仓库级结构治理：

它不是 Markdown 语法 linter。Markdown 格式应交给 markdownlint，链接检查应交给 lychee，文案风格应交给 Vale 或 cspell。Docs Hygiene 专注仓库级文档治理：

- README、CHANGELOG、LICENSE 等入口文件完整性
- `docs/` 下的编号文档结构
- 文档长度预算
- 根文档与本地化文档的 i18n 同位关系
- 基于路径与文件名推导、随项目成熟度增强的文档契约
- 从高亮术语到 `concept/*.md` 的概念外键
- Intent、Definition 和 Implementation 资产的版本化治理 Manifest
- 同层 `Body -> Library` 水平引用校验
- 相邻层 Body 派生与 Library 投影校验
- 对 markdownlint 等外部工具的 adapter 编排

产品方向是在这些基座上，从结构卫生继续扩展到语义契约和追溯契约：

- 受治理的通用语言（UL）和带版本的概念引用
- 显式的局部概念和可审议的语义变更提案
- PRD 中实体、动作、不变量、用户收益与验收标准之间的关系
- 从共享意图到可执行证据的条目级覆盖与追溯链

这些契约要在实现放大问题之前暴露认知债。它们不会让 LLM 代替团队决定业务
语义：确定性检查阻断无效引用和不完整契约，存在歧义的语义差异则成为显式评审
事项。

## 三层架构

Docs Hygiene 使用 Body 与 Reference Library 两个正交角色治理三层资产：

| 层次 | Body | Reference Library |
| --- | --- | --- |
| Intent | PRD 目录 Body Package | 递归 UL Tree，每个术语一个 Markdown 叶子 |
| Definition | Spec 目录 Body Package 与 Test Definition | 递归 Glossary Tree，每个术语一个 Markdown 叶子 |
| Implementation | Code 与 Configuration | SDK |

Body 追溯轴是 `PRD → Spec/Test Definition → Code/Configuration`；Library 投影轴是
`UL → Glossary → SDK`。Test Definition 属于 Definition，Test Result 和运行
观察属于独立 Evidence 平面。canonical UL 位于 `docs/intent/ul/`，Glossary 位于
`docs/definition/glossary/`；中文表示分别位于 `docs/zh/intent/ul/` 和
`docs/zh/definition/glossary/`。每个领域具有 Manifest，每个稳定术语使用一个同名
Markdown 叶子。PRD 与 Spec 分别位于 `docs/intent/prd/` 和 `docs/definition/spec/`
的递归 Body Package 中，并保持中文同构树、身份和版本一致。
Implementation 留在仓库根部：
`src/lib.rs` 就是 SDK，Code/Configuration 关系由 `implementation-manifest.yml` 声明。
核心检查器按稳定 ID 与版本解析这些资产，校验同层引用，并校验相邻层
`formalizes`、`realizes` 和 `projects` 边。

## 产品边界

Docs Hygiene 不是 SDD 工作流或执行计划工具。它不生成 PRD、技术设计和任务拆解，
也不规定 Coding Agent 应当如何实现变更。SDD 和 Coding Agent 可以消费受治理的
意图；Docs Hygiene 负责验证上游语言、文档和证据关系是否持续一致。

## 快速开始

```bash
cargo run -- check --fail-on-warning
```

创建初始策略文件：

```bash
cargo run -- init
```

创建初始文档树：

```bash
cargo run -- scaffold
```

管理语言策略：

```bash
cargo run -- lang list
cargo run -- lang add ja --min-cjk-ratio 0.10
cargo run -- lang set-threshold ja --max-cjk-ratio 0.90
cargo run -- lang remove ja
```

## 策略

本仓库使用 `docs-hygiene.yml` dogfood Docs Hygiene。已经实现的规则面以 `docs/`
中的说明为准；上述产品方向只有在进入这些说明后，才代表已经可用。

## Adapter

Docs Hygiene 可以调用外部工具，而不是重写它们的规则。第一版 adapter 是
markdownlint：

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
