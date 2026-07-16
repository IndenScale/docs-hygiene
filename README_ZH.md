# Docs Hygiene

Docs Hygiene 是一个仓库文档检查工具。它在本地和 CI 中检查文档是否完整、
组织是否清楚，以及不同层次的文档能否相互追溯。

在 AI 辅助研发中，实现能力的增长速度已经超过共享认知和业务验证能力。一条
模糊需求可以在团队发现概念、规则或预期收益存在歧义之前，驱动 Agent 生成数千
行内部一致但业务错误的代码。代码拥有编译器、类型系统、测试和静态分析，承载
意图的文档却很少拥有同等级别的质量体系。Docs Hygiene 希望让文档也能被持续检查。

## 文档

Docs Hygiene 把仓库中的 README、PRD、规格、ADR 和其他说明都视为需要维护的
文档资产。当前版本提供仓库级结构检查：

它不是 Markdown 语法 linter。Markdown 格式应交给 markdownlint，链接检查应交给 lychee，文案风格应交给 Vale 或 cspell。Docs Hygiene 专注仓库级文档治理：

- README、CHANGELOG、LICENSE 等入口文件完整性
- `docs/` 下的编号文档结构
- 文档长度预算
- 根文档与本地化文档的 i18n 同位关系
- 基于路径与文件名推导、随项目成熟度增强的文档契约
- 从高亮术语到 `concept/*.md` 的概念外键
- 意图、规格和实现文档的版本化 Manifest
- 主体文档到同层引用文档的引用校验
- 意图、规格和实现之间的相邻层追溯校验
- 对 markdownlint 等外部工具的 adapter 编排

产品方向是在这些基座上，从结构卫生继续扩展到语义契约和追溯契约：

- 受治理的通用语言（UL）和带版本的概念引用
- 显式的局部概念和可审议的语义变更提案
- PRD 中实体、动作、不变量、用户收益与验收标准之间的关系
- 从意图到规格和实现的条目级覆盖与追溯链

这些契约要在实现放大问题之前暴露认知债。它们不会让 LLM 代替团队决定业务
语义：确定性检查阻断无效引用和不完整契约，存在歧义的语义差异则成为显式评审
事项。

## 主体与引用

每层文档分为两个角色：

- **主体（Body）**：表达当前项目具体要做什么；
- **引用（Reference）**：保存可被多个主体复用的术语、类型或规则，避免重复定义，
  并让上下文保持连贯。

## 三层文档

文档按意图、规格和实现分为三层：

| 层次 | 主体 | 引用 |
| --- | --- | --- |
| 意图 | PRD：说明为什么做、为谁做、期望什么结果 | 通用语言（UL）：统一产品和业务术语 |
| 规格 | Spec 与测试定义：说明怎样才算正确 | Glossary：把产品术语收窄为精确定义 |
| 实现 | 代码与配置：实现规格 | SDK：提供可复用的类型、接口和规则 |

主体沿 `PRD → Spec/测试定义 → 代码/配置` 逐层落实；引用沿
`UL → Glossary → SDK` 逐层细化。UL 位于 `docs/intent/ul/`，Glossary 位于
`docs/definition/glossary/`；中文表示分别位于 `docs/zh/intent/ul/` 和
`docs/zh/definition/glossary/`。每个领域具有 Manifest，每个稳定术语使用一个同名
Markdown 文件。PRD 与 Spec 分别位于 `docs/intent/prd/` 和 `docs/definition/spec/`
中，并保持中文目录、身份和版本一致。
Implementation 留在仓库根部：
`src/lib.rs` 就是 SDK，Code/Configuration 关系由 `implementation-manifest.yml` 声明。
核心检查器按稳定 ID 与版本解析这些资产，校验同层引用，并校验相邻层
`formalizes`、`realizes` 和 `projects` 边。

## 产品边界

Docs Hygiene 不是 SDD 工作流或执行计划工具。它不生成 PRD、技术设计和任务拆解，
也不规定 Coding Agent 应当如何实现变更。SDD 和 Coding Agent 可以消费受治理的
意图；Docs Hygiene 负责验证这些文档及其引用关系是否持续一致。

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
