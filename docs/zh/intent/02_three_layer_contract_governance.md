---
id: PRD-001
version: 1.0.0
type: product-requirement
status: baselined
ul_registry: docs-hygiene
ul_version: 0.2.0
---

# PRD-001 三层契约治理

## 问题

AI Coding Agent 放大实现的速度已经超过团队验证共享业务语义的速度。只治理 PRD
格式或术语引用，不能发现 PRD、形式定义和实现之间的断裂。Docs Hygiene 需要把
Intent、Definition、Implementation 建立为可分类、可投影、可追溯的三层契约，而不把
文档存在本身误认为实现证据。

## 用户与收益

| 用户 | 需要 | 受治理收益 |
| --- | --- | --- |
| 产品负责人 | 在基线前评审意图含义和收益变化 | `BENEFIT-EARLY-DRIFT-DETECTION` |
| 工程师或 Coding Agent | 找到需求对应的定义和实现约束 | `BENEFIT-REPLAYABLE-INTENT` |
| 评审者 | 从需求追溯到可证伪定义和运行证据 | `BENEFIT-LAYERED-TRACEABILITY` |
| 文档维护者 | 在本地和 CI 重建同一治理结果 | `RESULT-POLICY-PASSED` |

## 语义依赖

```yaml
ul:
  registry: docs-hygiene
  version: 0.2.0
  references:
    - DH-THREE-LAYER-MODEL
    - DH-INTENT-LAYER
    - DH-DEFINITION-LAYER
    - DH-IMPLEMENTATION-LAYER
    - DH-REFERENCE-LIBRARY
    - DH-GOVERNED-BODY
    - DH-EVIDENCE-PLANE
    - DH-SEMANTIC-CONTRACT
    - DH-TRACEABILITY-CONTRACT
    - BENEFIT-EARLY-DRIFT-DETECTION
    - BENEFIT-REPLAYABLE-INTENT
    - BENEFIT-LAYERED-TRACEABILITY
  local_concepts: []
  change_proposals: []
```

## 需求

### FR-001 三层资产分类

策略必须按资产职责而非扩展名区分三个层次。Intent Body 是 PRD，Library 是 UL；
Definition Body 是 Spec 与 Test Definition，Library 是 Glossary；Implementation Body
是 Code 与 Configuration，Library 是 SDK；SDK 包含共享类型、Schema、接口和规则。
每项受管资产必须具有稳定身份、层次、角色和生命周期。

### FR-002 同层 Library 引用

每个 Body 必须声明其同层 Library 依赖：`PRD → UL`、`Spec/Test → Glossary`、
`Code/Configuration → SDK`。已基线化资产必须固定依赖版本，不能解析浮动的
最新版本。

### FR-003 Reference 投影

策略必须治理 `UL → Glossary → SDK` 投影。下游 Library 身份必须记录其来源
身份和语义版本；收窄、拆分、合并或不兼容变化必须显式提案，不能静默改变上游含义。

### FR-004 Body 追溯

策略必须治理 `PRD → Spec/Test Definition → Code/Configuration` 追溯。Spec 必须覆盖
来源 PRD 的需求、不变量、收益和验收标准；实现必须声明它兑现的 Spec。缺少中间层
不能被一个直接指向测试结果的链接掩盖。

### FR-005 Evidence 平面

Test Case、模型、Oracle 和 Verifier 属于 Definition Layer；Test Result、验收记录、
运行观察和指标值属于 Evidence 平面。Evidence 必须标明所验证的定义、实现版本和
Intent 收益。定义存在不等于验证通过，技术行为通过也不自动证明用户收益。

### FR-006 生命周期与评审队列

草稿可以包含已声明提案。资产进入 `baselined` 时必须具有固定且有效的引用，并解决
或显式延期未决提案。局部概念、语义变化、断裂追溯和即将到期的延期必须生成可重建
评审事项，决议继续在仓库中接受版本管理。

### FR-007 确定性检查与辅助检查

缺失 ID、无效版本、类型错误、断裂关系和不完整 Manifest 可以阻断。LLM 辅助的
相似性或矛盾分析只能创建 Warning 或评审事项，不能替团队决定业务含义。

## 非目标

- 生成 PRD、Spec、技术设计或任务清单。
- 替代 SDD 工作流、测试框架或 Coding Agent 的规划能力。
- 把目录名或文件扩展名当作层次的唯一判断依据。
- 声称文档、测试定义或指标本身可以证明当前实现行为。
- 要求 Implementation Library 只能采用某一种 SDK 打包形式。

## 验收标准

1. Given PRD 引用了未知或浮动 UL，When 运行基线门禁，Then 产生稳定阻断诊断。
2. Given Spec 没有引用 Glossary 或没有覆盖来源 PRD 需求，When 检查 Definition，
   Then 缺失身份和追溯边分别可见。
3. Given Code/Configuration 没有声明 Spec 或 SDK 依赖，When 检查
   Implementation，Then 不能仅凭文件存在宣称已实现。
4. Given Glossary 或 SDK 符号无法追溯到上游语义版本，When 检查 Reference 投影，
   Then 漂移被阻断或进入显式评审。
5. Given Test Definition 存在但没有对应结果，When 检查 Evidence，Then 状态为缺少
   证明，而不是通过。
6. Given Evidence 证明技术行为但未连接用户收益，When 检查端到端追溯，Then
   `PRD → Spec/Test → Code/Configuration → Evidence → Benefit` 的缺口明确可见。
7. Given 可选语义分析无法确定性裁决矛盾，When 输出结果，Then 它成为评审事项，
   而不是自动业务决策。

## 交付状态

三层资产模型和本 PRD 已建立基线；文档契约治理 `docs/intent` 中的 UL、PRD，以及
`docs/definition` 中的 Glossary、Spec。Implementation 留在仓库根部：公共 Rust
Library 就是 SDK，`implementation-manifest.yml` 声明 CLI/Configuration Body 对
Spec 与 SDK 的关系。通用 Manifest 校验、投影图和端到端追溯诊断仍属于后续实现，
不得描述为已经交付。
