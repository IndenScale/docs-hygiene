---
title: 文档
description: 从第一次检查开始，逐步理解 Docs Hygiene 的身份、依赖和治理模型。
---

# Docs Hygiene 文档

Docs Hygiene 帮助团队发现项目文档之间已经断掉、冲突或过期的关系。它读取项目中的显式策略，检查稳定身份、语义引用、依赖变化、生命周期和交付追溯，并输出可以在本地与 CI 中复现的结果。

## 从这里开始

| 你想做什么 | 阅读入口 |
| --- | --- |
| 第一次在仓库中运行 DH | [快速开始](getting-started.md) |
| 理解检查结果从哪里来 | [DH 如何工作](how-it-works.md) |
| 配置文档范围和规则 | [项目配置](configuration.md) |
| 把检查加入提交门禁 | [接入 CI](ci.md) |
| 理解身份、引用和 Pin | [身份与依赖](concepts/identity-and-dependency.md) |
| 确认 DH 做什么、不做什么 | [能力与边界](capabilities.md) |

## 最小工作流

```bash
docs-hygiene scaffold .
docs-hygiene check .
docs-hygiene explain-rules .
```

第一次运行时，先处理 `error`，再决定是否用 `--fail-on-warning` 把 warning 也提升为门禁。DH 不会在普通检查中自动修改文档；迁移、Pin 更新和复核重置均提供显式的计划与应用步骤。

::: info 当前状态
DH 仍处于早期版本。已交付行为以仓库中的代码、配置、测试和规则文档为准；Roadmap 中的方向不应被理解为当前能力。
:::

## 文档与工程资产

本站文档面向采用和使用 DH 的团队。仓库中的 UL、PRD、Issue 归档和架构决策是 DH 自身的产品工程资产，用来记录术语权威、需求、验收与实现证据，两者不共享同一阅读路径。
