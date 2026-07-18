# 认知资产治理模型

状态：adopted

范围：Docs Hygiene 的叙事与产品边界

## 立场主张

Docs Hygiene 治理需要在演化中保持含义、依赖、证据和人工判断可检查的认知资产。
语义权威、变更传播与人工判断是产品责任，而不是对称的目录层。

## 资产角色

公共基底区分受治理 Asset、类型化 Dependency 与 Attestation。UL 提供语义权威，PRD
记录长期产品意图，Issue 记录有范围的变更与判断，Artifact 提供实现或证据。角色描述
权威边界，而不是存储位置。

## 引用关系

Reference 是对权威的语义 Dependency；Pin 锁定已审阅的上游状态。Issue 关系把工作连接
到 PRD 需求与 Artifact 证据。一次变更可以产生潜在影响、锁定消费者的确定性失效，以及
由策略选择的复核集合。

## 语言表示

canonical 与 localized 形式表示同一个语义资产，共享稳定身份与治理关系；本地化不会
创建第二权威或独立决策。

## 治理关系图

```text
权威资产 ── Reference/Dependency ──▶ 消费资产
Issue ── addresses ──▶ Requirement
Issue ── evidencedBy ──▶ Artifact
Attestation ── evaluates/confirms ──▶ Asset 或 Dependency 状态
```

稳定身份、生命周期、所有权、证据与表示元数据是共享治理属性。锁定、结构影响、拓扑、
所有权、审计与生命周期控制都运行在这一基底上。

## 边界

Docs Hygiene 负责确定性发现、依赖分析、策略求值、诊断和可审计更新协议。它不调度
Agent，不从正文推断语义真相，不选择架构，也不规定仓库拓扑。内建软件文档 Profile
见[开放工程资产模型](01_open_engineering_asset_model.md)。
