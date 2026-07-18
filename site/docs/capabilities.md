---
title: 能力与边界
description: Docs Hygiene 当前能够确定性检查的内容，以及明确留给其他工具和人的判断。
---

# 能力与边界

DH 负责那些需要项目上下文、又能够被显式表达和重复验证的治理要求。

## 当前能力

### 结构与契约

- 公共入口文件、文档区域和允许的文件类型；
- Document Kind、frontmatter Schema 和脚手架；
- 可复用模板、revision 兼容与原子迁移；
- canonical/localized 路径、身份与结构同位。

### 身份与权威

- 稳定 slug 和重命名策略；
- Library 身份与唯一权威 claim；
- 重复定义候选和受控摘录；
- 生命周期义务与显式权威迁移。

### 依赖与变化

- 类型化语义引用和目标解析；
- file、block、repo 内容锚；
- 关键依赖 Pin 与审计更新；
- 反向影响、传递影响和失效证据。

### 图与责任

- Fan-In、Fan-Out、循环和社区边界；
- 有预算、期限和审计证据的例外；
- Owner、复核日落、覆盖率和知识冗余；
- 分维度的成熟度目标和执行状态。

## DH 不做什么

DH 不负责：

- 判断一段自然语言是否符合业务真相；
- 推断两段文字是否语义等价；
- 替团队选择产品或架构方案；
- 自动调度 Agent 修改项目；
- 通用 Markdown 格式、拼写、文风和外部 URL 爬取。

这些边界让阻断性 finding 保持可解释和可复现。团队可以在 DH 输出之上使用 AI 或人工评审，但应保留概率分析与确定性检查之间的区别。

## 尚未交付的方向

一等 Decision 资产、通用 Agent Attestation、Issue Review 和更丰富的外部 Issue/Artifact Adapter 仍是产品方向。它们不应仅因为出现在产品模型或 Roadmap 中，就被理解为当前 CLI 行为。
