# 通用语言

Registry ID: `docs-hygiene`

Registry version: `0.2.0`

本注册表定义 Docs Hygiene 产品参考、需求、诊断、测试和 Adapter 共同使用的语言。
ID 是稳定的语义身份；展示名称可以翻译，但不能因此改变身份。

## 产品概念

| ID | 标准术语 | 定义 | 能力状态 |
| --- | --- | --- | --- |
| `DH-PRODUCT` | Docs Hygiene | 在本地和 CI 中检查仓库意图层的 Policy Engine。 | 当前 |
| `DH-INTENT-CONTROL-PLANE` | 意图控制平面 | 约束实现的权威 Reference、需求、决策、验收标准和证据关系。 | 产品方向 |
| `DH-THREE-LAYER-MODEL` | 三层模型 | 以 Intent、Definition、Implementation 组织 Body、Library、追溯和 Evidence 的产品架构。 | 已采用 |
| `DH-INTENT-LAYER` | Intent Layer | Body 为 PRD、Reference Library 为 UL 的意图层。 | 已采用 |
| `DH-DEFINITION-LAYER` | Definition Layer | Body 为 Spec/Test Definition、Reference Library 为 Glossary 的定义层。 | 已采用 |
| `DH-IMPLEMENTATION-LAYER` | Implementation Layer | Body 为 Code/Configuration、Reference Library 为封装共享实现原语的 SDK。 | 已采用 |
| `DH-REFERENCE-LIBRARY` | Reference Library | 被同层多个 Body 复用的语言、定义身份或实现原语。 | 已采用 |
| `DH-GOVERNED-BODY` | 受管 Body | 在某一层中表达具体意图、定义或实现的受管主张。 | 已采用 |
| `DH-EVIDENCE-PLANE` | Evidence 平面 | 记录实现是否满足定义及结果是否兑现意图收益的观察，而不是第四个资产层。 | 已采用 |
| `DH-EXECUTION-TRUTH` | 执行事实 | 由当前代码、配置、测试和运行证据证明的系统行为。 | 当前 |
| `DH-MANAGED-DOCUMENT` | 受管文档 | 由已配置 docs base 或文档契约负责的仓库文档。 | 当前 |
| `DH-DOCUMENT-CONTRACT` | 文档契约 | 根据路径推导、随成熟度增强的必要字段与语义章节契约。 | 当前 |
| `DH-CONCEPT-REFERENCE` | 概念引用 | 受管文档指向受治理概念身份的声明关系。 | 当前，基于文件名 |
| `DH-SEMANTIC-CONTRACT` | 语义契约 | 治理类型化概念、固定引用、局部概念和语义变更提案的契约。 | 产品方向 |
| `DH-TRACEABILITY-CONTRACT` | 追溯契约 | 要求意图关系能够到达验收标准和验证证据的契约。 | 产品方向 |
| `DH-COGNITIVE-DEBT` | 认知债 | 共享语言、需求、实现行为、指标和证据之间尚未解决的分歧或歧义。 | 产品方向 |
| `DH-REVIEW-ITEM` | 语义评审事项 | 由机器发现、需要责任人决策而不能被自动裁决的语义问题。 | 产品方向 |

## 产品动作

| ID | 标准动作 | 成功结果 | 能力状态 |
| --- | --- | --- | --- |
| `CMD-CHECK-REPOSITORY-DOCS` | 检查仓库文档 | 针对配置的策略面返回确定性诊断。 | 当前 |
| `CMD-INFER-DOCUMENT-CONTRACT` | 推导文档契约 | 根据仓库路径和文件名选择第一个匹配的 Profile。 | 当前 |
| `CMD-VALIDATE-CONCEPT-REFERENCE` | 验证概念引用 | 确认高亮概念具有对应的概念定义。 | 当前 |
| `CMD-ORCHESTRATE-ADAPTER` | 编排 Adapter | 运行已配置外部检查器，并将失败归一为 Docs Hygiene 诊断。 | 当前 |
| `CMD-VALIDATE-SEMANTIC-MANIFEST` | 验证语义 Manifest | 检查类型化且带版本的 UL 引用、局部概念和变更提案。 | 产品方向 |
| `CMD-GENERATE-REVIEW-QUEUE` | 生成语义评审队列 | 产生可重建的评审事项，但不替团队决定业务含义。 | 产品方向 |
| `CMD-VALIDATE-INTENT-TRACE` | 验证意图追溯 | 检查受治理意图能否到达验收标准和验证证据。 | 产品方向 |

## 不变量

1. Docs Hygiene 必须区分当前能力和产品方向。
2. 代码和测试建立执行事实；没有可执行证据时，意图文档不能宣称实现已经存在。
3. 共享概念只有一个稳定身份。局部细化或竞争性含义必须显式声明，不能匿名进入正文。
4. 确定性缺陷可以阻断 CI；存在歧义的语义判断应成为评审事项，不能由 LLM 代替团队裁决。
5. 已基线化意图使用固定语义版本；Registry 变化不能改写历史意图。
6. Docs Hygiene 治理意图契约，不生成技术计划，也不规定实现任务顺序。

## 结果与用户收益

| ID | 标准结果 | 可观察证据 |
| --- | --- | --- |
| `RESULT-POLICY-PASSED` | 策略通过 | CLI 成功退出且没有阻断诊断。 |
| `RESULT-POLICY-FAILED` | 策略失败 | CLI 返回稳定诊断码、路径和位置。 |
| `RESULT-REVIEW-REQUIRED` | 需要语义评审 | 可重建的评审事项包含来源、概念关系和原因。 |
| `BENEFIT-EARLY-DRIFT-DETECTION` | 提前发现意图漂移 | 不完整或断裂的意图关系在被实现放大前可见。 |
| `BENEFIT-REPLAYABLE-INTENT` | 可重放历史意图 | 评审者能够解析基线使用的准确概念和验收含义。 |
| `BENEFIT-LAYERED-TRACEABILITY` | 分层可追溯 | 评审者能够沿 Body 和 Library 两条轴定位断裂关系及缺失证据。 |

## 变更规则

1. 语义变化必须提升 Registry 版本。
2. 含义稳定时，展示名称变化不创建新 ID。
3. 拆分、合并、收窄或扩展含义必须记录关系并评审影响面。
4. 产品需求必须固定本 Registry 版本，并列出所消费的受治理概念。
5. 三层关系变化必须同步评审 PRD、Glossary、Spec 和 SDK 投影影响。
