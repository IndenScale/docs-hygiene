---
id: PRD-001
type: product-requirement
status: proposed
ul_registry: docs-hygiene
ul_version: 0.1.0
---

# PRD-001 意图契约治理

## 问题

AI Coding Agent 放大实现的速度已经超过团队验证共享业务语义的速度。仓库文档拥有
格式和链接工具，却缺少对语义身份、需求依赖以及意图到验证证据路径的确定性治理。
一份结构正确的 PRD 仍然可能匿名引入概念分叉，或引用日后发生语义漂移的定义。

## 用户与收益

| 用户 | 需要 | 受治理收益 |
| --- | --- | --- |
| 产品负责人 | 在建立基线前评审业务含义变化 | `BENEFIT-EARLY-DRIFT-DETECTION` |
| 工程师或 Coding Agent | 解析变更必须保持的准确概念和不变量 | `BENEFIT-REPLAYABLE-INTENT` |
| 评审者 | 区分确定性缺陷与需要责任人判断的问题 | `RESULT-REVIEW-REQUIRED` |
| 文档维护者 | 在本地和 CI 使用同一套可重建策略 | `RESULT-POLICY-PASSED` |

## 语义依赖

```yaml
ul:
  registry: docs-hygiene
  version: 0.1.0
  references:
    - DH-PRODUCT
    - DH-INTENT-CONTROL-PLANE
    - DH-SEMANTIC-CONTRACT
    - DH-TRACEABILITY-CONTRACT
    - DH-COGNITIVE-DEBT
    - DH-REVIEW-ITEM
    - BENEFIT-EARLY-DRIFT-DETECTION
    - BENEFIT-REPLAYABLE-INTENT
  local_concepts: []
  change_proposals: []
```

## 需求

### FR-001 带版本的语义 Registry

策略必须从已配置 Registry 加载稳定、类型化的语义 ID，并区分当前能力与产品方向。
已基线化 PRD 必须解析固定 Registry 版本，不能引用浮动的最新定义。

### FR-002 显式 PRD 语义 Manifest

受治理 PRD 必须在机器可读 Manifest 中列出共享 UL 引用、局部概念和语义变更提案。
确定性验证不能依赖从不受限正文中猜测所有可能的领域术语。

### FR-003 感知生命周期的基线门禁

草稿和评审记录可以包含已声明提案。记录进入 `baselined` 时必须具有有效固定引用，
且不能包含缺少责任人、决策状态和到期时间的未决提案。`abandoned` 记录只保留历史
上下文，不成为规范意图。

### FR-004 可重建的语义评审队列

策略必须针对局部概念、语义变化、相似名称、重复局部使用和即将到期的延期生成
评审事项。队列必须是可重建产物；会议状态和决议继续在仓库中接受版本管理。

### FR-005 意图追溯

策略必须支持从 UL 概念经过 PRD 需求和验收标准，到可执行或已记录验证证据的类型化
关系。需求存在于 Markdown 中不能证明它已经交付。

### FR-006 确定性检查与辅助检查

缺失 ID、无效版本、类型错误和不完整 Manifest 可以成为阻断诊断。LLM 辅助的相似性
或矛盾分析可以创建 Warning 或评审事项，但不能替团队决定业务含义，也不能在仓库
未选择确定性策略时阻断基线。

## 非目标

- 生成 PRD、技术设计或实现任务清单。
- 替代 SDD 工作流或 Coding Agent 的规划能力。
- 把自然语言正文中的每个名词都当作受治理概念。
- 声称文档本身可以证明当前实现行为。
- 把会议决议保存在无法从仓库重建的隐藏服务中。

## 验收标准

1. Given PRD 引用了未知 UL ID，When 检查仓库，Then 稳定的阻断诊断定位到对应
   Manifest 条目。
2. Given PRD 使用浮动 UL 版本进入 `baselined`，When 运行基线门禁，Then 门禁失败，
   且不自动改写引用。
3. Given PRD 声明局部概念，When 生成评审输出，Then 事项包含定义、负责人、关系、
   来源 PRD 和到期时间。
4. Given 可选语义分析怀疑存在矛盾，When 结果无法确定性成立，Then 它成为评审事项，
   而不是自动业务决策。
5. Given 需求宣称已经交付但没有链接验证证据，When 检查追溯关系，Then 缺失证明
   明确可见，且 PRD 正文不被视为执行事实。

## 交付状态

本 PRD 状态为 `proposed`。当前引擎已经提供文档契约和基于文件名的概念外键；这里
描述的语义 Registry、Manifest 门禁、评审队列和意图追溯规则尚未实现。
